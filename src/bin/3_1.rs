use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Claim {
    id: i32,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32
}

impl FromStr for Claim {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<i32> = s.replace('#', "")
            .split(|c| c == '@' || c == ',' || c == ':' || c == 'x')
            .map(|s1| s1.trim().parse::<i32>().unwrap())
            .collect();

        Ok(Claim {
            id: fields[0],
            left: fields[1],
            top: fields[2],
            right: fields[1] + fields[3],
            bottom: fields[2] + fields[4]
        })
    }
}

fn map_claim(claim_map: &mut HashMap<(i32, i32), i32>, claim: &Claim) {
    for x in claim.left .. claim.right {
        for y in claim.top .. claim.bottom {
            let counter = claim_map.entry((x, y)).or_insert(0);
            *counter += 1;
        }
    }
}

fn overlap(claim1: &Claim, claim2: &Claim) -> bool {
    if claim1.left >= claim2.right ||
        claim1.right <= claim2.left ||
        claim1.top >= claim2.bottom ||
        claim1.bottom <= claim2.top {
        return false
    }
    true
}

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/3_1")?;
    let reader = BufReader::new(datafile);
    let claims: Vec<Claim> = reader.lines().filter_map(|line| line.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut claim_map = HashMap::new();
    for claim in &claims {
        map_claim(&mut claim_map, &claim);
    }

    for claim1 in &claims {
        let mut does_overlap = false;
        for claim2 in &claims {
            if claim1 == claim2 {
                continue;
            }
            if overlap(&claim2, &claim1) {
                does_overlap = true;
                break;
            }
        }

        if !does_overlap {
            println!("{:?}", claim1);
        }
    }

    let count_of_overlaps = claim_map.values().filter(|&c| c >= &2).count();
    println!("Count of overlaps: {}", count_of_overlaps);

    Ok(())
}
