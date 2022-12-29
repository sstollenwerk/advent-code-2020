type Password = Vec<char>;
type Row = (usize, usize, char, Password);

fn read_row(row: &str) -> Row {
    let binding = row.replace('-', " ").replace(':', "");
    let vals: Vec<&str> = binding.split(' ').collect();

    (
        vals[0].parse().unwrap(),
        vals[1].parse().unwrap(),
        vals[2].chars().next().unwrap(),
        vals[3].chars().collect(),
    )
}

fn parse(base: &str) -> Vec<Row> {
    base.lines().map(read_row).collect()
}

fn count<T: PartialEq>(xs: &[T], c: &T) -> usize {
    xs.iter().filter(|n| n == &c).count()
}

fn valid1(r: &Row) -> bool {
    let (low, hi, check, xs) = r.clone();
    let c = count(&xs, &check);
    low <= c && c <= hi
}

fn valid2(r: &Row) -> bool {
    let (low, hi, check, xs) = r;

    [low, hi]
        .into_iter()
        .filter(|n| xs[*n - 1] == *check)
        .count()
        == 1
}

pub fn part1(base: &str) -> usize {
    let data = parse(base);
    data.iter().filter(|n| valid1(n)).count()
}

pub fn part2(base: &str) -> usize {
    let data = parse(base);

    data.iter().filter(|n| valid2(n)).count()
}
