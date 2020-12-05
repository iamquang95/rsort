use rand::distributions::{Distribution, Uniform};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::{cell::RefCell, ops::DerefMut};

pub struct DataGeneratorConfig {
    length: usize,
    range: (usize, usize),
    seed: Option<u64>,
    seed_arr: Option<Vec<u32>>,
}

impl DataGeneratorConfig {
    pub fn new(length: usize, range: (usize, usize), seed: Option<u64>) -> DataGeneratorConfig {
        DataGeneratorConfig {
            length,
            range,
            seed,
            seed_arr: None,
        }
    }

    pub fn new_with_seed_arr(seed_arr: Vec<u32>) -> DataGeneratorConfig {
        DataGeneratorConfig {
            length: seed_arr.len(),
            range: (0, 0),
            seed: None,
            seed_arr: Some(seed_arr),
        }
    }
}

pub struct DataGenerator {
    conf: DataGeneratorConfig,
    rng: RefCell<StdRng>,
}

impl DataGenerator {
    pub fn new(conf: DataGeneratorConfig) -> DataGenerator {
        let rng: RefCell<StdRng> = RefCell::new(match conf.seed {
            Some(s) => SeedableRng::seed_from_u64(s),
            None => {
                let mut thread_rng = rand::thread_rng();
                let new_seed = thread_rng.next_u64();
                SeedableRng::seed_from_u64(new_seed)
            }
        });
        DataGenerator { conf, rng }
    }

    pub fn generate_vec(&self) -> Vec<u32> {
        match &self.conf.seed_arr {
            None => {
                let (left, right) = self.conf.range;
                let data_range = Uniform::from(left..right);
                let res: Vec<u32> = (0..self.conf.length)
                    .map(|_| data_range.sample(self.rng.borrow_mut().deref_mut()) as u32)
                    .collect();
                res
            },
            Some(seed_arr) => {
                seed_arr.clone()
            }
        }
    }
}
