use std::collections::hash_map::RandomState as BuildHasher;
use std::hash::{BuildHasher as _, Hash, Hasher};
use std::sync::{Arc, RwLock};

use rand::Rng;

pub struct CuckooFilter<const B: usize, const F: usize> {
    hasher: BuildHasher,
    array_size: usize,
    bucket_1: Vec<usize>,
    bucket_2: Vec<usize>,
    mask: usize,
    build: BuildHasher,
    max_kicks: usize, // e.g., 500
    //buckets: RwLock<Vec<[u16]>>,
    buckets: RwLock<Vec<[u16; B]>>,
}

impl<const B: usize, const F: usize> CuckooFilter<B, F> {
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "size must be power of two");

        let buckets = vec![[0u16; B]; size];

        Self {
            hasher: BuildHasher::default(),
            build: BuildHasher::default(),
            array_size: size,
            bucket_1: vec![0; size as usize],
            bucket_2: vec![0; size as usize],
            buckets: RwLock::new(buckets),
            mask: size - 1,
            max_kicks: 500,
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let h = self.hash_u64(item);
        let tag = self.fingerprint_u16(h);
        let i1 = (h as usize) & self.mask;
        let i2 = self.alt_index(i1, tag);

        let buckets = self.buckets.read().unwrap();
        buckets[i1].contains(&tag) || buckets[i2].contains(&tag)
    }

    pub fn insert<T: Hash>(&self, item: &T) -> bool {
        let h = self.hash_u64(item);
        let mut tag = self.fingerprint_u16(h);
        let mut i = (h as usize) & self.mask;
        let j = self.alt_index(i, tag);

        // Fast paths: try to place in either bucket.
        {
            let mut buckets = self.buckets.write().unwrap();
            if let Some(slot) = buckets[i].iter_mut().find(|s| **s == 0) {
                *slot = tag;
                return true;
            }
            if let Some(slot) = buckets[j].iter_mut().find(|s| **s == 0) {
                *slot = tag;
                return true;
            }
        }

        // Kick-out loop
        let mut rng = fastrand::Rng::new(); // small, fast RNG; add `fastrand = "2"` to Cargo.toml
        let mut idx = if rng.bool() { i } else { j };

        for _ in 0..self.max_kicks {
            let mut buckets = self.buckets.write().unwrap();
            let slot_idx = rng.usize(..B);
            std::mem::swap(&mut tag, &mut buckets[idx][slot_idx]);
            drop(buckets); // avoid holding lock during reindex

            idx = self.alt_index(idx, tag);

            let mut buckets = self.buckets.write().unwrap();
            if let Some(empty) = buckets[idx].iter_mut().find(|s| **s == 0) {
                *empty = tag;
                return true;
            }
        }
        // Table considered "full" at current load factor.
        false
    }

    pub fn delete<T: Hash>(&self, item: &T) -> bool {
        let h = self.hash_u64(item);
        let tag = self.fingerprint_u16(h);
        let i1 = (h as usize) & self.mask;
        let i2 = self.alt_index(i1, tag);

        let mut buckets = self.buckets.write().unwrap();
        if let Some(slot) = buckets[i1].iter_mut().find(|s| **s == tag) {
            *slot = 0;
            return true;
        }
        if let Some(slot) = buckets[i2].iter_mut().find(|s| **s == tag) {
            *slot = 0;
            return true;
        }
        false
    }

    /// Approximate false positive rate: ~ (2*B)/2^F for small p.
    pub const fn estimate_fpr() -> f64 {
        (2.0 * B as f64) / (2u64.pow(F as u32) as f64)
    }

    #[inline]
    fn hash_u64<T: Hash>(&self, x: &T) -> u64 {
        let mut h = self.build.build_hasher();
        x.hash(&mut h);
        h.finish()
    }

    #[inline]
    fn fingerprint_u16(&self, h: u64) -> u16 {
        // Keep F bits, ensure non-zero (0 means empty)
        let tag = (h & ((1u64 << F) - 1)) as u16;
        if tag == 0 { 1 } else { tag }
    }

    #[inline]
    fn alt_index(&self, i1: usize, tag: u16) -> usize {
        // i2 = i1 XOR hash(tag); mask for table size
        let h = self.hash_u64(&tag);
        (i1 ^ (h as usize)) & self.mask
    }
}
