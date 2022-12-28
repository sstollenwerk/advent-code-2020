type Num = u32;



fn read_row(row: &str) -> Num {
    row.parse::<Num>().unwrap()
}

fn parse(base: &str) -> Vec<Num> {
    base.lines().map(read_row).collect()
}



pub fn part1(base: &str) -> Num {
    let data = parse(base);
    dbg!(&data);
    todo!();
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    todo!();
}
