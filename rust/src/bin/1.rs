use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let rotations: Vec<_> = BufReader::new(File::open("../1.txt").unwrap())
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let direction = if line.starts_with('L') { -1 } else { 1 };
            isize::from_str(&line[1..]).unwrap() * direction
        })
        .collect();

    let mut dial = 50isize;
    let mut i = 0usize;

    for rot in rotations.iter().copied() {
        dial = (dial + rot) % 100;
        if dial < 0 {
            dial += 100;
        } else if dial == 0 {
            i += 1;
        }
    }

    println!("{}", i);

    dial = 50isize;
    i = 0usize;

    for rot in rotations {
        let full = rot / 100;
        let rem = rot % 100;
        i += full.unsigned_abs();
        let old_dial = dial;
        dial += rem;
        if old_dial != 0 && (dial <= 0 || dial >= 100) {
            i += 1;
        }
        dial %= 100;
        if dial < 0 {
            dial += 100;
        }
    }

    println!("{}", i);
}
