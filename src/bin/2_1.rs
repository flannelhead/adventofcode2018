use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/2_1")?;
    let reader = BufReader::new(datafile);
    let lines = reader.lines().filter_map(|line| line.ok());

    let mut count_2 = 0;
    let mut count_3 = 0;
    for line in lines {
        let mut histogram = HashMap::new();
        for chr in line.chars() {
            let counter = histogram.entry(chr).or_insert(0);
            *counter += 1;
        }

        if histogram.values().any(|&x| x == 2) {
            count_2 += 1;
        }
        if histogram.values().any(|&x| x == 3) {
            count_3 += 1;
        }
    }

    println!("Count of 2: {}", count_2);
    println!("Count of 3: {}", count_3);
    println!("Checksum: {}", count_2 * count_3);

    Ok(())
}
