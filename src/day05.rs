type Num = u32;

type Seat = (Num, Num);

type SeatRange = (Num, Num);

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

fn ranges(xs: &[SeatRange]) -> Vec<SeatRange> {
    let mut xs: Vec<Option<SeatRange>> = xs.iter().cloned().map(Some).collect();
    xs.sort_by_key(|x| x.unwrap());

    for i in 0..(xs.len() - 1) {
        let (a, b) = xs[i].unwrap();
        let (c, d) = xs[i + 1].unwrap();
        if b >= c {
            let p = (a.min(c), b.max(d));
            xs[i + 1] = Some(p);
            xs[i] = None
        }
    }
    xs.into_iter().flatten().collect()
}

pub fn part1(base: &str) -> Num {
    let data = parse(base);
    data.iter().map(as_seat).map(id).max().unwrap()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    let rx: Vec<SeatRange> = data
        .iter()
        .map(as_seat)
        .map(id)
        .map(|x| (x, x + 1))
        .collect();

    let groups = ranges(&rx);
    assert!(groups.len() == 2);
    groups[0].1
}
