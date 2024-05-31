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
            if self.page_size != self.queue.len() && !self.queue.contains(&Some(page)) {
                self.queue.push(Some(page));
                    self.frequency.insert(page, 1);
                    true
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
