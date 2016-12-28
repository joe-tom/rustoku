#![feature(type_ascription)]
#![feature(test)]

extern crate test;
use test::Bencher;
use std::io::{stdin, stdout, BufRead};

mod board;
mod build;
mod input;

fn main () {
  // Building everything
  build::all();
  println!("EVENT: READY!");

  // Start the loop to wait
  let mut buffer = String::new();
  loop {
    let inp = stdin();
    let out = stdout();

    let mut handle = out.lock();
    for line in inp.lock().lines() {
      println!("MOVES: {:?}", input::parse_board(line.unwrap() as String).gen_moves());
    }
  }
}




use std::collections::HashMap;
use std::slice;
use std::str;


#[bench]
fn gen_moves(b: &mut test::Bencher) {
  /*
  let mut brd = board::Board {
    multi: [[0; 15]; 15],
    
    horiz_y: [0; 15],
    horiz_o: [0; 15],
    verti_y: [0; 15],
    verti_o: [0; 15],
    diagr_y: [0; 19],
    diagr_o: [0; 19],
    diagl_y: [0; 19],
    diagl_o: [0; 19]
  };
*/
let mut books = HashMap::new();

  b.iter(|| {/*
    let a = test::black_box(10);
    let b = test::black_box(false);
    test::black_box(brd.won(true));*/
    unsafe{
      let mut a = test::black_box([12341u16,5245234u16,1234123u16]);
      books.insert(a,31);
      test::black_box(books.get(&a));
    }
  })
}


#[test]
fn move_place() {
  unsafe {
    let c = std::mem::transmute::<&[[(u8,u8); 15]; 14348907], &[u8; 430467210]>(&board::MOVES);
  }

}





















/**
 * The actual minimax function
 */

fn minimax (brd: &mut board::Board, depth: u8, you: bool) -> i16 {
  if depth == 0 {
    if brd.won(!you) {
      return if you {100} else {-100};
    }

    return 0;
  }

  if brd.won(!you) {
    return if you {100} else {-100};
  }

  let movs = brd.gen_moves();
  if you {  
    let mut v = 0;
    for mov in &movs {
      brd.place_piece(mov.0 as usize, you);
      let val = minimax(brd, depth - 1, !you);
      brd.remove_piece(mov.0 as usize, you);
      
      if val > v {
        v = val;
        break;
      }
    }
    return v;
  } else {
    let mut v = 0;
    for mov in &movs {
      brd.place_piece(mov.0 as usize, you);
      let val = minimax(brd, depth - 1, !you);
      brd.remove_piece(mov.0 as usize, you);
      
      if val > v {
        v = val;
        break;
      }
    }
    return v;
  }
}
