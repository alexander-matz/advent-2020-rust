extern crate advent_support;
extern crate regex;
extern crate lazy_static;

use lazy_static::lazy_static;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i64>,
}

lazy_static! {
    static ref HEIGHT_RE: regex::Regex = regex::Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HAIR_RE: regex::Regex = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_RE: regex::Regex = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: regex::Regex = regex::Regex::new(r"^[0-9]{9}$").unwrap();
}

fn add_to_passport(pass: &mut Passport, partial_desc: &String) {
    let fields = partial_desc.split(" ");
    for field in fields {
        let kv: Vec<&str> = field.split(":").collect();
        assert!(kv.len() == 2);
        let val = kv[1];
        match kv[0] {
            "byr" => pass.byr = Some(val.parse().unwrap()),
            "iyr" => pass.iyr = Some(val.parse().unwrap()),
            "eyr" => pass.eyr = Some(val.parse().unwrap()),
            "hgt" => pass.hgt = Some(String::from(val)),
            "hcl" => pass.hcl = Some(String::from(val)),
            "ecl" => pass.ecl = Some(String::from(val)),
            "pid" => pass.pid = Some(String::from(val)),
            "cid" => pass.cid = Some(val.parse().unwrap()),
            _ => {
                panic!("unsupported key: {}", kv[0])
            }
        }
    }
}

fn parse_passports(lines: &Vec<String>) -> Vec<Passport> {
    let mut result: Vec<Passport> = Vec::new();
    let mut curr_pass = Passport{..Default::default()};
    let mut was_modified = false;
    for line in lines {
        if line.len() > 0 {
            add_to_passport(&mut curr_pass, line);
            was_modified = true;
        } else {
            if was_modified {
                result.push(curr_pass);
            }
            curr_pass = Passport{..Default::default()};
            was_modified = false;
        }
    }
    if was_modified {
        result.push(curr_pass);
    }
    result
}

fn passport_valid(p: &Passport) -> bool {
    p.byr.is_some() && 
    p.iyr.is_some() &&
    p.eyr.is_some() &&
    p.hgt.is_some() &&
    p.hcl.is_some() &&
    p.ecl.is_some() &&
    p.pid.is_some() &&
    // p.cid.is_some() &&
    true
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn check_byr(p: &Passport) -> bool {
    match p.byr {
        Some(val) => val >= 1920 && val <= 2002,
        None => false
    }
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn check_iyr(p: &Passport) -> bool {
    match p.iyr {
        Some(val) => val >= 2010 && val <= 2020,
        None => false
    }
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn check_eyr(p: &Passport) -> bool {
    match p.eyr {
        Some(val) => val >= 2020 && val <= 2030,
        None => false
    }
}

// hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
fn check_hgt(p: &Passport) -> bool {
    match &p.hgt {
        Some(hgt) => match HEIGHT_RE.captures(&hgt) {
            Some(m) => {
                let number = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
                match m.get(2).unwrap().as_str() {
                    "cm" => number >= 150 && number <= 193,
                    "in" => number >= 59 && number <= 76,
                    _ => false
                }
            }
            None => false
        },
        None => false
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn check_hcl(p: &Passport) -> bool {
    match &p.hcl {
        Some(hcl) => match HAIR_RE.captures(&hcl) {
            Some(_) => true,
            None => false
        },
        None => false
    }
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn check_ecl(p: &Passport) -> bool {
    match &p.ecl {
        Some(ecl) => match EYE_RE.captures(&ecl) {
            Some(_) => true,
            None => false
        },
        None => false
    }
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn check_pid(p: &Passport) -> bool {
    match &p.pid {
        Some(pid) => match PID_RE.captures(&pid) {
            Some(_) => true,
            None => false
        },
        None => false
    }
}

// cid (Country ID) - ignored, missing or not.
fn check_cid(_: &Passport) -> bool {
    true
}


fn passport_valid2(p: &Passport) -> bool {
    check_byr(p) && 
    check_iyr(p) &&
    check_eyr(p) &&
    check_hgt(p) &&
    check_hcl(p) &&
    check_ecl(p) &&
    check_pid(p) &&
    check_cid(p) &&
    true
}

fn main() {
    let lines = advent_support::lines_arg1();
    // for line in &lines {
    //     println!("{}", line)
    // }
    let passports = parse_passports(&lines);
    // for p in &passports {
    //     println!("{:?}", p)
    // }
    let valid = passports.iter().filter(|p| passport_valid(p)).count();
    println!("part1 - valid passports: {}", valid);

    let valid2 = passports.iter().filter(|p| passport_valid2(p)).count();
    println!("part2 - valid passports: {}", valid2);
}
