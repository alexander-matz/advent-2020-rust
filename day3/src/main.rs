extern crate advent_support;

use advent_support::{Array2d};

type Forest = Array2d<bool>;

fn parse_forest(lines: &Vec<String>) -> Forest {
    assert!(lines.len() > 0);
    let cols = lines[0].len();
    assert!(cols > 0);
    for line in &lines[1..] {
        assert!(line.len() == cols);
    }
    let rows = lines.len();
    let mut forest = Forest::new(cols, rows, false);
    for row in 0..rows {
        let mut chars = lines[row].chars();
        for col in 0..cols {
            if chars.next().unwrap() == '#' {
                forest.set(col, row, true);
            }
        }
    }
    forest
}

fn dump(forest: &Forest) {
    let printable = forest.map_to(|c| if *c { '#' } else { '.' });
    printable.dump()
}

fn trees_on_slope(forest: &Forest, xd: usize, yd: usize) -> i32 {
    assert!(yd > 0);
    let mut count = 0;
    let mut xoffset = 0;
    let mut yoffset = 0;
    while yoffset < forest.rows() {
        if forest.get_wrapped(xoffset, yoffset) {
            count += 1;
        }
        xoffset += xd;
        yoffset += yd;
    }
    count
}

fn main() {
    let lines = advent_support::read_lines2(&advent_support::argv()[1]);
    let forest = parse_forest(&lines);
    println!("part1: {}", trees_on_slope(&forest, 3, 1));

    let mut product: i64 = 1;
    for (xd, yd) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let trees = trees_on_slope(&forest, xd, yd);
        println!("Encountered {} trees on slope {}, {}", trees, xd, yd);
        product *= trees as i64;
    }
    println!("part2: {}", product);
}
