#![feature(type_ascription)]
#![feature(test)]

extern crate test;
extern crate time;

use test::Bencher;
use std::io::{stdin, stdout, BufRead};
use std::vec;
use std::cmp;


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
      let moves = next_best(&mut brd);
      println!("HEY!@#$");
      println!("MOVES: {:?}", moves);
    }
  }
}
/*


fn next_best (brd: &mut board::Board) -> Vec<(u8,u8)> {
  let mut movs = brd.gen_moves();
  let mut val =
   
  for mov in &movs {
  }
}
*/

use std::collections::HashMap;
use std::slice;
use std::str;

/**
 * The actual minimax function
 */

const THRESHOLD: u16 = 1;


pub fn next_best(brd: &mut board::Board) -> Vec<(u8, u16)> {
  let mut moves = vec![];
  let mut mv:(u8,u16) = (0,0);

  let movs = brd.gen_moves();


  for mov in &movs {
    if mov.1 == 1{
      break;
    }
    brd.place_piece(mov.0 as usize, true);
    
    let movs_2 = brd.gen_moves();

    for mov_2 in &movs {
      if mov_2.1 == 1{
        break;
      }
      brd.place_piece();
    }







    println!("move {:?} value {:?}", mov, val);
    if mv.1 < val{
      mv.0 = mov.0;
      mv.1 = val;
    }
    brd.remove_piece(mov.0 as usize, true);
  }

  moves.push(mv);
  moves.push((0,0));
  return moves;
}
