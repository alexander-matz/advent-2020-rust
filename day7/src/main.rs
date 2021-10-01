extern crate advent_support;

use std::str::{FromStr, Split};

#[derive(Debug)]
struct Bag {
    color: String,
    style: String,
}

#[derive(Debug)]
struct BagsRule {
    name: Bag,
    contains: Vec<(usize, Bag)>,
}

fn trim(s: &str) -> &str {
    s.trim()
}

fn parse_bag_aux(parts: &[&str]) -> Bag {
    assert!(parts.len() == 3);
    Bag {
        style: String::from(parts[0]),
        color: String::from(parts[1]),
    }
}

fn parse_bag(bags: &str) -> Bag {
    let parts: Vec<&str> = bags.split(" ").collect();
    parse_bag_aux(&parts)
}

fn parse_counted_bag(desc: &str) -> Option<(usize, Bag)> {
    if desc == "no other bags" {
        return None;
    }
    let parts: Vec<&str> = desc.split(" ").collect();
    assert!(parts.len() == 4);
    let count = parts[0].parse::<usize>().unwrap();
    let bag = parse_bag_aux(&parts[1..]);
    Some(
        (count, bag)
    )
}

fn parse_rhs(rhs: &str) -> Vec<(usize, Bag)> {
    let parts: Vec<&str> = rhs.split(",").map(trim).collect();
    let bags = parts.iter()
        .map(|x| parse_counted_bag(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    bags
}

fn parse_sentence(sentence: &str) -> BagsRule {
    let wat_parts: Vec<&str> = sentence.split("contain").map(trim).collect();
    assert!(wat_parts.len() == 2);
    let name = parse_bag(&wat_parts[0]);
    assert!(wat_parts[1].len() > 0);
    let rhs = parse_rhs(&wat_parts[1]);
    BagsRule{
        name: name,
        contains: rhs,
    }
}

fn parse_lines(s: &str) -> Vec<BagsRule> {
    let sentences: Vec<&str> = s.split(".").map(trim).collect();
    let rules: Vec<BagsRule> = sentences.iter()
        .filter(|x| !x.is_empty())
        .map(|x| parse_sentence(x))
        .collect();
    rules
}

fn main() {
    let contents = advent_support::contents_arg1();
    println!("Hello, world!");
    let bags = parse_lines(&contents);
    println!("{:?}", bags);
}