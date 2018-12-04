use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/1_1")?;
    let reader = BufReader::new(datafile);
    let deltas: Vec<_> = reader.lines().filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();
    let freqs = deltas.iter().cycle().scan(0i32, |state, &x| {
        *state += x;
        Some(*state)
    });
    let mut seen_freqs = HashSet::new();
    let mut first_dup = 0;
    for freq in freqs {
        if seen_freqs.contains(&freq) {
            first_dup = freq;
            break;
        } else {
            seen_freqs.insert(freq);
        }
    };
    let freq: i32 = deltas.iter().sum();
    println!("freq: {:?}", freq);
    println!("first_dup: {:?}", first_dup);
    Ok(())
}
