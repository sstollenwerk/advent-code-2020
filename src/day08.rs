type Num = i32;

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
enum Operation {
    Acc(Num),
    Jmp(Num),
    Nop(Num),
}

fn read_row(row: &str) -> Operation {
    let p = row.split_once(' ').unwrap();

    let mut s = p.1.to_string();

    let mult = match s.remove(0) {
        '+' => 1,
        '-' => -1,
        _ => unreachable!(),
    };

    let n = s.parse::<Num>().unwrap() * mult;

    match p.0 {
        "acc" => Operation::Acc(n),
        "jmp" => Operation::Jmp(n),
        "nop" => Operation::Nop(n),
        _ => unreachable!(),
    }
}

fn parse(base: &str) -> Vec<Operation> {
    base.lines().map(read_row).collect()
}

fn interpret(xs: &[Operation]) -> Result<Num, Num> {
    let mut seen = HashSet::new();
    let mut acc = 0;

    let mut i = 0;
    loop {
        if i == xs.len() {
            break;
        }

        let out = seen.insert(i);
        if !out {
            return Err(acc);
        }
        match xs[i] {
            Operation::Acc(n) => {
                acc += n;
                i += 1;
            }
            Operation::Jmp(n) => {
                i = ((i as Num) + (n)) as usize;
            }
            Operation::Nop(_n) => {
                i += 1;
            }
        }
    }
    Ok(acc)
}

pub fn part1(base: &str) -> Num {
    let data = parse(base);

    interpret(&data).unwrap_err()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    for (i, el) in data.iter().enumerate() {
        let mut poss = data.clone();
        match el {
            Operation::Acc(_n) => {
                continue;
            }
            Operation::Jmp(n) => {
                poss[i] = Operation::Nop(*n);
            }
            Operation::Nop(n) => {
                poss[i] = Operation::Jmp(*n);
            }
        };
        let res = interpret(&poss);
        if let Ok(x) = res {
            return x;
        }
    }

    unreachable!();
}
