use cpu::algos::{FirstComeFirstServe, RoundRobin};
use cpu::object::Cpu;
use gen::data_generator::Process;

mod cpu;
mod gen;

#[derive(Debug)]
struct OutputProcessEntry {
    pid: u32,
    arrival: u32,
    burst: u32,
    turnaround: u32,
    waiting: u32,
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

// TODO: Move to feeder
fn main() {
    let mut cpu = Cpu::new();
    let mut timer = 0;
    let data = generic_test_data();
    let mut arrivals = data.clone();
    let mut output: Vec<OutputProcessEntry> = Vec::new();
    println!("{}", Cpu::process_table_header());
    let mut current_pid = None;
    let mut previous_pid = None;
    loop {
        let arrival = arrivals.first().cloned();
        if arrival.is_none() && cpu.stack.is_empty() {
            break;
        }
        if let Some(process) = arrival {
            if process.arrival == timer {
                arrivals.remove(0);
            }
        }
        previous_pid = current_pid;
        (timer, current_pid) = fsfc_next_loop_test(&mut cpu, arrival, timer);
        if current_pid != previous_pid {
            if let Some(pid) = previous_pid {
                let old_process = data.iter().find(|&x| x.pid == pid).cloned();
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
    println!("{}", parse_output(output));
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

fn fsfc_next_loop_test<T: FirstComeFirstServe>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer)
}

fn _rr_next_loop_test<T: RoundRobin>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer)
}
