use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut points: Vec<(usize, usize, usize)> = BufReader::new(File::open("../8.txt").unwrap())
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    points.sort_unstable();
    let mut known: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, first)| {
            points.iter().skip(i + 1).map(move |second| {
                (
                    (first, second),
                    ((first.0.abs_diff(second.0).pow(2)
                        + first.1.abs_diff(second.1).pow(2)
                        + first.2.abs_diff(second.2).pow(2)) as f64)
                        .sqrt(),
                )
            })
        })
        .collect();
    known.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut singletons: HashSet<_> = points.iter().collect();
    let mut circuits: Vec<HashSet<_>> = Vec::new();
    for (i, (pair, distance)) in known.into_iter().enumerate() {
        singletons.remove(pair.0);
        singletons.remove(pair.1);
        let mut to_add = Vec::new();
        for (j, circuit) in circuits.iter().enumerate() {
            if circuit.contains(pair.0) {
                to_add.push(j);
            } else if circuit.contains(pair.1) {
                to_add.push(j);
            }
        }
        if to_add.is_empty() {
            let mut new_circuits = HashSet::new();
            new_circuits.insert(pair.0);
            new_circuits.insert(pair.1);
            circuits.push(new_circuits);
        } else if to_add.len() == 1 {
            circuits[to_add[0]].insert(pair.0);
            circuits[to_add[0]].insert(pair.1);
        } else if to_add.len() == 2 {
            let other = circuits.swap_remove(to_add[1]);
            circuits[to_add[0]].extend(other);
        } else {
            panic!();
        }
        if i == 999 {
            let mut lengths: Vec<_> = circuits.iter().map(|c| c.len()).collect();
            lengths.sort_unstable_by_key(|l| Reverse(*l));
            println!("{}", lengths[..3].iter().product::<usize>());
        }
        if singletons.is_empty() {
            println!("{}", pair.0 .0 * pair.1 .0);
            break;
        }
    }
}
