#![feature(test)]

extern crate test;
use test::Bencher;
use std::cmp;
use std::vec;

mod board;

pub static mut count: u32 = 0u32;
fn main() {
  let mut brd = board::Board {
    horiz_y: [0; 15],
    horiz_o: [0; 15],
    verti_y: [0; 15],
    verti_o: [0; 15],
    diagr_y: [0; 21],
    diagr_o: [0; 21],
    diagl_y: [0; 21],
    diagl_o: [0; 21]
  };

  board::location::build();
  minmax(&mut brd,4,true);
  unsafe{

  println!("{:?}", count);
  }
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
  let nul = 16;
  let BACH:[[u8;17];6] = [[0;17];6];

  let mut brd = board::Board {
    horiz_y: [0; 15],
    horiz_o: [0; 15],
    verti_y: [0; 15],
    verti_o: [0; 15],
    diagr_y: [0; 21],
    diagr_o: [0; 21],
    diagl_y: [0; 21],
    diagl_o: [0; 21]
  };

  b.iter(||{
    let mut brd = test::black_box(&brd);
    unsafe {
      let a = brd.evaluate();
      if a == 1231 {
        println!("asdf");
      }
    }
  });
}




const INF: i16 = 100;
const N_INF: i16 = -100;

fn minmax(brd: &mut board::Board, depth: u8, you: bool) -> (u8, i16) {
  unsafe {count += 1;}
  let val = brd.evaluate();

  if depth == 0 || val != 0{
    return (board::location::NUL, val);
  }

  
    let movs = unsafe {brd.gen_movs(you)};
  if you {
    let mut v = N_INF;
    let mut mv = 0u8;

    for mov in &movs {
      brd.make_mov(*mov, you);
      let m = minmax(brd, depth - 1, !you);
      if (m.1 > v) {
        return (*mov, m.1);
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
        return (*mov, m.1)
      }
      brd.undo_mov(*mov, you);
    }

    return (mv, v);
  }
}