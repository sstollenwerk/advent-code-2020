#![allow(dead_code)]

use core::hash::Hash;
use num_complex::Complex;
use std::collections::HashMap;
use std::collections::HashSet;

type Position = Complex<usize>;

pub fn read_grid(base: &str) -> HashMap<Position, char> {
    let mut res: HashMap<Position, char> = HashMap::new();
    for (y, row) in base.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let p = Position::new(x, y);
            res.insert(p, c);
        }
    }
    res
}

pub fn map_values<A: Hash + Eq, B, C>(h: HashMap<A, B>, f: fn(B) -> C) -> HashMap<A, C> {
    h.into_iter().map(|(k, v)| (k, f(v))).collect()
}

pub fn map_keys<A: Hash + Eq, B, C: Hash + Eq>(h: HashMap<A, B>, f: fn(A) -> C) -> HashMap<C, B> {
    h.into_iter().map(|(k, v)| (f(k), v)).collect()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    //copied from https://stackoverflow.com/a/64499219
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn as_disp(g: &HashSet<Complex<i32>>) -> Vec<Vec<bool>> {
    let top = g.iter().map(|c| c.im).min().unwrap();
    let bottom = g.iter().map(|c| c.im).max().unwrap();
    let left = g.iter().map(|c| c.re).min().unwrap();
    let right = g.iter().map(|c| c.re).max().unwrap();

    let mut res = Vec::new();
    // let mut part_res: Vec<String> = Vec::new();
    for y in top..=bottom {
        let mut line = Vec::new();
        for x in left..=right {
            let cont = g.contains(&Complex::new(x, y));
            line.push(cont);
        }
        res.push(line)
    }
    res
}

pub fn display(g: &HashSet<Complex<i32>>) {
    let top = g.iter().map(|c| c.im).min().unwrap();
    let bottom = g.iter().map(|c| c.im).max().unwrap();
    let left = g.iter().map(|c| c.re).min().unwrap();
    let right = g.iter().map(|c| c.re).max().unwrap();

    // let mut part_res: Vec<String> = Vec::new();
    for y in top..=bottom {
        let mut line: String = String::new();
        for x in left..=right {
            let cont = g.contains(&Complex::new(x, y));
            let c = if cont { '*' } else { ' ' };
            line.push(c);
        }
        println!("{:?}", &line);

        //    part_res.push(line);
    }
}

pub fn sets_union<T: Hash + Eq + Clone>(r: &[HashSet<T>]) -> HashSet<T> {
    r.iter().cloned().reduce(|acc, e| (&acc | &e)).unwrap()
}

pub fn bin_items<T: Eq + Clone, V: Hash + Eq + Clone>(
    xs: &[T],
    f: fn(T) -> V,
) -> HashMap<V, Vec<T>> {
    let mut res: HashMap<V, Vec<T>> = HashMap::new();
    for x in xs.iter() {
        let k = f(x.clone());
        res.entry(k).or_default().push(x.clone());
    }
    res
}
