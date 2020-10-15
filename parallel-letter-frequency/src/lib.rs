use std::collections::HashMap;
use std::sync::mpsc;

// Provides scoped threads.
use crossbeam::thread;

/// Measures letter frequencies in the given `input` potentially using a parallel
/// algorithm producing `worker_count` pods.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Sequential version when too little; empirical value: see additional benches.
    if input.len() < 60 {
        seq_freqs(input)
    } else {
        let mut map = HashMap::new();
        let (tx, rx) = mpsc::channel();

        // Limit borrow scope to loop spawning threads.
        thread::scope(|s| {
            // Making chunks of lines shoud be the fastest mapping possible.
            // It does not guarantee balanced work loads between workers though.
            for chunk in input.chunks(input.len() / input.len().min(worker_count)) {
                let tx = tx.clone();
                s.spawn(move |_| {
                    tx.send(seq_freqs(chunk)).unwrap();
                });
            }
        })
        .unwrap(); // Threads are automatically joined.

        // Reduce results into one map.
        for chunk_result in rx.try_iter() {
            for (chr, count) in chunk_result.into_iter() {
                *map.entry(chr).or_insert(0) += count;
            }
        }

        map
    }
}

/// Sequentially measures letter frequencies in the given `input`.
fn seq_freqs(input: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    // Using `filter_map` is equivalent to `if let`, but both are actually
    // more efficient than lowercasing the entire line before filtering.
    for line in input {
        for chr in line
            .chars()
            .filter(|chr| chr.is_alphabetic())
            .filter_map(|chr| chr.to_lowercase().next())
        {
            *map.entry(chr).or_insert(0) += 1;
        }
    }

    map
}
