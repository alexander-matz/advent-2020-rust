extern crate advent_support;

fn update_bounds(min: i32, max: i32, upper: bool) -> (i32, i32) {
    let step = (max - min)/2;
    let result;
    if upper {
        result = (min+step+1, max);
    } else {
        result = (min, min+step)
    };
    // println!("({}, {}), {} -> ({}, {})", min, max, upper, result.0, result.1);
    result
}

fn parse_boarding_pass(line: &str) -> (i32, i32) {
    assert!(line.len() == 10);
    let mut row_bounds = (0, 127);
    let mut chars = line.chars();
    // println!("deciding row");
    for _ in 0..7 {
        row_bounds = match chars.next().unwrap() {
            'F' => update_bounds(row_bounds.0, row_bounds.1, false),
            'B' => update_bounds(row_bounds.0, row_bounds.1, true),
            c => panic!("unsupported row BSP: {}", c)
        }
    }
    // println!("deciding col");
    let mut col_bounds = (0, 7);
    for _ in 0..3 {
        col_bounds = match chars.next().unwrap() {
            'R' => update_bounds(col_bounds.0, col_bounds.1, true),
            'L' => update_bounds(col_bounds.0, col_bounds.1, false),
            c => panic!("unsupported row BSP: {}", c)
        }
    }
    (row_bounds.0, col_bounds.0)
}

fn seat_id(pos: &(i32, i32)) -> i64 {
    (pos.0 * 8 + pos.1) as i64
}

fn find_missing(seat_ids: &Vec<i64>) -> Vec<i64> {
    assert!(seat_ids.len() > 0);

    let mut result: Vec<i64> = Vec::new();    
    let mut seats = seat_ids.iter();
    let mut last_seat = seats.next().unwrap();
    for seat in seats {
        if seat-last_seat != 1 {
            assert!(seat-last_seat == 2);
            result.push(*seat - 1);
        }
        last_seat = seat
    }
    result
}

fn main() {
    let lines = advent_support::lines_arg1();
    let passes: Vec<(i32, i32)> = lines.iter().map(|x| parse_boarding_pass(&x)).collect();
    let mut seat_ids: Vec<i64> = passes.iter().map(|p| seat_id(p)).collect();
    seat_ids.sort();
    // let max_seat_id = passes.iter().map(|p| seat_id(p)).max().unwrap();
    println!("part1, max seat id: {}", seat_ids.last().unwrap());
    let candidates = find_missing(&seat_ids);
    println!("part2: candidates: {:?}", candidates)
}
