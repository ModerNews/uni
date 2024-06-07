use crate::{cpu_scheduler::scheduler::Process, pager_gen};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

pub fn low_burst_with_spikes(n: i32) -> Vec<Process> {
    let mut processes = Vec::new();
    let mut rng = thread_rng();
    let normal = Normal::new(3.0, 2.0).expect("Invalid parameters");
    for i in 0..n {
        if i % 16 == 0 {
            processes.push(Process {
                pid: i as u32,
                arrival: i as u32,
                burst: 25,
            });
        } else {
            processes.push(Process {
                pid: i as u32,
                arrival: i as u32,
                burst: normal.sample(&mut rng) as u32,
            });
        }
    }
    processes
}

pub fn high_burst_first_then_low(n: i32) -> Vec<Process> {
    let mut processes = Vec::new();
    let mut rng = thread_rng();
    let normal = Normal::new(3.0, 2.0).expect("Invalid parameters");
    processes.push(Process {
        pid: 0,
        arrival: 0,
        burst: 100,
    });
    for i in 1..n {
        processes.push(Process {
            pid: i as u32,
            arrival: i as u32,
            burst: normal.sample(&mut rng) as u32,
        });
    }
    processes
}

pub fn belady_anomaly(n: usize) -> Vec<u32> {
    // Known sequnce of page numbers that causes the Belady's anomaly
    // https://en.wikipedia.org/wiki/B%C3%A9l%C3%A1dy%27s_anomaly
    let known_sequence = [3, 2, 1, 0, 3, 2, 4, 3, 2, 1, 0, 4];
    let pages = known_sequence.repeat(n / known_sequence.len());
    pages.to_vec()
}

pub fn repeating_pages_sequence(seq: &[u32], n: usize) -> Vec<u32> {
    let pages = seq.repeat(n / seq.len());
    pages.to_vec()
}

pub fn frequent_page(n: usize, dupes: usize, avg: f64, stdev: f64) -> Vec<u32> {
    let mut pages = pager_gen::paging_data_generator::generate_page_numbers(n - dupes, avg, stdev);
    pages.extend(vec![0; dupes]);
    let mut rng = thread_rng();
    pages.shuffle(&mut rng);
    pages
}
