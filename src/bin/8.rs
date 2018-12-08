use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>
}

fn parse_node<'a>(data: &mut impl Iterator<Item=&'a i32>) -> Node {
    let n_children = *data.next().unwrap();
    let n_meta = *data.next().unwrap();
    let mut children: Vec<Node> = Vec::new();
    for _ in 0..n_children {
        children.push(parse_node(data));
    }
    let mut metadata: Vec<i32> = Vec::new();
    for _ in 0..n_meta {
        metadata.push(*data.next().unwrap());
    }
    Node {
        metadata: metadata,
        children: children
    }
}

fn sum_meta(node: &Node) -> i32 {
    node.metadata.iter().sum::<i32>() + node.children.iter().map(|node| sum_meta(node)).sum::<i32>()
}

fn main() -> std::io::Result<()> {
    let mut datafile = File::open("data/8")?;
    let mut data = String::new();
    datafile.read_to_string(&mut data)?;
    let numbers: Vec<i32> = data.split(' ')
        .filter_map(|x| x.trim().parse().ok())
        .collect();

    let tree = parse_node(&mut numbers.iter());
    println!("{:?}", sum_meta(&tree));

    Ok(())
}
