use cpu_pager::paging::{FirstInFirstOut, LeastFrequentlyUsed};
use cpu_scheduler::scheduler::{FirstComeFirstServe, RoundRobin};

mod cpu_pager;
mod cpu_scheduler;
mod pager_gen;
mod scheduler_gen;

static DEBUG: bool = false;

fn main() {
    let feeders = gen_scheduler_data();

    for feeder in feeders {
        println!("=========================================");
        println!("======= CPU scheduling algorithms =======");
        println!("Executing test cases with following data:");
        println!(
            "Test data:\n{}",
            scheduler_gen::scheduler_data_generator::parse_test_data(&feeder.processes)
        );
        execute_scheduler_feeder(feeder);
    }

    let pages = gen_paging_data();
    for page in pages {
        println!("=========================================");
        println!("====== Page replacement algorithms ======");
        println!("Executing test cases with following data:");
        println!("{:?}", page);
        let feeder = pager_gen::paging_data_generator::Feeder::new(page);
        execute_paging_feeder(feeder);
    }
}

fn execute_scheduler_feeder(mut feeder: scheduler_gen::scheduler_data_generator::Feeder) {
    // let mut feeder = Feeder::new(5, 0, 10, 5.0, 1.0);
    println!("Algorihms: FirstComeFirstServe, RoundRobin(2), RoundRobin(5)");
    feeder.add_function(Box::new(FirstComeFirstServe::new()));
    feeder.add_function(Box::new(RoundRobin::new(2)));
    feeder.add_function(Box::new(RoundRobin::new(5)));
    feeder.feed();
    println!("=========================================");
}

fn gen_scheduler_data() -> Vec<scheduler_gen::scheduler_data_generator::Feeder> {
    use scheduler_gen::scheduler_data_generator::Feeder;
    vec![
        Feeder::new(25, 0, 100, 5.0, 0.0), // Different arrival times, same burst times
        Feeder::new(50, 0, 100, 5.0, 1.0),
        Feeder::new(100, 0, 100, 5.0, 2.0),
        Feeder::new(25, 0, 0, 5.0, 5.0), // Same arrival times, different burst times
        Feeder::new(50, 0, 0, 5.0, 5.0),
        Feeder::new(100, 0, 0, 5.0, 5.0),
        Feeder::new(25, 0, 100, 5.0, 5.0), // Different arrival times, different burst times
        Feeder::new(50, 0, 100, 5.0, 5.0),
        Feeder::new(100, 0, 100, 5.0, 5.0),
    ]
}

fn gen_paging_data() -> Vec<Vec<u32>> {
    use pager_gen::paging_data_generator::generate_page_numbers;

    vec![
        generate_page_numbers(50, 10.0, 0.0), // only duplicates
        generate_page_numbers(100, 10.0, 0.0),
        generate_page_numbers(50, 10.0, 10.0), // low amount of duplicates
        generate_page_numbers(100, 10.0, 10.0),
        generate_page_numbers(50, 10.0, 3.0), // high amount of duplicates
        generate_page_numbers(100, 10.0, 3.0),
    ]
}

fn execute_paging_feeder(mut feeder: pager_gen::paging_data_generator::Feeder) {
    // Test with different page sizes to check for Belady's Anomaly
    println!("Algorithms: FirstInFirstOut(3), FirstInFirstOut(4), LeastFrequentlyUsed(3), LeastFrequentlyUsed(4)");
    feeder.add_function(Box::new(FirstInFirstOut::new(3)));
    feeder.add_function(Box::new(FirstInFirstOut::new(4)));
    feeder.add_function(Box::new(LeastFrequentlyUsed::new(3)));
    feeder.add_function(Box::new(LeastFrequentlyUsed::new(4)));
    feeder.feed();
    println!("=========================================");
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
