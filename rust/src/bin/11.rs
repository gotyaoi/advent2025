use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let buffer = read_to_string("../11.txt").unwrap();
    let graph: HashMap<_, _> = buffer
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (parts.0, parts.1.split_whitespace().collect::<Vec<_>>())
        })
        .collect();

    let mut cache = HashMap::new();
    println!("{}", paths(&graph, &mut cache, "you", "out"));

    let svr_to_dac = paths(&graph, &mut cache, "svr", "dac");
    let svr_to_fft = paths(&graph, &mut cache, "svr", "fft");
    let dac_to_fft = paths(&graph, &mut cache, "dac", "fft");
    let fft_to_dac = paths(&graph, &mut cache, "fft", "dac");
    let dac_to_out = paths(&graph, &mut cache, "dac", "out");
    let fft_to_out = paths(&graph, &mut cache, "fft", "out");
    println!(
        "{}",
        svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
    );
}

fn paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, &'a str), usize>,
    start: &'a str,
    end: &'a str,
) -> usize {
    let key = (start, end);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }
    let mut total = 0;
    if let Some(neighbors) = graph.get(&start) {
        for neighbor in neighbors {
            if *neighbor == end {
                total += 1
            } else {
                total += paths(graph, cache, neighbor, end)
            }
        }
    }
    cache.insert(key, total);
    total
}
