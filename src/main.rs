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
  loop {
    let mut Moves = HashMap::new();
    let mut buffer = String::new();
    let inp = stdin();
    let out = stdout();

    let mut handle = out.lock();
    for line in inp.lock().lines() {
      let val = line.unwrap() as String;
      println!("{:?}", val);
      let mut brd = input::parse_board(val);
      unsafe {
        counter = 0;
      }
      let moves = vec![minimax(&mut brd, 4, true, -20000, 20000, &mut Moves),(0,0),(0,0)];
      println!("MOVES: {:?}", moves);
      println!("VALUES: {:?}", brd.gen_moves());
    }
  }
}




use std::collections::HashMap;
use std::slice;
use std::str;
use std::string::String;

/**
 * The actual minimax function
 */

const THRESHOLD: u16 = 5;
static mut counter:u64 = 0;

fn minimax(brd: &mut board::Board, depth: u8, max: bool, a: i32, b: i32, map: &mut HashMap<String, (u8,i32)>) -> (u8, i32) {

  unsafe {
    counter += 1;
    if counter % 100000 == 0{
      let vals = (counter / 1000);
      println!("WE'VE EVALUATED {:?}k NODES", vals);
    }
  }

  let val = brd.won();
  
  if val != 0 {
    return (0, val);
  }

  let movs = brd.gen_moves();
  if depth == 0 {
    if max {
      return (0, (movs[0].1 as i32));
    } else {
      return (0, -(movs[0].1 as i32));
    }
  }

  let brd_str = String::from_utf8(brd.multi.clone()).unwrap();
  match map.get(&brd_str) {
    Some(value) => {return *value;}
    None => {}
  }

  let mut mv = (0,0);

  let mut alpha = a;
  let mut beta = b;

  if max {
    let mut v = -20000;
    for mov in &movs {
      if mov.1 <= THRESHOLD {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let d_val = (minimax(brd, depth - 1, !max, alpha, beta, map));
      brd.remove_piece(mov.0 as usize, max);
      let brd_str = String::from_utf8(brd.multi.clone()).unwrap();
      map.insert(brd_str, d_val);
      
      if v < d_val.1 {
        v = d_val.1;
        mv = *mov;

        alpha = cmp::max(alpha, v);
        if beta <= alpha {
          break;
        }
      }
    }

    return (mv.0, v);
  } else {
    let mut v = 20000;
    for mov in &movs {
      if mov.1 <= THRESHOLD {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let d_val = (minimax(brd, depth - 1, !max, alpha, beta,  map));
      brd.remove_piece(mov.0 as usize, max);
      let brd_str = String::from_utf8(brd.multi.clone()).unwrap();
      map.insert(brd_str, d_val);
      
      if v > d_val.1 {
        v = d_val.1;
        mv = *mov;

        beta = cmp::min(beta, v);
        if beta <= alpha {
          break;
        }
      }
    }

    return (mv.0, v);
  }

}


