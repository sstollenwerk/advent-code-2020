type Num = u64;

use std::collections::HashSet;

use itertools::Itertools;

type Posses = Vec<Num>;

fn read_row(row: &str) -> Num {
    row.parse::<Num>().unwrap()
}

fn parse(base: &str) -> Posses {
    let r = base.lines().map(read_row);
    assert!(r.clone().all_unique());

    r.collect()
}

fn target(data: &Posses, n: usize) -> Num {
    // should really do O(n-1) instead of O(n) but n is small
    data.iter()
        .combinations(n)
        .find(|xs| xs.clone().into_iter().sum::<Num>() == 2020)
        .unwrap()
        .into_iter()
        .product()
}

pub fn part1(base: &str) -> Num {
    let data = parse(base);
    target(&data, 2)
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    target(&data, 3)
}
