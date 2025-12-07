use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let input = read_to_string("../2.txt").unwrap();
    let ranges: Vec<_> = input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .collect();

    println!(
        "{}",
        ranges
            .iter()
            .flat_map(|(start, end)| Repeats::new(start, end, 2))
            .sum::<usize>()
    );

    println!(
        "{}",
        ranges
            .iter()
            .flat_map(|(start, end)| (2..=end.len()).flat_map(|i| Repeats::new(start, end, i)))
            .collect::<HashSet<_>>()
            .into_iter()
            .sum::<usize>()
    );
}

struct Repeats {
    range: RangeInclusive<usize>,
    parts: usize,
}

impl Repeats {
    #[allow(clippy::reversed_empty_ranges)]
    fn new(start: &str, end: &str, parts: usize) -> Self {
        let start_len = start.len();
        let start_num = if start_len.is_multiple_of(parts) {
            let part_len = start_len / parts;
            let start_first = &start[..part_len];
            let start_int = usize::from_str(start_first).unwrap();
            let start_rest = usize::from_str(&start[part_len..]).unwrap();
            let theoretical = usize::from_str(&start_first.repeat(parts - 1)).unwrap();
            if theoretical >= start_rest {
                start_int
            } else {
                start_int + 1
            }
        } else {
            usize::from_str(&("1".to_string() + &"0".repeat(start_len / parts))).unwrap()
        };
        let end_len = end.len();
        let end_num = if end_len.is_multiple_of(parts) {
            let part_len = end_len / parts;
            let end_first = &end[..part_len];
            let end_int = usize::from_str(end_first).unwrap();
            let end_rest = usize::from_str(&end[part_len..]).unwrap();
            let theoretical = usize::from_str(&end_first.repeat(parts - 1)).unwrap();
            if theoretical <= end_rest {
                end_int
            } else {
                end_int - 1
            }
        } else {
            if end_len / parts == 0 {
                return Repeats {
                    range: 1..=0, // an intentionally empty range to iterate over
                    parts,
                };
            }
            usize::from_str(&"9".repeat(end_len / parts)).unwrap()
        };
        Repeats {
            range: start_num..=end_num,
            parts,
        }
    }
}

impl Iterator for Repeats {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(usize::from_str(&self.range.next()?.to_string().repeat(self.parts)).unwrap())
    }
}
