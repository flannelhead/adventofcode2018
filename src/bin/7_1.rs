use std::fs::File;
use std::io::prelude::*;
use std::collections::*;

fn main() -> std::io::Result<()> {
    let mut datafile = File::open("data/7")?;
    let mut data = String::new();
    datafile.read_to_string(&mut data)?;

    let deps_pairs: Vec<(char, char)> = data.lines()
        .map(|line| {
            let fields: Vec<_> = line.split(' ').collect();
            (fields[1].chars().next().unwrap(), fields[7].chars().next().unwrap())
        })
        .collect();

    let mut deps: HashMap<char, HashSet<char>> = HashMap::new();
    for (dep, target) in deps_pairs {
        deps.entry(target).or_insert(HashSet::new())
            .insert(dep);
        deps.entry(dep).or_insert(HashSet::new());
    }

    let mut steps_ordered = String::new();
    loop {
        let chosen_step = deps.iter()
            .filter(|(_, dep_set)| dep_set.is_empty())
            .min_by_key(|(target, _)| *target);

        match chosen_step {
            Some(target) => {
                let step = *target.0;
                steps_ordered.push(step);
                deps.iter_mut().for_each(|(_, dep_set)| { dep_set.remove(&step); });
                deps.remove(&step);
            },
            None => break
        }
    }

    println!("{}", steps_ordered);

    Ok(())
}
