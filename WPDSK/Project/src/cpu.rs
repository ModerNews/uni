pub mod object {
    #[derive(Copy, Clone, Debug)]
    pub struct Process {
        pub pid: u32,
        pub arrival: u32,
        pub burst: u32,
    }

    pub struct Cpu {
        pub stack: Vec<Process>,
    }

    impl Cpu {
        pub fn new() -> Cpu {
            Cpu { stack: Vec::new() }
        }
        pub fn process_table_header() -> String {
            "Time;PID;Arrival;Stack;Remaining Burst".to_string()
        }

        pub fn process_table(&self, time: &u32) -> Vec<String> {
            self.stack
                .clone()
                .into_iter()
                .map(|process| {
                    format!(
                        "{};{};{};{};{}",
                        time,
                        process.pid,
                        process.arrival,
                        self.stack
                            .clone()
                            .into_iter()
                            .map(|x| x.pid.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                        process.burst
                    )
                })
                .collect()
        }
    }
}

pub mod algos {
    use super::object::{Cpu, Process};

    pub trait RoundRobin {
        fn next_loop(
            &mut self,
            arrival: Option<Process>,
            timer: u32,
            quantum_time: u32,
        ) -> (u32, Option<u32>);
    }

    impl RoundRobin for Cpu {
        fn next_loop(
            &mut self,
            arrival: Option<Process>,
            timer: u32,
            quantum_time: u32,
        ) -> (u32, Option<u32>) {
            let mut quantum_time = quantum_time;
            let mut pid = None;
            if let Some(process) = self.stack.last() {
                // Check if the process was done in the previous loop
                // remove process as first step instead of last for logging purposes
                if process.burst == 0 {
                    self.stack.pop();
                }
            }
            if let Some(process) = self.stack.first_mut() {
                // Simulate processing of the process
                pid = Some(process.pid);
                if process.burst > 0 {
                    // This if block prevents the process from going into negative burst
                    // (and CPU clock from idling for a remainder of the quantum time)
                    if process.burst > quantum_time {
                        process.burst -= quantum_time;
                    } else {
                        quantum_time = process.burst;
                        process.burst = 0;
                    }
                }

                // Move the process to the end of the stack
                // Put freshly arrived process in stack, before the currently processed one
                if let Some(process) = arrival {
                    self.stack.push(process);
                }
                let process = self.stack.remove(0);
                self.stack.push(process);
            } else if let Some(process) = arrival {
                self.stack.push(process);
            }
            (timer + quantum_time, pid)
        }
    }

    pub trait FirstComeFirstServe {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>);
    }

    impl FirstComeFirstServe for Cpu {
        /// First Come First Serve algorithm
        ///
        /// # Arguments
        /// * `arrival` - Option<Process> - Process to be added to the stack
        /// * `timer` - u32 - Current timer state
        ///
        /// # Returns
        /// * (new_timer, Option<pid>) - (u32, Option<u32>) - New timer state and PID of the process that was processed
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>) {
            // Check if the process was done in the previous loop
            // remove process as first step instead of last for logging purposes
            if let Some(process) = self.stack.first_mut() {
                if process.burst == 0 {
                    self.stack.remove(0);
                }
            }
            let mut pid = None;

            // reborrow the first element in case previous if-let block removed it
            if let Some(process) = self.stack.first_mut() {
                pid = Some(process.pid);
                if process.burst > 0 {
                    process.burst -= 1;
                }
                // process.turnaround = timer - process.arrival;
            }
            if let Some(process) = arrival {
                self.stack.push(process);
            }
            (timer + 1, pid)
        }
    }
}
