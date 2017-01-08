#![feature(type_ascription)]
#![feature(test)]

extern crate test;
extern crate time;

use test::Bencher;
use std::io::{stdin, stdout, BufRead};
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use std::string::String;

use std::str;
use std::slice;
use std::vec;
use std::cmp;

mod board;
mod build;
mod input;
mod tests;

static mut Counter: u32 = 0;

fn main () {

  //let Trans_Table = Mutex::new(HashMap::new());

  // Building everything
  build::all();
  println!("EVENT: READY!");
  // Start the loop to wait
  loop {
    //let mut Moves = HashMap::new();
    let mut buffer = String::new();
    let inp = stdin();
    let out = stdout();

    let mut handle = out.lock();
    for line in inp.lock().lines() {
      let val = line.unwrap() as String;  
      let mut brd = input::parse_board(val);
      unsafe {
        Counter = 0;
      }
      println!("VALUE: {:?}", brd.evaluate());
      println!("{:?}", brd.horiz_o[0]);
      println!("VALUES: {:?}", brd.gen_moves(false));
      let mut moves = next_best(&mut brd);
      moves.sort_by(|a,b| (b.1).cmp(&a.1));
      println!("MOVES: {:?}", moves);
    }
  }
}

pub fn next_best (brd: &mut board::Board) -> Vec<(u8, i32)>{
  let best = i32::min_value();
  let mut make = vec![];
  let moves = brd.gen_moves(true);

  for (index, mov) in moves.iter().enumerate() {
    if mov.1 <= 10 && index > 0 {
      break;
    }

    println!("DOING: {:?}", mov);
    brd.place_piece(mov.0 as usize, true);

    // The depth here has to be odd.
    let val = minmax(brd, 9, false, -20000, 20000);
    make.push((mov.0, val));

    brd.remove_piece(mov.0 as usize, true);
    println!("DONE: {:?}", mov);
  }


  return make;
}



pub fn minmax(brd: &mut board::Board, depth: u8, max: bool, a: i32, b: i32) -> i32 {
  let won = brd.won(!max);
  if won != 0 {
    return won;
  }

  if depth == 0 {
    return brd.evaluate();
  }

  let moves = brd.gen_moves(max);
  let mut alpha = a;
  let mut beta = b;

  if max {
    let mut v = i32::min_value();
    for (index, mov) in moves.iter().enumerate() {
      if mov.1 <= 10 && index > 0 {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let val = minmax(brd, depth - 1, !max, alpha, beta);
      if val > v {
        v = val;
      }
      brd.remove_piece(mov.0 as usize, max);
    }
    return v;
  } else {
    let mut v = i32::max_value();
    for (index, mov) in moves.iter().enumerate() {
      if mov.1 <= 10 && index > 0 {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let val = minmax(brd, depth - 1, !max, alpha, beta);
      if val < v {
        v = val;
      }
      brd.remove_piece(mov.0 as usize, max);
    }
    return v;
  }
}