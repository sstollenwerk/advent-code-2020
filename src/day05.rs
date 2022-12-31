type Num = u32;

type Seat = (Num, Num);

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Bisect {
    Upper,
    Lower,
}

type Boarding = Vec<Bisect>;

fn read_row(row: &str) -> Boarding {
    row.chars()
        .map(|c| match c {
            'F' => Bisect::Lower,
            'B' => Bisect::Upper,

            'L' => Bisect::Lower,
            'R' => Bisect::Upper,
            _ => unreachable!(),
        })
        .collect()
}

fn parse(base: &str) -> Vec<Boarding> {
    base.lines().map(read_row).collect()
}

fn from_bi(r: &[Bisect]) -> Num {
    let mut lo = 0;
    let mut hi: Num = (2 as Num).pow(r.len() as Num);
    for i in r {
        let mid = (lo + hi) / 2;
        match i {
            Bisect::Lower => {
                hi = mid;
            }
            Bisect::Upper => {
                lo = mid;
            }
        }
    }
    assert!(lo + 1 == hi);
    lo
}

fn as_seat(r: &Boarding) -> Seat {
    assert!(r.len() == 10);

    (from_bi(&r[0..7]), from_bi(&r[7..10]))
}

fn id(s: Seat) -> Num {
    let (row, column) = s;
    row * 8 + column
}

pub fn part1(base: &str) -> Num {
    let data = parse(base);
    data.iter().map(as_seat).map(id).max().unwrap()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
