use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<i32> = s.split(',')
            .map(|s1| s1.trim().parse().unwrap())
            .collect();
        if fields.len() < 2 {
            return Err("Couldn't parse point")
        }

        Ok(Point {
            x: fields[0],
            y: fields[1]
        })
    }
}

fn distance(pt1: &Point, pt2: &Point) -> u32 {
    ((pt1.x - pt2.x).abs() + (pt1.y - pt2.y).abs()) as u32
}

fn find_bounding_box(points: &Vec<Point>) -> Rect {
    let xs: Vec<i32> = points.iter().map(|pt| pt.x).collect();
    let ys: Vec<i32> = points.iter().map(|pt| pt.y).collect();

    Rect {
        left: *xs.iter().min().unwrap(),
        right: *xs.iter().max().unwrap() + 1,
        top: *ys.iter().min().unwrap(),
        bottom: *ys.iter().max().unwrap() + 1
    }
}

fn main() -> std::io::Result<()> {
    let mut datafile = File::open("data/6_1")?;
    let mut data = String::new();
    datafile.read_to_string(&mut data)?;

    let points: Vec<Point> = data.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    let bbox = find_bounding_box(&points);
    let width = bbox.right - bbox.left;
    let height = bbox.bottom - bbox.top;
    let mut areas: Vec<u32> = vec![0; points.len()];
    let mut sum_distances: Vec<Vec<u32>> = vec![vec![0; width as usize]; height as usize];
    let mut is_infinite: Vec<bool> = vec![false; points.len()];

    for x in bbox.left..bbox.right {
        for y in bbox.top..bbox.bottom {
            let pt = Point { x, y };
            let mut distances: Vec<(usize, u32)> = points.iter().enumerate()
                .map(|(index, pt1)| (index, distance(&pt, pt1)))
                .collect();
            let yidx = (y - bbox.top) as usize;
            let xidx = (x - bbox.left) as usize;
            sum_distances[yidx][xidx] = distances.iter().map(|(_, dist)| dist).sum();
            distances.sort_by(|(_, dst1), (_, dst2)| dst1.cmp(dst2));
            if distances[0].1 != distances[1].1 {
                // Not a tie, mark in the map
                areas[distances[0].0] += 1;
                if x == bbox.left || x == bbox.right - 1 || y == bbox.top || y == bbox.bottom - 1 {
                    is_infinite[distances[0].0] = true;
                }
            }
        }
    }

    let non_infinite_areas: Vec<u32> = areas.iter().zip(is_infinite.iter())
        .filter(|(_, infinite)| !(**infinite)).map(|(area, _)| *area).collect();
    let count_safe = sum_distances.iter().flatten().filter(|sum| **sum < 10000).count();

    println!("max area: {}", non_infinite_areas.iter().max().unwrap());
    println!("safe area size: {}", count_safe);

    Ok(())
}
