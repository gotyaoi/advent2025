use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let mut lines = BufReader::new(File::open("../5.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut ingredients = Vec::new();
    'outer: for line in lines.by_ref() {
        if !line.contains('-') {
            ingredients.push(usize::from_str(&line).unwrap());
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let mut start = usize::from_str(start).unwrap();
        let mut end = usize::from_str(end).unwrap();
        let mut kill = Vec::new();
        let mut new_range;
        'inner: loop {
            new_range = start..=end;
            for (i, r) in ranges.iter().enumerate() {
                if new_range.contains(r.start()) && new_range.contains(r.end()) {
                    kill.push(r.clone());
                    continue;
                }
                if r.contains(&start) {
                    if r.contains(&end) {
                        continue 'outer;
                    }
                    start = *r.start();
                    ranges.swap_remove(i);
                    continue 'inner;
                }
                if r.contains(&end) {
                    end = *r.end();
                    ranges.swap_remove(i);
                    continue 'inner;
                }
            }
            break;
        }
        ranges.push(new_range);
        for k in kill {
            ranges.swap_remove(ranges.iter().position(|p| *p == k).unwrap());
        }
    }
    for line in lines {
        ingredients.push(usize::from_str(&line).unwrap());
    }

    println!(
        "{}",
        ingredients
            .iter()
            .filter(|i| ranges.iter().any(|r| r.contains(i)))
            .count()
    );

    println!(
        "{}",
        ranges
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<usize>()
    );
}
