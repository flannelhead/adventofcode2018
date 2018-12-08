use std::fs::File;
use std::io::prelude::*;
use std::collections::*;

fn part1(mut steps: HashMap<char, HashSet<char>>) -> () {
    let mut steps_ordered = String::new();
    loop {
        match take_task(&mut steps) {
            Some(step) => {
                steps_ordered.push(step);
                task_done(step, &mut steps);
            },
            None => break
        }
    }

    println!("{}", steps_ordered);
}

#[derive(Clone, Debug)]
struct Task {
    name: char,
    finish: i32
}

fn task_duration(task_name: char) -> i32 {
    return task_name as i32 - 'A' as i32 + 61;
}

fn take_task(steps: &mut HashMap<char, HashSet<char>>) -> Option<char> {
    steps.iter()
        .filter(|(_, dep_set)| dep_set.is_empty())
        .min_by_key(|(target, _)| *target)
        .map(|target| *target.0)
        .and_then(|name| {
            steps.remove(&name);
            Some(name)
        })
}

fn task_done(name: char, steps: &mut HashMap<char, HashSet<char>>) {
    steps.iter_mut().for_each(|(_, dep_set)| { dep_set.remove(&name); });
}

fn part2(mut steps: HashMap<char, HashSet<char>>) -> () {
    let mut workers: Vec<Option<Task>> = vec![None; 5];
    let mut time: i32 = 0;
    while !steps.is_empty() || workers.iter().any(|worker| worker.is_some()) {
        println!("{:?}", workers);
        // Finish the next task
        let next_work = workers.iter_mut()
            .filter(|worker| worker.is_some())
            .min_by_key(|worker| (*worker).clone().unwrap().finish);

        next_work.and_then(|worker: &mut Option<Task>| {
            match worker {
                Some(task) => {
                    time = task.finish;
                    task_done(task.name, &mut steps);
                    *worker = None;
                },
                None => ()
            }
            Some(())
        });

        // Start new tasks for free workers
        workers.iter_mut().filter(|worker| worker.is_none())
            .for_each(|worker| {
                *worker = take_task(&mut steps)
                    .map(|name| Task {
                        name: name,
                        finish: time + task_duration(name)
                    });
            });
    }

    println!("finish time: {}", time);
}

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

    let mut steps: HashMap<char, HashSet<char>> = HashMap::new();
    for (dep, target) in deps_pairs {
        steps.entry(target).or_insert(HashSet::new())
            .insert(dep);
        steps.entry(dep).or_insert(HashSet::new());
    }

    part1(steps.clone());
    part2(steps.clone());

    Ok(())
}
