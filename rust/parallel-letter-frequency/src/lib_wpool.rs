// slightly faster for large, much slower for small and tiny
// test bench_large_parallel   ... bench:     362,546 ns/iter (+/- 32,911)
// test bench_large_sequential ... bench:     395,761 ns/iter (+/- 8,465)
// test bench_small_parallel   ... bench:      55,528 ns/iter (+/- 3,068)
// test bench_small_sequential ... bench:      13,556 ns/iter (+/- 465)
// test bench_tiny_parallel    ... bench:      50,084 ns/iter (+/- 1,912)
// test bench_tiny_sequential  ... bench:          43 ns/iter (+/- 0)

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
	let mut pool = (
		Vec::with_capacity(worker_count),
		Vec::with_capacity(worker_count),
	);
	// array instead of hashmap because it's faster
	// panics if input is more than 8 bit ASCII
	let (res_tx, res_rx) = mpsc::channel::<[usize; 255]>();

	for _ in 0..worker_count {
		let (tx, rx) = mpsc::channel::<&str>();
		let res_tx = res_tx.clone();

		let handle = thread::spawn(move || {
			while let Ok(job) = rx.recv() {
				let mut res = [0; 255];
				for c in job.chars().filter(|c| c.is_alphabetic()) {
					let c = c.to_ascii_lowercase();
					res[c as usize] += 1;
				}
				res_tx.send(res).unwrap();
			}
		});

		pool.0.push(handle);
		pool.1.push(tx);
	}

	for (i, s) in input.iter().enumerate() {
		pool.1[i % worker_count].send(s).unwrap();
	}
	for tx in pool.1 {
		drop(tx);
	}
	thread::spawn(move || {
		for handle in pool.0 {
			handle.join().unwrap();
		}
		drop(res_tx);
	});

	let mut freqmap = HashMap::new();
	while let Ok(res) = res_rx.recv() {
		for (i, c) in res.iter().enumerate().filter(|(_, c)| **c != 0) {
			*freqmap.entry(i as u8 as char).or_insert(0) += c;
		}
	}

	return freqmap;
}
