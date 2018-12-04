use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/1_1")?;
    let reader = BufReader::new(datafile);
    let deltas = reader.lines().filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok());
    let amount: i32 = deltas.sum();
    println!("sum: {:?}", amount);
    Ok(())
}
