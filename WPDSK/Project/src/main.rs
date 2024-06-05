use cpu_pager::paging::{FirstInFirstOut, LeastFrequentlyUsed};
use cpu_scheduler::scheduler::{FirstComeFirstServe, RoundRobin};
use scheduler_gen::scheduler_data_generator::Feeder;
use std::fs;
mod cpu_pager;
mod cpu_scheduler;
mod custom_gen;
mod pager_gen;
mod scheduler_gen;

static DEBUG: bool = false;
static GENERATE_NEW_DATA: bool = true;

fn main() {
    let feeders: Vec<Feeder>;
    if GENERATE_NEW_DATA {
        feeders = gen_scheduler_data();
        export_scheduler_data(&feeders);
    } else {
        feeders = import_scheduler_data("./tests/scheduler");
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

    // let pages = gen_paging_data();
    // let mut feeders = Vec::new();
    // for page_set in pages {
    //     let feeder = pager_gen::paging_data_generator::Feeder::new(page_set);
    //     feeders.push(feeder);
    // }
    // export_paging_data(&feeders);
    let feeders = import_paging_data("./tests/paging");
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
    vec![
        Feeder::new(100, 0, 100, 5.0, 2.0), // Different arrival times, same burst times
        Feeder::new(100, 0, 0, 5.0, 4.0),   // Same (0) arrival times, different burst times
        Feeder::new(100, 0, 100, 5.0, 4.0), // Different arrival times, different low burst times
        Feeder::new(100, 0, 100, 20.0, 5.0), // Different arrival times, different high burst times (low differences in burst times)
        Feeder::from(custom_gen::low_burst_with_spikes(100)), // Low burst times with spikes
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
    let mut feeders = Vec::new();
    for file_name in find_files(test_dir) {
        let feeder = Feeder::import_from_file(file_name);
        feeders.push(feeder);
    }
    feeders
}

fn export_scheduler_data(feeders: &Vec<scheduler_gen::scheduler_data_generator::Feeder>) {
    let mut i = 0;
    for feeder in feeders {
        feeder.export_to_file(format!("test_data_scheduler_{i:02}.json").to_string());
        i += 1;
    }
}

fn export_scheduler_outputs(outputs: &Vec<Vec<String>>) {
    let mut i = 0;
    for output in outputs {
        let mut output_str = String::new();
        for line in output {
            output_str.push_str(line);
            output_str.push_str("\n");
        }
        fs::write(
            format!("output_scheduler_{i:02}.csv").to_string(),
            output_str,
        )
        .unwrap();
        i += 1;
    }
}

fn export_paging_data(feeders: &Vec<pager_gen::paging_data_generator::Feeder>) {
    let mut i = 0;
    for feeder in feeders {
        feeder.export_to_file(format!("test_data_paging_{i:02}.json").to_string());
        i += 1;
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
