extern crate advent_support;

struct GroupAnswers {
    group_size: i32,
    answers: std::collections::HashMap<char, i32>
}

impl GroupAnswers {
    fn new() -> GroupAnswers {
        GroupAnswers{
            group_size: 0,
            answers: std::collections::HashMap::<char, i32>::new()
        }
    }
}

fn add_to_group(answers: &mut GroupAnswers, line: &str) {
    answers.group_size += 1;
    for ch in line.chars() {
        let entry = answers.answers.entry(ch).or_insert(0);
        *entry += 1;
    }
}

fn parse_groups(lines: &Vec<String>) -> Vec<GroupAnswers> {
    let mut result: Vec<GroupAnswers> = Vec::new();
    let mut curr_answer: GroupAnswers = GroupAnswers::new();
    let mut was_touched = false;
    for line in lines {
        if line.len() > 0 {
            add_to_group(&mut curr_answer, line);
            was_touched = true;
        } else {
            result.push(curr_answer);
            curr_answer = GroupAnswers::new();
            was_touched = true;
        }
    }
    if was_touched {
        result.push(curr_answer);
    }
    result
}

fn get_count1(answers: &GroupAnswers) -> i32 {
    answers.answers.len() as i32
}

fn get_count2(answers: &GroupAnswers) -> i32 {
    let mut group_sum = 0;
    for (_, v) in &answers.answers {
        if *v == answers.group_size {
            group_sum += 1;
        }
    }
    group_sum
}

fn main() {
    let lines = advent_support::lines_arg1();
    let answers = parse_groups(&lines);
    let counts: Vec<i32> = answers.iter().map(|x| get_count1(x)).collect();
    let sum1 = counts.iter().fold(0, |a, b| a + b);
    println!("part1 - sum: {}", sum1);

    let counts2: Vec<i32> = answers.iter().map(|x| get_count2(x)).collect();
    let sum2 = counts2.iter().fold(0, |a, b| a + b);
    println!("part2 - sum: {}", sum2);
}
