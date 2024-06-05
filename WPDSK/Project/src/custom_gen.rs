use crate::cpu_scheduler::scheduler::Process;
use rand::thread_rng;
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
