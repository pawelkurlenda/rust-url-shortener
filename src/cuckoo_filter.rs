use std::collections::hash_map::RandomState as BuildHasher;
use std::hash::DefaultHasher;
use std::sync::{Arc, RwLock};

pub struct CuckooFilter<const B: usize> {
    hasher: BuildHasher,
    array_size: u64,
    bucket_1: Vec<usize>,
    bucket_2: Vec<usize>,
    build: BuildHasher,
    //buckets: RwLock<Vec<[u16]>>,
    buckets: RwLock<Vec<[u16; B]>>,
}

impl<const B: usize> CuckooFilter<B> {
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "size must be power of two");

        let buckets = vec![[0u16; B]; size];

        Self {
            hasher: BuildHasher::default(),
            build: BuildHasher::default(),
            array_size: size,
            bucket_1: vec![0; size as usize],
            bucket_2: vec![0; size as usize],
        }
    }
}
