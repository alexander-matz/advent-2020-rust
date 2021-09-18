extern crate advent_support;
extern crate regex;

#[derive(Debug)]
struct PasswordPolicy {
    min: i32,
    max: i32,
    letter: char,
}

#[derive(Debug)]
struct PasswordAttempt {
    policy: PasswordPolicy,
    password: String,
}

#[derive(Debug)]
struct PasswordAttempt2 {
    positions: Vec<i32>,
    letter: char,
    password: String,
}

fn check_attempt(attempt: &PasswordAttempt) -> bool {
    let letter = attempt.policy.letter;
    let count = attempt.password.chars()
        .filter(|x| *x == letter)
        .count() as i32;
    return count >= attempt.policy.min && count <= attempt.policy.max;
}

fn parse_attempt(re: &regex::Regex, line: &str) -> PasswordAttempt {
    match re.captures(line) {
        Some(m) => {
            PasswordAttempt{
                policy: PasswordPolicy{
                    min: m.get(1).unwrap().as_str().parse().unwrap(),
                    max: m.get(2).unwrap().as_str().parse().unwrap(),
                    letter: m.get(3).unwrap().as_str().chars().next().unwrap()
                },
                password: String::from(m.get(4).unwrap().as_str())
            }
        },
        None => panic!("Line '{}' did not describe password spec", line)
    }
}

fn part1(lines: &Vec<String>) {
    let re: regex::Regex = regex::Regex::new(r"^\s*(\d+)-(\d+)\s+([a-zA-Z0-9]):\s+([a-zA-Z0-9]+)\s*$").unwrap();
    let attempts: Vec<PasswordAttempt> = lines.iter()
        .filter(|x| !x.is_empty())
        .map(|x| parse_attempt(&re, x))
        .collect();
    let num_valid = attempts.iter().filter(|x| check_attempt(x)).count();
    println!("part1: {}", num_valid)
}

fn parse_attempt2(re: &regex::Regex, line: &str) -> PasswordAttempt2 {
    match re.captures(line) {
        Some(m) => {
            let positions: Vec<i32> = m.get(1).unwrap().as_str().split("-").map(|x| x.parse().unwrap()).collect();
            PasswordAttempt2{
                positions: positions,
                letter: m.get(2).unwrap().as_str().chars().next().unwrap(),
                password: String::from(m.get(3).unwrap().as_str()),
            }
        },
        None => panic!("Line '{}' did not describe password spec", line)
    }
}

fn check_attempt2(attempt: &PasswordAttempt2) -> bool {
    let letter = attempt.letter;
    let mut positions_with_letter = 0;
    let mut consumed = 1; // start counting at 1
    let password = &attempt.password;
    let mut chars = password.chars();
    for position in &attempt.positions {
        let at_pos = chars.nth((position - consumed) as usize).unwrap();
        // println!("letter at {} in {}: {}", position, password, at_pos);
        if at_pos == letter {
            positions_with_letter += 1;
        }
        consumed += position;
    }
    // println!("Password {}, occurences of {}: {} ", password, letter, positions_with_letter);
    positions_with_letter == 1
}

fn part2(lines: &Vec<String>) {
    let re = regex::Regex::new(r"^\s*([-0-9]+)\s+([a-zA-Z0-9]):\s+([a-zA-Z0-9]+)\s*$").unwrap();
    let attempts: Vec<PasswordAttempt2> = lines.iter()
        .filter(|x| !x.is_empty())
        .map(|x| parse_attempt2(&re, x))
        .collect();
    let num_valid = attempts.iter().filter(|x| check_attempt2(x)).count();
    println!("part2: {}", num_valid)
}

fn main() {
    let args = advent_support::argv();
    let lines = advent_support::read_lines2(&args[1]);
    if args.len() == 3 && args[2] == "part2" {
        part2(&lines)
    } else {
        part1(&lines)
    }
}
