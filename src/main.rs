use std::fs;

mod day01;
//mod utility;
use day01::{part1, part2};

fn read_day(day: i32) -> String {
    let filename = to_filename(day);
    fs::read_to_string(filename)
        .expect("Could not read file")
        .replace("\r\n", "\n")
}

fn to_filename(day: i32) -> String {
    format!("data/{:0>2}.txt", day)
}

fn main() {
    let data = read_day(1);
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}
