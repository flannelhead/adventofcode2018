use std::fs::File;
use std::io::{BufReader, BufRead};

fn levenshtein(str1: &String, str2: &String) -> i32 {
    let pairs = str1.chars().zip(str2.chars());
    let mut distance = 0;
    for (chr1, chr2) in pairs {
        if chr1 != chr2 {
            distance += 1;
        }
    }
    distance
}

fn common_chars(str1: &String, str2: &String) -> String {
    let mut common: String = String::new();
    let pairs = str1.chars().zip(str2.chars());
    for (chr1, chr2) in pairs {
        if chr1 == chr2 {
            common.push(chr1);
        }
    }
    common
}

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/2_1")?;
    let reader = BufReader::new(datafile);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut found_str1 = None;
    let mut found_str2 = None;
    for (i, str1) in lines.iter().enumerate() {
        for str2 in &lines[i + 1 ..] {
            if levenshtein(&str1, &str2) == 1 {
                found_str1 = Some(str1);
                found_str2 = Some(str2);
            }
        }
    }

    println!("{:?}", found_str1);
    println!("{:?}", found_str2);
    println!("common: {}", common_chars(&found_str1.unwrap(), &found_str2.unwrap()));

    Ok(())
}

