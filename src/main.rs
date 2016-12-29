#![feature(type_ascription)]
#![feature(test)]

extern crate test;
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
      println!("{:?}",brd.won());
      println!("MOVES: {:?}", minimax(&mut brd,-100,100,4,true));
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

const THRESHOLD: u16 = 100;

fn minimax (brd: &mut board::Board,mut a: i16,mut b: i16, depth: u8, you: bool) -> (u8, i16) {
  if depth == 0 {
    let done = brd.gen_moves()[0];
    return (done.0, done.1 as i16);
  }
  let won = brd.won();
  match won {
    2 => {
      return (0, 100);
    },
    1 => {
      return (0, -100);
    }
    _ => {}
  }
  let movs = brd.gen_moves();


  if you {
    let v = -100;
    for mov in &movs {
      if mov.1 <= THRESHOLD {
        break;
      }
      brd.place_piece(mov.0 as usize, you);
      let v = cmp::max(minimax(brd,a,b,depth - 1, !you).1, v);
      a = cmp::max(a, v);
      brd.remove_piece(mov.0 as usize, you);
      if b <= a {
        return (mov.0, v);
      }
    }
    
    return (movs[0].0, v);
  } else {
    let v = 100;
    for mov in &movs {
      if mov.1 <= THRESHOLD {
        break;
      }
      brd.place_piece(mov.0 as usize, you);
      let v = cmp::min(minimax(brd,a,b,depth - 1, !you).1, v);
      b = cmp::min(v,b);
      brd.remove_piece(mov.0 as usize, you);
      if b <= a{
        return (mov.0, v);
      }
    }
    
    return (movs[0].0, v);
  }
}