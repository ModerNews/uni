pub mod paging_data_generator {
    use rand::prelude::*;
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

        /// Import the JSON file and deserialize it into array of Processes
        /// Then load it into new Feeder object
        ///
        /// # Arguments
        /// * `filename` - A string containing the JSON filename or path
        ///
        /// # Returns
        /// * A Feeder object with the processes loaded from the JSON file
        pub fn import_from_file(filename: String) -> Feeder {
            let json_string = std::fs::read_to_string(filename);
            let json_string = match json_string {
                Ok(json_string) => json_string,
                Err(e) => {
                    panic!("Error reading file: {}", e);
                }
            };
            Feeder::from_deserialized_pages(json_string)
        }

        /// Deserialize the JSON-standard String into a Vec<Process>
        /// And load it into new Feeder object
        ///
        /// # Arguments
        /// * `json` - A string containing the JSON
        ///
        /// # Returns
        /// * A Feeder object with the processes loaded from the JSON string
        pub fn from_deserialized_pages(json: String) -> Feeder {
            let pages: Vec<u32> = serde_json::from_str(&json).unwrap();
            Feeder {
                pages,
                functions: Vec::new(),
            }
        }

        /// Serialize the processes into a JSON-standard String
        ///
        /// # Returns
        /// * A string containing the JSON
        pub fn to_serialized_pages(&self) -> String {
            serde_json::to_string(&self.pages).unwrap()
        }

        /// Export the processes into a JSON file
        ///
        /// # Arguments
        /// * `filename` - A string containing the JSON filename or path
        ///
        /// # Returns
        /// * None - Everything is written to file successfully and function exits, otherwise it panics
        pub fn export_to_file(&self, filename: String) {
            let json_string = self.to_serialized_pages();
            let result = std::fs::write(filename, json_string);
            match result {
                Ok(_) => {
                    println!("File saved successfully");
                }
                Err(e) => {
                    panic!("Error writing file: {}", e);
                }
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
