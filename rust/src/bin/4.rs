use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rolls = BufReader::new(File::open("../4.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c == '@').collect())
        .collect();

    let (mut total, mut rolls) = process(rolls);

    println!("{}", total);

    loop {
        let (removable, new_rolls) = process(rolls);
        rolls = new_rolls;
        if removable == 0 {
            break;
        }
        total += removable;
    }

    println!("{}", total);
}

fn process(rolls: Vec<Vec<bool>>) -> (usize, Vec<Vec<bool>>) {
    let max_y = rolls.len() - 1;
    let max_x = rolls[0].len() - 1;
    let mut removable = 0;
    let mut new_rolls = Vec::with_capacity(max_y + 1);

    for (y, row) in rolls.iter().enumerate() {
        let mut new_row = Vec::with_capacity(max_x + 1);
        for (x, c) in row.iter().enumerate() {
            if !c {
                new_row.push(false);
                continue;
            }
            let mut surround = 0;
            if y > 0 && rolls[y - 1][x] {
                surround += 1;
            }
            if y > 0 && x < max_x && rolls[y - 1][x + 1] {
                surround += 1;
            }
            if x < max_x && rolls[y][x + 1] {
                surround += 1;
            }
            if y < max_y && x < max_x && rolls[y + 1][x + 1] {
                surround += 1;
            }
            if y < max_y && rolls[y + 1][x] {
                surround += 1;
            }
            if y < max_y && x > 0 && rolls[y + 1][x - 1] {
                surround += 1;
            }
            if x > 0 && rolls[y][x - 1] {
                surround += 1;
            }
            if y > 0 && x > 0 && rolls[y - 1][x - 1] {
                surround += 1;
            }
            if surround < 4 {
                new_row.push(false);
                removable += 1;
            } else {
                new_row.push(true);
            }
        }
        new_rolls.push(new_row);
    }
    (removable, new_rolls)
}
