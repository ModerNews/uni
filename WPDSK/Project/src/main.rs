mod cpu;
mod gen;

use cpu::algos::{FirstComeFirstServe, RoundRobin};
use cpu::object::Process;
use gen::data_generator::Feeder;

fn main() {
    // let mut feeder = Feeder::new(5, 0, 10, 5.0, 1.0);
    let mut feeder = Feeder::default();
    feeder.add_function(fsfc_next_loop_test);
    feeder.add_function(rr_next_loop_test_q2);
    feeder.feed();
}
// ############################
//          Test cases
// ############################
// Each algorithm is implemented as a trait for the Cpu object
// However for testing purposes, all traits are imported in main scope simultaneously,
// Therefore, those functions let the compiler differentiate between implementations
fn fsfc_next_loop_test<T: FirstComeFirstServe>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer)
}

fn _rr_next_loop_test_q1<T: RoundRobin>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer, 1)
}

fn rr_next_loop_test_q2<T: RoundRobin>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer, 2)
}

fn _rr_next_loop_test_q5<T: RoundRobin>(
    cpu: &mut T,
    arrival: Option<Process>,
    timer: u32,
) -> (u32, Option<u32>) {
    cpu.next_loop(arrival, timer, 5)
}
