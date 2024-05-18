pub mod object {
    use crate::gen::data_generator::Process;

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
    use super::object::Cpu;
    use crate::gen::data_generator::Process;

    pub trait RoundRobin {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>);
    }

    impl RoundRobin for Cpu {
        fn next_loop(&mut self, _arrival: Option<Process>, _timer: u32) -> (u32, Option<u32>) {
            unimplemented!("RoundRobin")
        }
    }

    pub trait FirstComeFirstServe {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>);
    }

    impl FirstComeFirstServe for Cpu {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>) {
            // let current_process = self.stack.first_mut();
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
