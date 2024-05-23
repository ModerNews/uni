mod cpu;
mod gen;

// use cpu::algos::{FirstComeFirstServe, RoundRobin};
// use cpu::object::Process;
use cpu::paging::{FirstInFirstOut, LeastFrequentlyUsed, PagingAlgorithm};
use cpu::scheduler::{FirstComeFirstServe, RoundRobin};
use gen::scheduler_data_generator::Feeder;

/* fn main() {
    // let mut feeder = Feeder::new(5, 0, 10, 5.0, 1.0);
    let mut feeder = Feeder::default();
    feeder.add_function(Box::new(FirstComeFirstServe::new()));
    feeder.add_function(Box::new(RoundRobin::new(2)));
    feeder.feed();
} */

fn generic_test_data() -> Vec<u32> {
    vec![1, 2, 3, 2, 4, 2, 1, 3, 2, 1]
}

fn main() {
    let mut fifo = FirstInFirstOut::new(3);
    println!("Algorithm: {:?}", fifo);
    let mut total_page_faults = 0;
    for page in generic_test_data() {
        if fifo.page_in(page) {
            total_page_faults += 1;
        }
        println!("Page: {}, State: {:?}", page, fifo)
    }
    println!("Total page faults: {}", total_page_faults);

let mut fifo = LeastFrequentlyUsed::new(3);
    println!("Algorithm: {:?}", fifo);
    let mut total_page_faults = 0;
    for page in generic_test_data() {
        if fifo.page_in(page) {
            total_page_faults += 1;
        }
        println!("Page: {}, State: {:?}", page, fifo)
    }
    println!("Total page faults: {}", total_page_faults);
}

// ############################
//          Test cases
// ############################
// Each algorithm is implemented as a trait for the Cpu object
// However for testing purposes, all traits are imported in main scope simultaneously,
// Therefore, those functions let the compiler differentiate between implementations
/* fn fsfc_next_loop_test<T: FirstComeFirstServe>(
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
} */
