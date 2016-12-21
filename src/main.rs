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

  brd.make_mov(9, true);
  brd.make_mov(25, true);
  brd.make_mov(41, true);
  brd.make_mov(57, true);

  println!("{:?}", minmax(&mut brd, 2,true));

  unsafe{

  println!("{:?}", count);
  }
}

#[bench]
fn bench_add_two(b: &mut Bencher) {
  // let nul = 16;
  // let BACH:[[u8;17];6] = [[0;17];6];

  // let mut brd = board::Board {
  //   horiz_y: [0; 15],
  //   horiz_o: [0; 15],
  //   verti_y: [0; 15],
  //   verti_o: [0; 15],
  //   diagr_y: [0; 21],
  //   diagr_o: [0; 21],
  //   diagl_y: [0; 21],
  //   diagl_o: [0; 21]
  // };

  b.iter(||{

    let mut a = test::black_box(12u8);
    let mut b = test::black_box(13u8);

    unsafe {/*
      let c = board::location::WON[a as usize] + (2 * board::location::WON[b as usize]);
      let d = board::location::WON[c as usize];
      let e = test::black_box(&board::location::WON);
      let mut f = e.iter();
      f.next();
      test::black_box(f);
      test::black_box(d);*/
      let a = test::black_box(7);
      let d = 120312312123u32;
      let e = (d-3u32.pow(a));
      test::black_box(e);
    }
    // let mut brd = test::black_box(&brd);
    // unsafe {
    //   let a = brd.evaluate();
    //   if a == 1231 {
    //     println!("asdf");
    //   }
    // }


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
  println!("{:?}", movs);
  if you {
    let mut v = 0;
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
    let mut v = 0;
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