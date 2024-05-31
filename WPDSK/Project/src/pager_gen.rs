pub mod paging_data_generator {
    use std::any::Any;

    // use log::{debug, info, warn};
    use rand::prelude::*;
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    use crate::cpu_pager::paging::PagingAlgorithm;

    pub fn generate_page_numbers(n: usize, avg: f64, std_dev: f64) -> Vec<u32> {
        let mut rng = thread_rng();
        let normal = Normal::new(avg, std_dev).expect("Invalid parameters");
        let mut data = Vec::new();
        for _ in 0..n {
            data.push(normal.sample(&mut rng));
        }
        data.into_iter().map(|x| x as u32).collect()
    }

    pub struct Feeder {
        pub pages: Vec<u32>,
        pub functions: Vec<Box<dyn PagingAlgorithm>>,
    }

    fn generic_test_data() -> Vec<u32> {
        vec![1, 2, 3, 2, 4, 2, 1, 3, 2, 1]
    }

    impl Default for Feeder {
        fn default() -> Self {
            Feeder {
                pages: generic_test_data(),
                functions: Vec::new(),
            }
        }
    }

    impl Feeder {
        pub fn new(pages: Vec<u32>) -> Feeder {
            Feeder {
                pages,
                functions: Vec::new(),
            }
        }

        pub fn add_function(&mut self, function: Box<dyn PagingAlgorithm>) {
            self.functions.push(function);
        }

        pub fn feed(&mut self) {
            for function in self.functions.iter_mut() {
                let mut total_page_faults = 0;
                for page in self.pages.iter() {
                    if function.page_in(*page) {
                        total_page_faults += 1;
                    }
                    // println!("Page: {}, State: {:?}", page, function);
                }
                println!("Total page faults: {}", total_page_faults);
            }
        }
    }
}
