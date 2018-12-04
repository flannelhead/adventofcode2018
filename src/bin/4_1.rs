use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::collections::HashMap;
use std::iter::FromIterator;

extern crate chrono;
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
enum Event {
    FallsAsleep,
    WakesUp,
    BeginsShift(i32)
}

#[derive(Debug, PartialEq)]
struct LogLine {
    dt: DateTime<Local>,
    event: Event
}

type AsleepMinutes = [u32; 60];

impl FromStr for LogLine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err("Could not parse time stamp");
        }
        let fields: Vec<&str> = s.split(']').collect();
        if fields.len() != 2 {
            return Err("Could not parse time stamp");
        }
        let datetime_str = fields[0].trim_matches('[');
        let datetime = Local.datetime_from_str(datetime_str, "%Y-%m-%d %H:%M")
            .unwrap();
        let event = fields[1].trim().parse::<Event>()?;
        Ok(LogLine {
            dt: datetime,
            event: event
        })
    }
}

impl FromStr for Event {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "falls asleep" {
            return Ok(Event::FallsAsleep);
        } else if s == "wakes up" {
            return Ok(Event::WakesUp);
        } else if s.starts_with("Guard") && s.ends_with("begins shift") {
            let fields: Vec<&str> = s.split(' ').collect();
            if fields.len() != 4 || !fields[1].starts_with('#') {
                return Err("Could not parse BeginsShift event");
            }

            let guard_id_result = fields[1].trim_matches('#').parse::<i32>();
            return match guard_id_result {
                Ok(guard_id) => Ok(Event::BeginsShift(guard_id)),
                Err(_msg) => Err("Could not parse BeginsShift event")
            }
        }

        Err("Event not recognized")
    }
}

fn main() -> std::io::Result<()> {
    let datafile = File::open("data/4_1")?;
    let reader = BufReader::new(datafile);
    let mut loglines: Vec<LogLine> = reader.lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    loglines.sort_by(|a, b| a.dt.cmp(&b.dt));

    let mut guard_minutes: HashMap<i32, AsleepMinutes> = HashMap::new();
    let mut current_guard: Option<i32> = None;
    let mut fell_asleep: Option<u32> = None;

    for line in &loglines {
        match line.event {
            Event::BeginsShift(guard_id) => {
                current_guard = Some(guard_id);
                fell_asleep = None;
            },
            Event::FallsAsleep => {
                fell_asleep = Some(line.dt.minute());
            },
            Event::WakesUp => {
                let woke_up = line.dt.minute();
                if current_guard.is_some() && fell_asleep.is_some() &&
                    fell_asleep.unwrap() < woke_up {
                    let minutes = guard_minutes.entry(current_guard.unwrap()).or_insert([0; 60]);
                    for counter in &mut minutes[fell_asleep.unwrap() as usize .. woke_up as usize] {
                        *counter += 1;
                    }
                }
                fell_asleep = None;
            }
        }
    }

    // Strategy 1

    let minute_sums: Vec<(i32, u32)> = guard_minutes.iter()
        .map(|(guard_id, asleep_minutes)| (*guard_id, asleep_minutes.iter().sum()))
        .collect();

    let max_minutes: HashMap<i32, (u32, u32)> = HashMap::from_iter(guard_minutes.iter().map(|(guard_id, minutes)| {
        (*guard_id, minutes.iter().enumerate().fold((0, 0), |(minute, count), (new_minute, new_count)| {
            if *new_count > count {
                return (new_minute as u32, *new_count);
            }
            (minute, count)
        }))
    }));

    let max_sum_guard_id = minute_sums.iter()
        .fold((0, 0), |(id, max_sum), (new_id, new_sum)| {
            if *new_sum > max_sum {
                return (*new_id, *new_sum);
            }
            (id, max_sum)
        }).0 as i32;

    let max_minute = max_minutes.get(&max_sum_guard_id).unwrap().0 as i32;

    println!("Strategy 1");
    println!("Chosen guard: {}", max_sum_guard_id);
    println!("Chosen minute: {}", max_minute);
    println!("Product: {}", max_minute * max_sum_guard_id);

    // Strategy 2

    let strategy2 = max_minutes.iter().fold((0, 0, 0),
        |(guard_id, minute, count), (new_id, (new_min, new_count))| {
            if *new_count > count {
                return (*new_id, *new_min, *new_count);
            }
            (guard_id, minute, count)
        });

    println!("Strategy 2");
    println!("Chosen guard: {}", strategy2.0);
    println!("Chosen minute: {}", strategy2.1);
    println!("Product: {}", strategy2.0 * strategy2.1 as i32);

    Ok(())
}