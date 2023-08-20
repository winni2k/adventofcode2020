use combinations::Combinations;
//use ndarray::{s, Array2, ArrayView, ArrayBase};
use ndarray::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let s = read_to_string("test.txt")?;
    // let s = read_to_string("adventofcode.com_2020_day_20_input.txt")?;

    let tile_objects = s
        .trim_end_matches('\n')
        .split("\n\n")
        .map(Tile::from_string);
    let mut tiles = HashMap::new();
    for tile in tile_objects {
        tiles.insert(tile.id, tile);
    }

    let combs: Vec<_> = Combinations::new(tiles.keys().copied().collect(), 2).collect();
    for comb in combs {
        let tid1 = &comb[0];
        let tid2 = &comb[1];
        let tile1 = &tiles[tid1];
        let tile2 = &tiles[tid2];
        if tile1.is_neighbor(tile2) {
            tiles.entry(*tid1).and_modify(|tile| {
                tile.neighbors.insert(*tid2);
            });
            tiles.entry(*tid2).and_modify(|tile| {
                tile.neighbors.insert(*tid1);
            });
        }
    }
    let mut corner_ids: Vec<_> = vec![];
    for (key, val) in tiles.iter() {
        if val.neighbors.len() == 2 {
            println!("Corner: {key}");
            corner_ids.push(key);
        }
    }
    println!(
        "Product of corner ids: {:?}\n",
        corner_ids
            .iter()
            .map(|v| **v as u128)
            .reduce(|acc, e| acc * e)
            .unwrap()
    );
    Ok(())
}

const NROW: usize = 10;

#[derive(Debug)]
struct Tile {
    id: u32,
    data: ndarray::Array2<u8>,
    neighbors: HashSet<u32>,
    edges_smaller: Vec<u16>,
}

fn edges_from_arr2(data: &Array2<u8>) -> Vec<u16> {
    let slices = vec![
        data.slice(s![0, ..]),
        data.slice(s![.., -1]),
        data.slice(s![-1, ..]),
        data.slice(s![.., 0]),
    ];
    let mut edges_smaller = vec![0u16, 0, 0, 0];
    for (edge_idx, slice) in slices.iter().enumerate() {
        let mut dat_int: u16 = 0;
        let mut dat_int2: u16 = 0;
        for (i, val) in slice.iter().enumerate() {
            dat_int |= (*val as u16) << i;
            dat_int2 |= (*val as u16) << (NROW - i - 1);
        }
        let dat_val_smaller = match dat_int < dat_int2 {
            true => dat_int,
            false => dat_int2,
        };
        edges_smaller[edge_idx] = dat_val_smaller;
    }
    edges_smaller
}

impl Tile {
    fn from_string(tile_text: &str) -> Tile {
        let num_re = Regex::new(r"^Tile (\d+)").unwrap();
        let mut lines = tile_text.split('\n');
        let line = lines.next().unwrap();
        let (_, [id_str]) = num_re.captures_iter(line).next().unwrap().extract();
        let id = id_str.parse::<u32>().unwrap();

        let mut dat: Vec<u8> = Vec::new();
        for line in lines {
            for char in line.chars() {
                dat.push((char == '#') as u8);
            }
        }
        let data = Array2::from_shape_vec((NROW, NROW), dat).unwrap();

        Self::from_arr2(id, data)
    }

    fn from_arr2(id: u32, data: Array2<u8>) -> Tile {
        let edges_smaller = edges_from_arr2(&data);
        Tile {
            id,
            data,
            neighbors: HashSet::new(),
            edges_smaller,
        }
    }
    fn edges(&self, idx: usize) -> ArrayView1<u8> {
        let slices = vec![s![0, ..], s![.., -1], s![-1, ..], s![.., 0]];
        self.data.slice(slices[idx])
    }
    fn is_neighbor(&self, other: &Tile) -> bool {
        match self.find_neighbor_edges(other) {
            Some((i, j)) => true,
            None => false,
        }
    }
    fn find_neighbor_edges(&self, other: &Tile) -> Option<(usize, usize)> {
        for (i, ev1) in self.edges_smaller.iter().enumerate() {
            for (j, ev2) in other.edges_smaller.iter().enumerate() {
                if ev1 == ev2 {
                    return Some((i, j));
                }
            }
        }
        None
    }
    fn flip_and_rotate_to_match(&mut self, other: &Tile) {
        let (eidx_self, eidx_other) = self.find_neighbor_edges(other).unwrap();
        if self.edges(eidx_self) != other.edges(eidx_other) {
            self.flip()
        }
        //let (eidx_self, eidx_other) = self.find_neighbor_edges(other).unwrap();
    }
    fn flip(&mut self) {
        self.edges_smaller.swap(1, 3);
        let data_flip = self.data.slice_mut(s![.., ..;-1]).to_owned();
        self.data = data_flip;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use ndarray::{arr2, Array};

    #[test]
    fn test_flip() {
        // given
        let mut tile = Tile::from_arr2(0, Array::eye(10));
        // when
        tile.flip();
        // then
        assert_eq!(
            tile.data,
            arr2(&[
                [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ])
        );
    }
}
