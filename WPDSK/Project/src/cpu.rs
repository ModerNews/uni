pub mod paging {
    use std::{collections::HashMap, usize};

    pub trait PagingAlgorithm {
        fn page_in(&mut self, page: u32) -> bool;
    }

    #[derive(Debug)]
    pub struct FirstInFirstOut {
        pub queue: Vec<Option<u32>>,
        pub page_size: usize,
    }

    impl FirstInFirstOut {
        pub fn new(memory_size: usize) -> FirstInFirstOut {
            /* FirstInFirstOut {
                queue: vec![None; memory_size].into_boxed_slice(),
            } */
            FirstInFirstOut {
                queue: Vec::new(),
                page_size: memory_size,
            }
        }
    }

    impl PagingAlgorithm for FirstInFirstOut {
        /// Add a new page to the page frame
        /// According to FirstInFirstOut algorithm
        ///
        /// # Arguments
        /// * `page` - u32 - Page number to be added
        ///
        /// # Returns
        /// * bool - True if algorith yielded a Page Fault, False otherwise
        fn page_in(&mut self, page: u32) -> bool {
            if self.page_size != self.queue.len() {
                // There is no additional check for page existence needed, due to FIFO nature
                self.queue.push(Some(page));
                true
            } else if self.queue.contains(&Some(page)) {
                false
            } else {
                self.queue.remove(0);
                self.queue.push(Some(page));
                true
            }
        }
    }

    #[derive(Debug)]
    pub struct LeastFrequentlyUsed {
        pub queue: Vec<Option<u32>>,
        pub frequency: HashMap<u32, u32>,
        pub page_size: usize,
    }

    impl LeastFrequentlyUsed {
        pub fn new(memory_size: usize) -> LeastFrequentlyUsed {
            LeastFrequentlyUsed {
                queue: Vec::new(),
                frequency: HashMap::new(),
                page_size: memory_size,
            }
        }
    }

    impl PagingAlgorithm for LeastFrequentlyUsed {
        fn page_in(&mut self, page: u32) -> bool {
            if self.page_size != self.queue.len() {
                if self.queue.contains(&Some(page)) {
                    // This is only a fail-safe, as the page should not be in the queue, if it is not in the frequency map
                    let freq = self.frequency.entry(page).or_insert(0);
                    *freq += 1;
                    false
                } else {
                    self.queue.push(Some(page));
                    self.frequency.insert(page, 1);
                    true
                }
            } else if self.queue.contains(&Some(page)) {
                // This is only a fail-safe, as the page should not be in the queue, if it is not in the frequency map
                let freq = self.frequency.entry(page).or_insert(0);
                *freq += 1;
                false
            } else {
                let mut min = usize::MAX as u32;
                let mut min_page = 0;
                for (page, freq) in self.frequency.iter() {
                    if *freq < min {
                        min = *freq;
                        min_page = *page;
                    }
                }
                self.queue.retain(|x| x != &Some(min_page));
                self.frequency.remove(&min_page);
                self.queue.push(Some(page));
                self.frequency.insert(page, 1);
                true
            }
        }
    }
}

pub mod scheduler {
    #[derive(Copy, Clone, Debug)]
    pub struct Process {
        pub pid: u32,
        pub arrival: u32,
        pub burst: u32,
    }

    pub fn process_table_header() -> String {
        "Time;PID;Arrival;Stack;Remaining Burst".to_string()
    }

    pub fn process_table(stack: &[Process], time: &u32) -> Vec<String> {
        stack
            .iter()
            .map(|process| {
                format!(
                    "{};{};{};{};{}",
                    time,
                    process.pid,
                    process.arrival,
                    stack
                        .iter()
                        .map(|x| x.pid.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    process.burst
                )
            })
            .collect()
    }

    pub trait Cpu {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>);
        fn get_stack(&self) -> &Vec<Process>;
    }

    pub struct FirstComeFirstServe {
        pub stack: Vec<Process>,
    }

    impl FirstComeFirstServe {
        pub fn new() -> FirstComeFirstServe {
            FirstComeFirstServe { stack: Vec::new() }
        }
    }

    impl Cpu for FirstComeFirstServe {
        /// First Come First Serve algorithm
        ///
        /// # Arguments
        /// * `arrival` - Option<Process> - Process to be added to the stack
        /// * `timer` - u32 - Current timer state
        ///
        /// # Returns
        /// * (new_timer, Option<pid>) - (u32, Option<u32>) - New timer state and PID of the process, if the process was finished (in previous loop)
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>) {
            // Check if the process was done in the previous loop
            // remove process as first step instead of last for logging purposes
            let mut pid = None;
            if let Some(process) = self.stack.first_mut() {
                if process.burst == 0 {
                    let process = self.stack.remove(0);
                    pid = Some(process.pid);
                }
            }

            // reborrow the first element in case previous if-let block removed it
            if let Some(process) = self.stack.first_mut() {
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

        fn get_stack(&self) -> &Vec<Process> {
            &self.stack
        }
    }

    pub struct RoundRobin {
        pub stack: Vec<Process>,
        pub quantum_time: u32,
        pub quantum_timer: u32,
        pub stall_arrival: Option<Process>,
    }

    impl RoundRobin {
        pub fn new(quantum_time: u32) -> RoundRobin {
            RoundRobin {
                stack: Vec::new(),
                quantum_time,
                quantum_timer: 0,
                stall_arrival: None,
            }
        }
    }

    impl Cpu for RoundRobin {
        fn next_loop(&mut self, arrival: Option<Process>, timer: u32) -> (u32, Option<u32>) {
            let mut pid = None;
            let quantum_time = self.quantum_time;
            if let Some(&process) = self.stack.first() {
                // Check if the process was done in the previous loop
                // remove process as first step instead of last for logging purposes

                // if (self.quantum_timer % quantum_time == 0) && self.quantum_timer != 0 {
                if process.burst == 0 {
                    let process = self.stack.remove(0);
                    // Reset timer to prevent it from messing up, when the process is done in under
                    // k * quantum_time, where k is a positive
                    self.quantum_timer = 0;
                    pid = Some(process.pid);
                } else if self.quantum_timer == quantum_time {
                    self.quantum_timer = 0;
                    let process = self.stack.remove(0);
                    self.stack.push(process);
                }
            }
            if let Some(process) = self.stack.first_mut() {
                // Simulate processing of the process
                if process.burst > 0 {
                    process.burst -= 1;
                    self.quantum_timer += 1;
                }
            }
            if let Some(process) = arrival {
                self.stack.push(process);
            }
            (timer + 1, pid)
        }

        fn get_stack(&self) -> &Vec<Process> {
            &self.stack
        }
    }
}
