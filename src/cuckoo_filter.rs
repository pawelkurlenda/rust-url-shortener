use std::hash::DefaultHasher;

pub struct CuckooFilter {
    hasher: DefaultHasher,
    array_size: u64,
    bucket_1: Vec<usize>,
    bucket_2: Vec<usize>,
}

impl CuckooFilter {
    pub fn new(size: u64) -> Self {
        Self {
            hasher: DefaultHasher::new(),
            array_size: size,
            bucket_1: vec![0; size as usize],
            bucket_2: vec![0; size as usize],
        }
    }
}
