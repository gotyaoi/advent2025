use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("../7.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    let first = lines.next().unwrap();
    let width = first.len();
    let start = first
        .chars()
        .enumerate()
        .find(|(_i, c)| *c == 'S')
        .unwrap()
        .0;
    let splitters: Vec<_> = lines
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect::<HashSet<_>>()
        })
        .collect();

    let mut beams = vec![0usize; width];
    beams[start] = 1;
    let mut splits = 0usize;

    for row in splitters.iter() {
        let mut new_beams = vec![0; width];
        for (i, beam) in beams.into_iter().enumerate() {
            if beam == 0 {
                continue;
            }
            if row.contains(&i) {
                splits += 1;
                new_beams[i + 1] += beam;
                new_beams[i - 1] += beam;
            } else {
                new_beams[i] += beam;
            }
        }
        beams = new_beams;
    }

    println!("{}", splits);
    println!("{}", beams.into_iter().sum::<usize>());
}
