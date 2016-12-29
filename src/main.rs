#![feature(type_ascription)]
#![feature(test)]

extern crate test;
use test::Bencher;
use std::io::{stdin, stdout, BufRead};
use std::vec;

mod board;
mod build;
mod input;
mod tests;

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
      let mut brd = input::parse_board(line.unwrap() as String);
      println!("{:?}",brd.won());
      println!("MOVES: {:?}", brd.gen_moves());
    }
  }
}



fn next_best (brd: &mut board::Board) -> Vec<(u8,u8)> {
  let mut movs = brd.gen_moves();
  let mut val =
   
  for mov in &movs {
  }
}


use std::collections::HashMap;
use std::slice;
use std::str;

/**
 * The actual minimax function
 */

fn minimax (brd: &mut board::Board, depth: u8, you: bool) -> i16 {
  if depth == 0 {
    if brd.won() != 0 {
      return if you {100} else {-100};
    }

    return 0;
  }

  if brd.won() != 0 {
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
