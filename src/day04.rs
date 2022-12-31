type Num = u32;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Height {
    Cm,
    In,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Passport {
    byr: Num,
    iyr: Num,
    eyr: Num,
    hgt: (Height, Num),
    hcl: String,
    ecl: String,
    pid: String,
}

type UncheckedPassport = HashMap<String, String>;

/*

fn to_num(c:&Colour) -> Num {
    Num::from_str_radix(c.trim_matches('#'), 16)
}
*/

fn read_row(row: &str) -> Option<Passport> {
    let parts: HashMap<String, String> = row
        .split_whitespace()
        .map(|x| {
            let mut m = x.split(':').map(|k| k.to_string());
            (m.next().unwrap(), m.next().unwrap())
        })
        .collect();

    let byr = parts.get("byr")?.parse::<Num>().ok()?;
    let iyr = parts.get("iyr")?.parse::<Num>().ok()?;
    let eyr = parts.get("eyr")?.parse::<Num>().ok()?;

    let tmp = parts.get("hgt")?;

    let hgt = if let Some(m) = tmp.strip_suffix("cm") {
        (Height::Cm, m.parse::<Num>().ok()?)
    } else if let Some(m) = tmp.strip_suffix("in") {
        (Height::In, m.parse::<Num>().ok()?)
    } else {
        return None;
    };

    let hcl = parts.get("hcl")?.to_string();
    let ecl = parts.get("ecl")?.to_string();
    let pid = parts.get("pid")?.to_string();

    Some(Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
    })
}
fn parse_old(base: &str) -> Vec<UncheckedPassport> {
    base.split("\n\n").map(read_row_old).collect()
}

fn read_row_old(row: &str) -> UncheckedPassport {
    row.split_whitespace()
        .map(|x| {
            let mut m = x.split(':').map(|k| k.to_string());
            (m.next().unwrap(), m.next().unwrap())
        })
        .collect()
}

fn valid_old(p: &UncheckedPassport) -> bool {
    let req = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

    p.keys()
        .map(|x| x.as_str())
        .collect::<HashSet<_>>()
        .is_superset(&req)
}

fn valid(p: &Passport) -> bool {
    let mut vals = Vec::new();
    let b = p.byr;
    vals.push((1920..=2002).contains(&b));
    let b = p.iyr;
    vals.push((2010..=2020).contains(&b));
    let b = p.eyr;
    vals.push((2020..=2030).contains(&b));

    let (t, hgt) = p.hgt;
    let (start, end) = match t {
        Height::Cm => (150, 193),
        Height::In => (59, 76),
    };
    vals.push((start..=end).contains(&hgt));

    vals.push(
        p.hcl
            .strip_prefix('#')
            .map(|x| x.len() == 6 && Num::from_str_radix(x, 16).is_ok())
            .unwrap_or(false),
    );

    vals.push(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&p.ecl.as_str()));

    let b = &p.pid;
    vals.push(b.len() == 9 && Num::from_str_radix(b, 10).is_ok());

    vals.into_iter().all(|x| x)
}

fn parse(base: &str) -> Vec<Passport> {
    base.split("\n\n").flat_map(read_row).collect()
}

pub fn part1(base: &str) -> usize {
    let data = parse_old(base);
    data.into_iter().filter(valid_old).count()
}

pub fn part2(base: &str) -> usize {
    let data = parse(base);

    data.into_iter().filter(valid).count()
}
