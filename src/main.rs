#![feature(test)]

extern crate test;
mod boob;

use test::Bencher;
fn main() {

}

#[bench]
fn bench_add_two(b: &mut Bencher) {

  b.iter(||{
    unsafe{
      for i in 0..15 {
        let n = test::black_box(1000);
        let b = test::black_box(1000);
        test::black_box(boob::EVAL[((boob::B_T[n as usize]) + (2 * boob::B_T[b as usize])) as usize]);
      }
    }
  });
}