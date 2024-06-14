pub mod scheduler_data_generator {
    use rand_distr::Uniform;

    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    use crate::cpu_scheduler;
    use crate::cpu_scheduler::scheduler::{Cpu, Process};
    use crate::DEBUG;

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
        if range_start == range_end {
            vec![range_start; n]
        } else {
            let mut numbers: Vec<u32> = Uniform::new(range_start, range_end)
                .sample_iter(&mut rng)
                .take(n)
                .collect();
            numbers.sort();
            numbers
        }
    }

    fn generic_test_data() -> Vec<Process> {
        vec![
            Process {
                pid: 1,
                arrival: 0,
                burst: 4,
            },
            Process {
                pid: 2,
                arrival: 1,
                burst: 3,
            },
            Process {
                pid: 3,
                arrival: 2,
                burst: 1,
            },
            Process {
                pid: 4,
                arrival: 3,
                burst: 2,
            },
            Process {
                pid: 5,
                arrival: 4,
                burst: 5,
            },
        ]
    }

    #[derive(Debug)]
    pub struct OutputProcessEntry {
        pid: u32,
        arrival: u32,
        burst: u32,
        turnaround: u32,
        waiting: u32,
    }

    pub fn parse_test_data(processes: &[Process]) -> String {
        let mut result = String::new();
        result.push_str("PID;Arrival;Burst\n");
        for process in processes.iter() {
            result.push_str(&format!(
                "{};{};{}\n",
                process.pid, process.arrival, process.burst
            ));
        }
        result
    }

    // type TraitSpecificFunction = fn(&mut Cpu, Option<Process>, u32) -> (u32, Option<u32>);

    pub struct Feeder {
        pub processes: Vec<Process>,
        functions: Vec<Box<dyn Cpu>>,
    }

    impl Default for Feeder {
        fn default() -> Self {
            let processes = generic_test_data();
            Feeder {
                processes,
                functions: Vec::new(),
            }
        }
    }

    impl From<Vec<Process>> for Feeder {
        fn from(processes: Vec<Process>) -> Self {
            Feeder {
                processes,
                functions: Vec::new(),
            }
        }
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

        /// Import the JSON file and deserialize it into array of Processes
        /// Then load it into new Feeder object
        ///
        /// # Arguments
        /// * `filename` - A string containing the JSON filename or path
        ///
        /// # Returns
        /// * A Feeder object with the processes loaded from the JSON file
        pub fn import_from_file(filename: String) -> Feeder {
            let json_string = std::fs::read_to_string(filename);
            let json_string = match json_string {
                Ok(json_string) => json_string,
                Err(e) => {
                    panic!("Error reading file: {}", e);
                }
            };
            Feeder::from_deserialized_processes(json_string)
        }

        /// Deserialize the JSON-standard String into a Vec<Process>
        /// And load it into new Feeder object
        ///
        /// # Arguments
        /// * `json` - A string containing the JSON
        ///
        /// # Returns
        /// * A Feeder object with the processes loaded from the JSON string
        pub fn from_deserialized_processes(json: String) -> Feeder {
            let processes: Vec<Process> = serde_json::from_str(&json).unwrap();
            Feeder {
                processes,
                functions: Vec::new(),
            }
        }

        /// Serialize the processes into a JSON-standard String
        ///
        /// # Returns
        /// * A string containing the JSON
        pub fn to_serialized_processes(&self) -> String {
            serde_json::to_string(&self.processes).unwrap()
        }

        /// Export the processes into a JSON file
        ///
        /// # Arguments
        /// * `filename` - A string containing the JSON filename or path
        ///
        /// # Returns
        /// * None - Everything is written to file successfully and function exits, otherwise it panics
        pub fn export_to_file(&self, filename: String) {
            let json_string = self.to_serialized_processes();
            let result = std::fs::write(filename, json_string);
            match result {
                Ok(_) => {
                    println!("File saved successfully");
                }
                Err(e) => {
                    panic!("Error writing file: {}", e);
                }
            }
        }

        pub fn add_function(&mut self, f: Box<dyn Cpu>) {
            self.functions.push(f);
        }

        fn parse_output(output: Vec<OutputProcessEntry>) -> String {
            let mut output = output;
            // Sort by PID, for algorithms other than FCFS
            output.sort_by(|a, b| a.pid.cmp(&b.pid));
            let mut result = String::new();
            result.push_str("PID;Arrival;Burst;Turnaround;Waiting\n");
            let avg_turnaround =
                output.iter().map(|x| x.turnaround).sum::<u32>() as f64 / output.len() as f64;
            let avg_waiting =
                output.iter().map(|x| x.waiting).sum::<u32>() as f64 / output.len() as f64;
            for entry in output {
                result.push_str(&format!(
                    "{};{};{};{};{}\n",
                    entry.pid, entry.arrival, entry.burst, entry.turnaround, entry.waiting
                ));
            }
            result.push_str(&format!(
                "Average;--;--;{};{}\n",
                avg_turnaround, avg_waiting
            ));
            result
        }

        pub fn feed(&mut self) -> Vec<String> {
            let mut outputs = Vec::new();
            for cpu in self.functions.iter_mut() {
                let mut timer = 0; // Reset timer for each Algorithm
                let mut arrivals = self.processes.clone();
                let mut output: Vec<OutputProcessEntry> = Vec::new();
                if DEBUG {
                    println!("{}", cpu_scheduler::scheduler::process_table_header());
                }
                let mut current_pid;
                loop {
                    // let mut arrival = arrivals.first().cloned();
                    if arrivals.is_empty() && cpu.get_stack().is_empty() {
                        break;
                    }
                    let arrivals_now = arrivals
                        .iter()
                        .filter(|x| x.arrival == timer)
                        .map(|x| x.to_owned())
                        .collect(); // Gather all processes that have arrived
                    arrivals.retain(|x| x.arrival != timer); // Remove all processes that have arrived
                    (timer, current_pid) = cpu.next_loop(arrivals_now, timer);
                    if DEBUG {
                        println!(
                            "{}",
                            cpu_scheduler::scheduler::process_table(cpu.get_stack(), &(&timer - 1))
                                .join("\n")
                        );
                    }
                    if let Some(pid) = current_pid {
                        let process = self.processes.iter().find(|&x| x.pid == pid).cloned();
                        let turnaround = if let Some(process) = process {
                            (timer - 2) - process.arrival
                        } else {
                            0
                        };
                        let waiting = if let Some(process) = process {
                            turnaround - process.burst
                        } else {
                            0
                        };
                        output.push(OutputProcessEntry {
                            pid,
                            arrival: process.unwrap().arrival,
                            burst: process.unwrap().burst,
                            turnaround,
                            waiting,
                        });
                    }
                }
                outputs.push(Feeder::parse_output(output));
            }
            outputs
        }
    }
}
