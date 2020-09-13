#![feature(test)]
extern crate test;

use std::time::{Duration, UNIX_EPOCH};
use test::Bencher;

use webe_id::*;

#[bench]
fn bench_next(b: &mut Bencher) {
    // set up factory
    let epoch = UNIX_EPOCH
        .checked_add(Duration::from_millis(1546300800000)) // 01-01-2019 12:00:00 AM GMT
        .expect("failed to create custom epoch");
    let mut factory = WebeIDFactory::new(epoch, 0u8).unwrap();

    b.iter(|| {
        factory.next()
    });
}