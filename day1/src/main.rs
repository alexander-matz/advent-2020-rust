extern crate advent_support;

use std::env;
use std::collections::HashSet;
use std::cmp::{Eq, PartialEq, min, max};
use std::hash::{Hash};
use std::clone::{Clone};

type Number = i32;

fn sort_tuple(t: &(Number, Number)) -> (Number, Number) {
    (min(t.0, t.1), max(t.0, t.1))
}

fn two_combinations(input: &Vec<Number>) -> Vec<(Number, Number)> {
    #[derive(Eq, PartialEq, Hash, Clone)]
    struct Key {
        x: Number,
        y: Number,
    }
    let mut seen: HashSet<Key> = HashSet::new();
    let mut result: Vec<(Number, Number)> = Vec::new();
    for v1 in input {
        for v2 in input {
            let pair = sort_tuple(&(*v2, *v1));
            let key = Key{ x: pair.0, y: pair.1 };
            if ! seen.contains(&key) {
                result.push(pair);
                seen.insert(key);
            }
        }
    };
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines = advent_support::read_lines(&args[1]).unwrap();

    let numbers: Vec<Number> = lines.map(|s| s.unwrap().parse().unwrap()).collect();
    let permutations: Vec<(Number, Number)> = two_combinations(&numbers);

    let solutions: Vec<&(Number, Number)> = permutations.iter()
        .filter(|x| x.0 + x.1 == 2020)
        .collect();
    for (x ,y) in solutions {
        println!("({}, {}) -> {}", x, y, x * y);
    }
}