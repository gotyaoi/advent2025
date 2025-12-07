use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input = read_to_string("../2.txt").unwrap();
    let ranges: Vec<_> = input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|r| {
            let (left, right) = r.split_once('-').unwrap();
            usize::from_str(left).unwrap()..=usize::from_str(right).unwrap()
        })
        .collect();

    let mut total = 0;

    for r in ranges.iter() {
        for i in r.clone() {
            let test = i.to_string();
            let length = test.len();
            if length % 2 != 0 {
                continue;
            }
            let half = length / 2;
            if test[..half] == test[half..] {
                total += i;
            }
        }
    }

    println!("{}", total);

    total = 0;

    for r in ranges {
        for i in r {
            let test = i.to_string();
            let length = test.len();
            for j in 1..=length / 2 {
                if length % j != 0 {
                    continue;
                }
                let v: Vec<_> = test.chars().collect();
                let mut groups = v.chunks_exact(j);
                let first = groups.next().unwrap();
                if groups.all(|g| g == first) {
                    total += i;
                    break;
                }
            }
        }
    }

    println!("{}", total);
}
