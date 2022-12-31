type Num = u32;

use std::collections::HashSet;

type Passenger = HashSet<char>;
type Group = Vec<Passenger>;

fn read_row(row: &str) -> Group {
    row.lines().map(|r| r.chars().collect()).collect()
}

fn parse(base: &str) -> Vec<Group> {
    base.split("\n\n").map(read_row).collect()
}

fn any(g: &Group) -> Passenger {
    g.iter()
        .cloned()
        .reduce(|acc, e| &acc | &e)
        .unwrap()
}

pub fn part1(base: &str) -> usize {
    let data = parse(base);
    data.iter().map(any).map(|x| x.len()).sum()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
