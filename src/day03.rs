type Num = u32;

use std::collections::HashSet;

use num_complex::Complex;

type Position = Complex<Num>;
type Positions = HashSet<Position>;

use crate::utility::read_grid;

type Size = (Num, Num);

fn to_num(c: Complex<usize>) -> Position {
    Position::new(c.re as Num, c.im as Num)
}

fn parse(base: &str) -> (Size, Positions) {
    let b = read_grid(base);
    let size = b
        .keys()
        .map(|c| (c.re as Num + 1, c.im as Num + 1))
        .max()
        .unwrap();
    let trees = b
        .into_iter()
        .filter(|k| k.1 == '#')
        .map(|k| to_num(k.0))
        .collect();

    (size, trees)
}

fn r#loop(p: Position, s: Size) -> Position {
    Position::new(p.re % s.0, p.im)
}

pub fn part1(base: &str) -> usize {
    let (size, trees) = parse(base);
    let path: Positions = (1..(size.1))
        .map(|n| r#loop(Position::new(3 * n, n), size))
        .collect();
    (&path & &trees).len()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
