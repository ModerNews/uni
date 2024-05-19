pub mod data_generator {
    // use log::{debug, info, warn};
    use rand::prelude::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    use crate::cpu::object::{Cpu, Process};

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

    #[derive(Debug)]
    pub struct OutputProcessEntry {
        pid: u32,
        arrival: u32,
        burst: u32,
        turnaround: u32,
        waiting: u32,
    }

    type TraitSpecificFunction = fn(&mut Cpu, Option<Process>, u32) -> (u32, Option<u32>);

    pub struct Feeder {
        processes: Vec<Process>,
        functions: Vec<TraitSpecificFunction>,
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
                .zip(duration_times)
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

        pub fn add_function(&mut self, f: TraitSpecificFunction) {
            self.functions.push(f);
        }

        fn parse_output(output: Vec<OutputProcessEntry>) -> String {
            let mut result = String::new();
            result.push_str("PID;Arrival;Burst;Turnaround;Waiting\n");
            for entry in output {
                result.push_str(&format!(
                    "{};{};{};{};{}\n",
                    entry.pid, entry.arrival, entry.burst, entry.turnaround, entry.waiting
                ));
            }
            result
        }

        pub fn parse_test_data(&self) -> String {
            let mut result = String::new();
            result.push_str("PID;Arrival;Burst\n");
            for process in self.processes.iter() {
                result.push_str(&format!(
                    "{};{};{}\n",
                    process.pid, process.arrival, process.burst
                ));
            }
            result
        }

        pub fn feed(&self) {
            println!("Test data:\n{}", self.parse_test_data());
            for function in self.functions.iter() {
                println!(
                    "Preparing to test for {:?}",
                    std::any::type_name_of_val(function)
                );
                let mut timer = 0; // Reset timer for each Algorithm
                let mut cpu = Cpu::new(); // Create new CPU instance for each Algorithm to prevent data corruption
                let mut arrivals = self.processes.clone();
                let mut output: Vec<OutputProcessEntry> = Vec::new();
                println!("{}", Cpu::process_table_header());
                let mut current_pid = None;
                let mut previous_pid;
                loop {
                    let mut arrival = arrivals.first().cloned();
                    if arrival.is_none() && cpu.stack.is_empty() {
                        break;
                    }
                    if let Some(process) = arrival {
                        if process.arrival == timer {
                            arrivals.remove(0);
                        } else {
                            arrival = None;
                        }
                    }
                    previous_pid = current_pid;
                    (timer, current_pid) = (function)(&mut cpu, arrival, timer);
                    // FIX: This behavior is correct for implementations that work one process at a time 
                    // However it will break with implementations such as Round Robin
                    if current_pid != previous_pid {
                        if let Some(pid) = previous_pid {
                            let old_process =
                                self.processes.iter().find(|&x| x.pid == pid).cloned();
                            let waiting = if let Some(process) = old_process {
                                // timer - 2 is the last time the process was executed
                                (timer - 2) - process.arrival - process.burst
                            } else {
                                0
                            };
                            let turnaround = if let Some(process) = old_process {
                                // timer - 2 is the last time the process was executed
                                (timer - 2) - process.arrival
                            } else {
                                0
                            };
                            output.push(OutputProcessEntry {
                                pid,
                                arrival: old_process.unwrap().arrival,
                                burst: old_process.unwrap().burst,
                                turnaround,
                                waiting,
                            });
                        }
                    }
                    println!("{}", cpu.process_table(&(&timer - 1)).join("\n"));
                }
                println!("{}", Feeder::parse_output(output));
            }
        }
    }
}
