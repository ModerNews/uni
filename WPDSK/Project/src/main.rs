use cpu_pager::paging::{FirstInFirstOut, LeastFrequentlyUsed};
use cpu_scheduler::scheduler::{FirstComeFirstServe, RoundRobin};
// use scheduler_gen::scheduler_data_generator::Feeder;
use std::fs;
mod cpu_pager;
mod cpu_scheduler;
mod custom_gen;
mod pager_gen;
mod scheduler_gen;

static DEBUG: bool = false;
static GENERATE_NEW_DATA: bool = false;
static LOAD_EXISTING_DATA: bool = true;

fn main() {
    test_main();
    let mut data = vec![
        "123", "234", "345", "456", "567", "678", "789", "890", "901", "012",
    ];
    let bind = data.iter().map(|&x| x.chars().rev().collect::<String>()).collect::<Vec<String>>();
    let output = bind.join("\n");
    println!("{}", output);
}

fn test_main() {
    let mut feeders: Vec<scheduler_gen::scheduler_data_generator::Feeder> = Vec::new();
    if GENERATE_NEW_DATA {
        feeders.append(&mut gen_scheduler_data());
        export_scheduler_data(&feeders);
    }
    if LOAD_EXISTING_DATA {
        feeders.append(&mut import_scheduler_data("./tests/scheduler"));
    }
    let mut outputs = Vec::new();
    for feeder in feeders {
        println!("=========================================");
        println!("======= CPU scheduling algorithms =======");
        println!("Executing test cases with following data:");
        println!(
            "Test data:\n{}",
            scheduler_gen::scheduler_data_generator::parse_test_data(&feeder.processes)
        );
        outputs.push(execute_scheduler_feeder(feeder));
        export_scheduler_outputs(&outputs);
        println!("=========================================");
    }

    let mut feeders: Vec<pager_gen::paging_data_generator::Feeder> = Vec::new();
    if GENERATE_NEW_DATA {
        let pages = gen_paging_data();
        for page_set in pages {
            let feeder = pager_gen::paging_data_generator::Feeder::new(page_set);
            feeders.push(feeder);
        }

        export_paging_data(&feeders);
    }
    if LOAD_EXISTING_DATA {
        feeders.append(&mut import_paging_data("./tests/paging"));
    }
    for feeder in feeders {
        println!("=========================================");
        println!("====== Page replacement algorithms ======");
        println!("Executing test cases with following data:");
        println!("{:?}", &feeder.pages);
        execute_paging_feeder(feeder);
    }
}

fn execute_scheduler_feeder(
    mut feeder: scheduler_gen::scheduler_data_generator::Feeder,
) -> Vec<String> {
    // let mut feeder = Feeder::new(5, 0, 10, 5.0, 1.0);
    println!("Algorihms: FirstComeFirstServe, RoundRobin(2), RoundRobin(5)");
    feeder.add_function(Box::new(FirstComeFirstServe::new()));
    feeder.add_function(Box::new(RoundRobin::new(2)));
    feeder.add_function(Box::new(RoundRobin::new(5)));
    feeder.feed()
}

fn gen_scheduler_data() -> Vec<scheduler_gen::scheduler_data_generator::Feeder> {
    use scheduler_gen::scheduler_data_generator::Feeder;
    vec![
        Feeder::new(100, 0, 100, 5.0, 0.0), // Different arrival times, same burst times
        Feeder::new(100, 0, 0, 5.0, 4.0),   // Same (0) arrival times, different burst times
        Feeder::new(100, 0, 100, 5.0, 4.0), // Different arrival times, different low burst times
        Feeder::new(100, 0, 100, 20.0, 5.0), // Different arrival times, different high burst times (low differences in burst times)
        Feeder::from(custom_gen::low_burst_with_spikes(100)), // Low burst times with spikes (should show the starving problem in FCFS)
        Feeder::from(custom_gen::high_burst_first_then_low(100)), // Single high burst time, rest low burst times (should show the starving problem in FCFS)
    ]
}

fn find_files(test_dir: &str) -> Vec<String> {
    let paths = fs::read_dir(test_dir).unwrap();
    let mut files = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        files.push(path);
    }
    files
}

fn import_scheduler_data(test_dir: &str) -> Vec<scheduler_gen::scheduler_data_generator::Feeder> {
    use scheduler_gen::scheduler_data_generator::Feeder;
    let mut feeders = Vec::new();
    for file_name in find_files(test_dir) {
        let feeder = Feeder::import_from_file(file_name);
        feeders.push(feeder);
    }
    feeders
}

fn export_scheduler_data(feeders: &[scheduler_gen::scheduler_data_generator::Feeder]) {
    for (i, feeder) in feeders.iter().enumerate() {
        feeder.export_to_file(format!("test_data_scheduler_{i:02}.json").to_string());
    }
}

fn export_scheduler_outputs(outputs: &[Vec<String>]) {
    for (i, output) in outputs.iter().enumerate() {
        let mut output_str = String::new();
        for line in output {
            output_str.push_str(line);
            output_str.push('\n');
        }
        fs::write(format!("output_scheduler_{i:02}.csv"), output_str).unwrap();
    }
}

fn export_paging_data(feeders: &[pager_gen::paging_data_generator::Feeder]) {
    for (i, feeder) in feeders.iter().enumerate() {
        feeder.export_to_file(format!("test_data_paging_{i:02}.json").to_string());
    }
}

fn import_paging_data(test_dir: &str) -> Vec<pager_gen::paging_data_generator::Feeder> {
    let mut feeders = Vec::new();
    for file_name in find_files(test_dir) {
        let feeder = pager_gen::paging_data_generator::Feeder::import_from_file(file_name);
        feeders.push(feeder);
    }
    feeders
}

fn gen_paging_data() -> Vec<Vec<u32>> {
    use pager_gen::paging_data_generator::generate_page_numbers;
    let mut extended_sequence = generate_page_numbers(100, 3.0, 2.0);
    extended_sequence.extend(custom_gen::repeating_pages_sequence(&[1, 2, 3, 4, 5], 400));
    vec![
        custom_gen::frequent_page(500, 200, 10.0, 3.0), // One page is repeated often
        custom_gen::belady_anomaly(50),                 // Known case of Belady's Anomaly, extended
        custom_gen::repeating_pages_sequence(&[1, 2, 3, 4, 5], 500), // Repeating sequence
        extended_sequence, // Repeating sequence prepped with random numbers (to check for LFU recovery time)
        generate_page_numbers(500, 3.0, 2.0), // low amount of duplicates, completly random
        generate_page_numbers(500, 10.0, 5.0), // high amount of duplicates, completly random
    ]
}

fn execute_paging_feeder(mut feeder: pager_gen::paging_data_generator::Feeder) {
    // Test with different page sizes to check for Belady's Anomaly
    println!("Algorithms: FirstInFirstOut(n), LeastFrequentlyUsed(n), where n is in range 2 to 5");
    for n in 2..=5 {
        feeder.add_function(Box::new(FirstInFirstOut::new(n)));
        feeder.add_function(Box::new(LeastFrequentlyUsed::new(n)));
    }
    feeder.feed();
    println!("=========================================");
}
