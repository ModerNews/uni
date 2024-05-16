use cpu::algos::{FirstComeFirstServe, RoundRobin};
use cpu::object::Cpu;
use gen::data_generator::Process;

mod cpu;
mod gen;

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

fn main() {
    let mut cpu = Cpu::new();
    // let round_robin: &dyn RoundRobin = &cpu;
    let mut timer = 0;
    let data = generic_test_data();
    let mut arrivals = data.clone();
    let mut _output: Vec<OutputProcessEntry> = Vec::new();
    Cpu::process_table_header();
    loop {
        let mut _pid = None;
        let arrival = arrivals.first().cloned();
        if arrival.is_none() && cpu.stack.is_empty() {
            break;
        }
        if let Some(process) = arrival {
            if process.arrival == timer {
                arrivals.remove(0);
            }
        }
        let first_come_first_serve: &mut dyn FirstComeFirstServe = &mut cpu;
        (timer, _pid) = first_come_first_serve.next_loop(arrival, timer);
    }
}
