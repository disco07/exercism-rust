use std::cmp::max;
use std::collections::HashMap;
use std::thread;

pub fn frequency<'a>(inputs: &'a [&'a str], worker_count: usize) -> HashMap<char, usize> {
    let mut threads = vec![];
    let chunk = max(1, inputs.len()/worker_count);
    for chunk in inputs.chunks(chunk) {
        let str = chunk.join("");
        threads.push(thread::spawn(move|| frequency_process(&str)))
    }
    threads
        .into_iter()
        .fold(HashMap::new(), |mut acc, join_handle|{
            if let Ok(x) = join_handle.join() {
                for (k, v) in x {
                    *acc.entry(k).or_insert(0) += v;
                }
            }
            acc
        })
}

fn frequency_process(input: &str) -> HashMap<char, usize> {
    input
        .chars()
        .filter(|ch|ch.is_alphabetic())
        .fold(HashMap::new(), |mut acc, ch|{
            if let Some(ctl) = ch.to_lowercase().next() {
                *acc.entry(ctl).or_insert(0) += 1
            }
            acc
        })
}