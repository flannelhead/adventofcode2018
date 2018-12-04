use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

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
    let loglines: Vec<LogLine> = reader.lines().filter_map(|line| line.ok())
        .map(|line| line.parse().unwrap())
        .collect();

    for line in loglines {
        println!("{:?}", line);
    }

    Ok(())
}