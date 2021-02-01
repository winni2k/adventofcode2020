use regex::Regex;
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;
    let mut n_valid_pw: u32 = 0;
    for cap in re.captures_iter(&contents) {
        let lower = cap[1].parse::<u32>()? as usize;
        let upper = cap[2].parse::<u32>()? as usize;

        println!("{}-{} {}: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
        assert!(lower <= upper);

        let n_matches: usize = cap[4].matches(&cap[3]).count();
        println!("{}", n_matches);

        if lower <= n_matches && n_matches <= upper {
            n_valid_pw += 1;
        }
    }
    println!("n valid passwords: {}", n_valid_pw);
    Ok(())
}
