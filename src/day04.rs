type Num = u32;

type Colour = str;

use std::collections::HashMap;
use std::collections::HashSet;

/*
#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Passport {
    byr: Option<Num>,
}
*/
type Passport = HashMap<String, String>;

/*

fn to_num(c:&Colour) -> Num {
    Num::from_str_radix(c.trim_matches('#'), 16)
}
*/

fn read_row(row: &str) -> Passport {
    row.split_whitespace()
        .map(|x| {
            let mut m = x.split(':').map(|k| k.to_string());
            (m.next().unwrap(), m.next().unwrap())
        })
        .collect()
}

fn parse(base: &str) -> Vec<Passport> {
    base.split("\n\n").map(read_row).collect()
}

fn valid(p: &Passport) -> bool {
    let req = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    p.keys()
        .map(|x| x.as_str())
        .collect::<HashSet<_>>()
        .is_superset(&req)
}

pub fn part1(base: &str) -> usize {
    let data = parse(base);
    data.into_iter().filter(valid).count()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
