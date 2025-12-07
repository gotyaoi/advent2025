use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let lines: Vec<_> = BufReader::new(File::open("../6.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut nums: Vec<_> = lines[..lines.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| usize::from_str(n).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let ops: Vec<_> = lines[lines.len() - 1].split_whitespace().collect();

    println!(
        "{}",
        (0..ops.len())
            .map(|i| {
                let column = nums.iter().map(|n| n[i]);
                if ops[i] == "+" {
                    column.into_iter().sum::<usize>()
                } else {
                    column.into_iter().product()
                }
            })
            .sum::<usize>()
    );

    nums.clear();
    let mut problem = Vec::new();
    let length = lines[0..lines.len() - 1]
        .iter()
        .map(String::len)
        .min()
        .unwrap();
    for i in 0..length {
        let column = lines[0..lines.len() - 1]
            .iter()
            .map(|l| &l[i..i + 1])
            .collect::<String>();
        let col = column.trim();
        if col.is_empty() {
            nums.push(problem);
            problem = Vec::new();
            continue;
        }
        problem.push(usize::from_str(col).unwrap());
    }
    if !problem.is_empty() {
        nums.push(problem);
    }

    println!(
        "{}",
        nums.into_iter()
            .enumerate()
            .map(|(i, column)| if ops[i] == "+" {
                column.into_iter().sum::<usize>()
            } else {
                column.into_iter().product()
            })
            .sum::<usize>()
    );
}
