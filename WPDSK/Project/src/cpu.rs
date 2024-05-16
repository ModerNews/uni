pub mod object {
    use crate::gen::data_generator::Process;

    pub struct Cpu {
        pub stack: Vec<Process>,
    }

    impl Cpu {
        pub fn new() -> Cpu {
            Cpu { stack: Vec::new() }
        }
        pub fn process_table_header() {
            println!("Time;PID;Arrival;Stack;Remaining Burst");
        }

        pub fn process_table(&self, time: &u32) {
            for process in &self.stack {
                println!(
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
                );
            }
        }
    }
}

pub mod algos {
    use super::object::Cpu;
    use crate::gen::data_generator::Process;

    pub trait RoundRobin {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> u32;
    }

    impl RoundRobin for Cpu {
        fn next_loop(&mut self, _arrival: Option<Process>, _timer: u32) -> u32 {
            unimplemented!("RoundRobin")
        }
    }

    pub trait FirstComeFirstServe {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>);
    }

    impl FirstComeFirstServe for Cpu {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>) {
            let current_process = self.stack.first_mut();
            let mut pid = None;
            if let Some(process) = current_process {
                pid = Some(process.pid);
                if process.burst > 0 {
                    process.burst -= 1;
                }
                if process.burst == 0 {
                    self.stack.remove(0);
                }
                // process.turnaround = timer - process.arrival;
            }
            if let Some(process) = arrival {
                self.stack.push(process);
            }
            self.process_table(&timer);
            (timer + 1, pid) 
        }
    }
}
