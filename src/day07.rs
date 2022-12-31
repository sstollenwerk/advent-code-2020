type Num = u32;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::utility::map_values;

type Node = String;
type Key = Node;
type Child = (Num, Key);
type Value = Vec<Child>;

type Bags = HashMap<Key, Value>;

fn as_node(s: &str) -> Node {
    s.to_string()
}

fn read_row(row: &str) -> (Key, Value) {
    let row = row
        .replace([',', '.'], "")
        .replace("contain", "")
        .replace("bags", "bag");

    let mut parts: Vec<_> = row.split("bag").map(|s| s.trim()).collect();

    assert!(parts.pop().unwrap().is_empty());

    let key = as_node(parts.remove(0));

    let mut children = Vec::new();

    for v in parts {
        if v == "no other" {
            continue;
        }
        let p = v.split_once(' ').unwrap();
        let n = p.0.parse::<Num>().unwrap();
        let m = as_node(p.1);

        children.push((n, m));
    }

    (key, children)
}

fn parents(b: &Bags) -> Bags {
    let mut res: Bags = Bags::new();
    for (k, v) in b.iter() {
        for (amt, child) in v {
            res.entry(child.to_string())
                .or_default()
                .push((*amt, k.to_string()));
        }
    }
    res
}

fn posses(start: &Node, b: Bags) -> Vec<HashSet<Node>> {
    let groups: HashMap<Node, Vec<Node>> =
        map_values(b, |r| r.iter().map(|x| x.1.clone()).collect::<Vec<_>>());

    let mut currents: HashSet<String> = HashSet::from([start.clone()]);
    let mut totals = Vec::new();
    while !currents.is_empty() {
        currents = currents
            .into_iter()
            .flat_map(|k| groups.get(&k).unwrap_or(&Vec::new()).clone())
            .collect();
        totals.push(currents.clone());
    }

    totals
}

fn required(start: &Node, b: &Bags) -> Num {
    let mut xs = posses(start, b.clone());
    xs.reverse();

    xs.push(HashSet::from([start.clone()]));

    let mut cache: HashMap<Node, Num> = HashMap::new();

    for val in xs.into_iter().flatten().unique() {
        let p = b[&val].clone();

        let total: Num = 1 + p.iter().map(|(amt, b)| amt * cache[b]).sum::<Num>();

        cache.insert(val, total);
    }

    cache[start] - 1
    // -1 because want ot ignore gold bag itself
}

fn parse(base: &str) -> Bags {
    base.lines().map(read_row).collect()
}

pub fn part1(base: &str) -> usize {
    let data = parse(base);

    let inv = parents(&data);
    posses(&"shiny gold".to_string(), inv)
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(base: &str) -> Num {
    let data = parse(base);

    required(&"shiny gold".to_string(), &data)
}
