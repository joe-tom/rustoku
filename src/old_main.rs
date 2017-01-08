




































































/*

use std::collections::HashMap;
use std::slice;
use std::str;
use std::string::String;

/**
 * Just a wrapper for the first level
 */
fn next_best (brd: &mut board::Board, depth: u8, max: bool, a: i32, b: i32, map: &mut HashMap<String, (u8,i32)>) -> Vec<(u8, i32)> {
  let mut moves = brd.gen_moves(true);
  //let mut values = vec![];


  let mut alpha = a;
  let mut beta = b;

  let mut v = -1;
  /*
  for mov in moves {
    brd.place_piece(mov.0 as usize, max);
    let d_val = (minimax(brd, depth + 1, !max, alpha, beta, map));
    brd.remove_piece(mov.0 as usize, max);
    values.push((mov.0, d_val.1));
    println!("DONE: {:?}", (mov.0, d_val.1));
    if d_val.1 == 20000 {
      break;
    }
    if v < d_val.1 {
      v = d_val.1;
      alpha = cmp::max(alpha, v);
      if beta <= alpha {
        break;
      }
    }
  }

  values.sort_by(|a,b| (b.1).cmp(&a.1));
  if values.len() == 0 {
    return brd.gen_moves(true).iter().map(|x| (x.0,x.1 as i32)).collect();
  } else {
    return values;
  }
  */
 return vec![(0,1)];
}

/**
 * The actual minimax function
 */
pub const DEPTH: u8 = 5;
pub const THRESHOLD: i16 = 4;
pub const MAX_MOVES: usize = 5;

static mut Counter:u64 = 0;
static mut Hits:u64 = 0;

fn minimax(brd: &mut board::Board, depth: u8, max: bool, a: i32, b: i32, map: &mut HashMap<String, (u8,i32)>) -> (u8, i32) {

  unsafe {
    Counter += 1;
    if Counter % 1000000 == 0{
      let vals = (Counter / 1000000);
      println!("WE'VE EVALUATED {:?}M NODES", vals);
    }
  }

  let val = brd.won(max);
  
  if val != 0 {
    return (0, val);
  }
  if depth >= DEPTH {
    return (0, brd.evaluate(max));
  }

  let movs = brd.gen_moves(true);
  
  
  let brd_str: String = brd.multi.iter().cloned().collect();
  match map.get(&brd_str) {
    Some(value) => {unsafe{Hits += 1;}return *value;}
    None => {}
  }

  let mut mv = (0,0);

  let mut alpha = a;
  let mut beta = b;

  if max {
    let mut v = -20000;
    for mov in &movs {
      if mov.1.abs() < THRESHOLD + (depth as i16 - 1) {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let d_val = (minimax(brd, depth + 1, !max, alpha, beta, map));
      let brd_str: String = brd.multi.iter().cloned().collect();
      map.insert(brd_str, d_val);
      brd.remove_piece(mov.0 as usize, max);
      
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
      if mov.1.abs() <= THRESHOLD + (depth as i16 - 1) {
        break;
      }
      brd.place_piece(mov.0 as usize, max);
      let d_val = (minimax(brd, depth + 1, !max, alpha, beta,  map));
      let brd_str: String = brd.multi.iter().cloned().collect();
      map.insert(brd_str, d_val);
      brd.remove_piece(mov.0 as usize, max);
      
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


*/