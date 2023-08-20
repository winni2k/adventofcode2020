use std::fs::read_to_string;
use ndarray::{Array2, ArrayView1, s};
use regex::Regex;
use std::collections::{HashSet, HashMap};
use combinations::Combinations;



fn main() -> std::io::Result<()> {
//    let s = read_to_string("test.txt")?;
    let s = read_to_string("adventofcode.com_2020_day_20_input.txt")?;
      
    let tile_objects = s.trim_end_matches("\n").split("\n\n")
        .map(|t| {
            Tile::new(t)
        });
    let mut tiles = HashMap::new();
    for tile in tile_objects{
        tiles.insert(tile.id, tile);
    }
    
    let combs: Vec<_>= Combinations::new(tiles.keys().map(|k| *k).collect(), 2).collect();
    for comb in combs {
        let ref tid1 = comb[0];
        let ref tid2 = comb[1];
        let ref tile1 = tiles[tid1];
        let ref tile2 = tiles[tid2];
        if tile1.is_neighbor(tile2) {
            tiles
                .entry(*tid1)
                .and_modify(|tile| { tile.neighbors.insert(*tid2); });
            tiles
                .entry(*tid2)
                .and_modify(|tile| { tile.neighbors.insert(*tid1); });
        }
    }
    print!("{:?}\n", tiles);
    let mut corner_ids: Vec<_> = vec![];
    for (key, val) in tiles.iter() {
        if val.neighbors.len() == 2 {
            print!("Corner: {key}\n");
            corner_ids.push(key);
        }
    }
    print!(
        "Product of corner ids: {:?}\n", 
        corner_ids
            .iter()
            .map(|v| **v as u128)
            .reduce(|acc, e| acc * e)
            .unwrap()
    );
    Ok(())
}


#[derive(Debug)]
struct Tile {
    id: u32,
    //data: ndarray::Array2<u8>,
    neighbors: HashSet<u32>,
    edges: Vec<u16>,
}

impl Tile {
    fn new(tile_text: &str) -> Tile{
        let num_re = Regex::new(r"^Tile (\d+)").unwrap();
        let mut lines = tile_text.split("\n");
        let line = lines.next().unwrap();
        let (_, [id_str]) = num_re.captures_iter(line)
            .next().unwrap().extract();
        let id = id_str.parse::<u32>().unwrap();

        let mut dat: Vec<u8> = Vec::new();
        for line in lines {
            for char in line.chars(){
                dat.push((char == '#') as u8);
            }
        }
        const NROW: usize = 10;
        let data = Array2::from_shape_vec((NROW,NROW), dat).unwrap();
        print!("{:?}\n", data);
        let slices = vec![
            data.slice(s![0,..]),
            data.slice(s![-1,..]),
            data.slice(s![..,0]),
            data.slice(s![..,9]),
        ];
        let mut edges = vec![0u16, 0,0,0];
        for (edge_idx, slice) in slices.iter().enumerate() {
            let mut dat_int: u16 = 0;
            let mut dat_int2: u16 = 0;
            for (i, val) in slice.iter().enumerate() {
                dat_int = dat_int | ((*val as u16) << i);
                dat_int2 = dat_int2 | ((*val as u16) << (NROW - i -1));
            }
            let dat_val = match dat_int < dat_int2 {
                true => dat_int,
                false => dat_int2,
            };
            edges[edge_idx] = dat_val;
        }

        Tile {
            id: id, 
            //data: data,
            neighbors: HashSet::new(),
            edges: edges,
        }
    }
//    fn edge_vecs(&self) -> Vec<ArrayView1<u8>>{
//        vec![
//            self.data.slice(s![0,..]),
//            self.data.slice(s![-1,..]),
//            self.data.slice(s![..,0]),
//            self.data.slice(s![..,9]),
//            self.data.slice(s![0,..;-1]),
//            self.data.slice(s![9,..;-1]),
//            self.data.slice(s![..;-1,0]),
//            self.data.slice(s![..;-1,-1]),
//        ]
//    }
    fn is_neighbor(&self, other: &Tile) -> bool {
        for ev1 in self.edges.iter() {
            for ev2 in other.edges.iter() {
                if ev1 == ev2 {
                    return true;
                }
            }
        }
        false
    }
}
