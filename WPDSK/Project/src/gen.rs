pub mod data_generator {
    use std::i64;

    use crate::cpu::object::Cpu;
    use rand::prelude::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    pub fn generate_duration_times(n: usize, avg: f64, std_dev: f64) -> Vec<u32> {
        let mut rng = thread_rng();
        let normal = Normal::new(avg, std_dev).expect("Invalid parameters");
        let mut data = Vec::new();
        for _ in 0..n {
            data.push(normal.sample(&mut rng));
        }
        data.into_iter().map(|x| x as u32).collect()
    }

    pub fn generate_arrival_times(n: usize, range_start: u32, range_end: u32) -> Vec<u32> {
        if range_start > range_end {
            panic!("range_start must be less than range_end");
        }
        let mut rng = thread_rng();
        let mut numbers: Vec<u32> = (range_start..range_end).collect();
        numbers.shuffle(&mut rng);
        numbers.into_iter().take(n).collect()
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Process {
        pub pid: u32,
        pub arrival: u32,
        pub burst: u32,
    }

    pub struct Feeder {
        processes: Vec<Process>,
        functions: Vec<Cpu>,
    }

    impl Feeder {
        pub fn new(
            n: usize,
            arrival_range_start: u32,
            arrival_range_end: u32,
            duration_avg: f64,
            duration_std_dev: f64,
        ) -> Feeder {
            let arrival_times = generate_arrival_times(n, arrival_range_start, arrival_range_end);
            let duration_times = generate_duration_times(n, duration_avg, duration_std_dev);
            let mut processes = arrival_times
                .into_iter()
                .zip(duration_times.into_iter())
                .collect::<Vec<(u32, u32)>>();
            processes.sort_by(|a, b| a.0.cmp(&b.0));
            let processes: Vec<Process> = processes
                .into_iter()
                .enumerate()
                .map(|(i, (arrival_time, duration_time))| Process {
                    pid: i as u32,
                    arrival: arrival_time,
                    burst: duration_time,
                })
                .collect();
            Feeder {
                processes,
                functions: Vec::new(),
            }
        }

        /* pub fn add_function(&mut self, f: fn(&Process, &mut i64) -> i64) {
            self.functions.push(f);
        } */

        // pub fn feed(&self) {
        //     for function in self.functions.iter() {
        //         let tmp_processes = self.processes.clone(); // Create a copy of processes array for
        //                                                     // CPU simulator to manage freely
        //         let mut time = 0;
        //         // Main program loop
        //         loop {
        //             unimplemented!("Feed")
        //             //(function)(tmp_processes.first().unwrap(), &mut time);
        //         }
        //     }
        // }
    }
}
