// test bench_large_parallel   ... bench:     298,331 ns/iter (+/- 13,797)
// test bench_large_sequential ... bench:     405,630 ns/iter (+/- 6,580)
// test bench_small_parallel   ... bench:      58,425 ns/iter (+/- 3,037)
// test bench_small_sequential ... bench:       8,694 ns/iter (+/- 132)
// test bench_tiny_parallel    ... bench:      52,346 ns/iter (+/- 1,630)
// test bench_tiny_sequential  ... bench:          39 ns/iter (+/- 0)

use futures::executor::block_on;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
	block_on(freq(input, worker_count))
}

async fn freq(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
	let mut pool = FuturesUnordered::new();
	for s in input {
		pool.push(async {
			let mut freqmap = HashMap::new();
			for c in s.chars().filter(|c| c.is_alphabetic()) {
				let c = c.to_ascii_lowercase();
				*freqmap.entry(c).or_insert(0) += 1;
			}
			return freqmap;
		})
	}

	let mut freqmap = HashMap::new();
	while let Some(res) = pool.next().await {
		for (k, v) in res {
			*freqmap.entry(k).or_insert(0) += v;
		}
	}
	return freqmap;
}
