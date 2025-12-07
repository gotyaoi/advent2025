use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let banks: Vec<_> = BufReader::new(File::open("../3.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    println!(
        "{}",
        banks.iter().map(|bank| get_joltage(bank, 2)).sum::<usize>()
    );

    println!(
        "{}",
        banks
            .iter()
            .map(|bank| get_joltage(bank, 12))
            .sum::<usize>()
    );
}

fn get_joltage(bank: &str, cells: usize) -> usize {
    let length = bank.len();
    let mut joltage = String::with_capacity(cells);
    let mut index = 0;
    for i in 0..cells {
        let largest = bank[index..length - (cells - 1 - i)].chars().max().unwrap();
        index += bank[index..].find(largest).unwrap() + 1;
        joltage.push(largest);
    }
    usize::from_str(&joltage).unwrap()
}
