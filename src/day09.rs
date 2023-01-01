type Num = i64;

use std::collections::HashSet;

fn read_row(row: &str) -> Num {
    row.parse::<Num>().unwrap()
}

fn parse(base: &str) -> Vec<Num> {
    base.lines().map(read_row).collect()
}

fn excluded(xs: &[Num]) -> Option<Num> {
    assert!(xs.len() == 26);
    let ax: HashSet<Num> = xs[..25].iter().cloned().collect();
    let k = xs[25];
    let rems: HashSet<Num> = ax.iter().map(|n| (k - *n)).collect();
    if ax.is_disjoint(&rems) {
        Some(k)
    } else {
        None
    }
}

fn invalid(xs: &[Num]) -> Num {
    xs.windows(26).flat_map(excluded).next().unwrap()
}

pub fn part1(base: &str) -> Num {
    let data = parse(base);
    invalid(&data)
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
