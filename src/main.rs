#![feature(test)]

extern crate test;
use test::Bencher;
use std::cmp;
use std::vec;

mod board;

fn main() {
  board::location::build();
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
  let nul = 16;
  let BACH:[[u8;17];6] = [[0;17];6];
  b.iter(||{
    let i = test::black_box(3usize);
    let win = test::black_box([1u8,2u8,3u8,4u8,5u8,nul,nul,nul]);
    let mut c = win.iter();
    c.next();
    c.next();
    unsafe{
      let mut b: Vec<u8> = c.map(|s| BACH[i][*s as usize]).collect();
      b.sort();
      b.dedup();
      b.retain(|s| *s != nul);
    }
  });
}




const INF: i16 = 100;
const N_INF: i16 = -100;

fn minmax(brd: &mut board::Board, depth: u8, you: bool) -> (u8, i16) {
  let val = brd.evaluate();

  if depth == 0 || val != 0{
    return (board::location::NUL, val);
  }

  let movs = brd.gen_movs();
  if you {
    let mut v = N_INF;
    let mut mv = 0u8;

    for mov in &movs {
      brd.make_mov(*mov, you);
      let m = minmax(brd, depth - 1, !you);
      if (m.1 > v) {
        mv = m.0;
        v = m.1;
      }
      brd.undo_mov(*mov, you);
    }

    return (mv, v);
  } else {
    let mut v = INF;
    let mut mv = 0u8;

    for mov in &movs {
      brd.make_mov(*mov, you);
      let m = minmax(brd, depth - 1, !you);
      if (m.1 < v) {
        mv = m.0;
        v = m.1;
      }
      brd.undo_mov(*mov, you);
    }

    return (mv, v);
  }
}