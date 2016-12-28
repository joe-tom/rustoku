use std::boxed::Box;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::{self, BufReader};
use std::io::prelude::*;

use std::mem;
use std;


static mut tf:[u8; 430467210] = [0; 430467210];
pub fn all () {
  unsafe {

    // Build the BT Table.
    println!("COMMENT: BUILDING BINARY - TERNARY TABLE");
    for state in 0..65536{
      super::board::BT[state as usize] = u32::from_str_radix(&format!("{:b}", state),3).unwrap();
    }

    // Build the Move and WON Table.
    // We might want to cache this.
    println!("COMMENT: NO CACHE FOUND. GENERATING...");
    binary_recurse(0,0,14);
    super::board::MOVES[0] = [(0,0);15];
    /*
    match File::open("WON_TABLE_CACHE.bin") {
      Ok(mut won_file) => {
        println!("COMMENT: CACHES FOUND, READING WON CACHE");
        let mut move_file = File::open("MOVE_TABLE_CACHE.bin").ok().unwrap();
        won_file.read(&mut super::board::WON);
        println!("COMMENT: CACHES FOUND, READING MOVE CACHE");

        unsafe {
          move_file.read(&mut tf);
          let mut i = 0;
          while i < 14348907 {
            super::board::MOVES[i] = [(tf[i+0],tf[i+1]),(tf[i+2],tf[i+3]),(tf[i+4],tf[i+5]),(tf[i+6],tf[i+7]),(tf[i+8],tf[i+9]),(tf[i+10],tf[i+11]),(tf[i+12],tf[i+13]),(tf[i+14],tf[i+15]),(tf[i+16],tf[i+17]),(tf[i+18],tf[i+19]),(tf[i+20],tf[i+21]),(tf[i+22],tf[i+23]),(tf[i+24],tf[i+25]),(tf[i+26],tf[i+27]),(tf[28],tf[29])];
            i += 30;
          }
        }
      }
      Err(e) => {
        println!("COMMENT: NO CACHE FOUND. GENERATING...");
        binary_recurse(0,0,14);
        println!("COMMENT: FINISHED GENERATING, WRITING FILES.");

        //let mut won_file = File::create("WON_TABLE_CACHE.bin").ok().unwrap();
        //won_file.write(&super::board::WON);
        let mut move_file = File::create("MOVE_TABLE_CACHE.bin").ok().unwrap();

        unsafe {
          let c = std::mem::transmute::<&[[(u8,u8); 15]; 14348907], &[u8; 430467210]>(&super::board::MOVES);
          move_file.write(c);
        }
      }
    }
    */
  }
}


/**
 * Recurses through all the places.
 */
pub static mut counter:u32 = 0;
unsafe fn binary_recurse(you: u16, opp: u16, depth: i32) {
  
  if depth < 0 {
    counter += 1;
    if counter % 100000 == 0{
      println!("COMMENT: {:}% FINISHED", (((counter as f32) / 14348907f32) * 100f32).round());
    }
    let state: u32 = ((2 * (super::board::BT[opp as usize] as u32)) + (super::board::BT[you as usize] as u32));
    build_state(you, opp, state as usize);
    return;
  }

  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1);
  binary_recurse(you, opp, depth - 1);
}


/**
 * Builds WON_TABLE and STATE_TABLE together
 */
unsafe fn build_state(you: u16, opp: u16, state: usize) {
  let mut you_movs: Vec<(u8, u8)> = vec![];

  for shift in 0..11u16{
    let you_state = (you >> shift) & 0b11111;
    let opp_state = (opp >> shift) & 0b11111;

    if (you_state == 0) && (opp_state == 0) {
      continue;
    }
    if (you_state == 0b11111) || (opp_state == 0b11111) {
      super::board::WON[you as usize] = 100;
      return;
    }

    if you_state == 0 {
      you_movs.extend(get_five(opp_state, shift));
    }
    if opp_state == 0 {
      you_movs.extend(get_five(you_state, shift));
    }
  }

  you_movs.sort_by(|a,b| (a.0).cmp(&b.0));
  let mut real_movs: Vec<(u8, u8)> = vec![];
  let mut first = false;
  let mut cur_mov = (15,15);

  for mov in &you_movs {
    if mov.0 == cur_mov.0 {
      cur_mov.1 += mov.1
    } else {
      if first {
        real_movs.push(cur_mov);
        cur_mov = *mov;
      }else {
        first = true;
        cur_mov = *mov;
      }
    }
  }

  let mut i:u8 = 0;
  for mov in &real_movs {
    super::board::MOVES[state][i as usize] = *mov;
    i += 1;
  }
}


/**
 * Accepts a binary, returns a vector of tuples, with moves and urgency.
 */
fn get_five(binary: u16, cur_shift: u16) -> Vec<(u8, u8)>{
  let mut movs: Vec<u8> = vec![];

  for shift in 0..5u16 {
    if (binary >> shift) & 1 == 0 {
      movs.push((shift + cur_shift) as u8); 
    }
  }
  
  let value = (5 - (movs.len() as u8)) * 5;
  let mut mov_urg: Vec<(u8, u8)> = vec![];

  for mov in &movs {
    mov_urg.push((*mov, value));
  }

  return mov_urg;
}