#![feature(test)]

extern crate test;
use test::Bencher;

pub mod workload;

#[bench]
fn as_main(b: &mut Bencher) {
    b.iter(|| {
        let _throw_away = workload::calculate(workload::get_input());
    });
}
