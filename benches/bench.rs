#![feature(test)]


extern crate test;

extern crate hash_set;
extern crate collection_traits;


use test::Bencher;


const SIZE: usize = 1024;


#[bench]
fn bench_hash_set(b: &mut Bencher) {
    use hash_set::HashSet;

    b.iter(|| {
        let mut v = HashSet::new();
        for i in 0..SIZE {
            v.insert(i);
        }
        v
    });
}
#[bench]
fn bench_std_hash_set(b: &mut Bencher) {
    use std::collections::HashSet;

    b.iter(|| {
        let mut v = HashSet::new();
        for i in 0..SIZE {
            v.insert(i);
        }
        v
    });
}
