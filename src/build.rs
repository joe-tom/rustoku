
use std::boxed::Box;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

use std::mem;
use std;

use std::cmp;

use std::thread;
//use std::time::Duration;


pub const THREE_VALUE: i8 = 63;
pub const FOUR_VALUE: i8 = 127;
pub const FOUR_STATE: i32 = 10000;

pub static mut counter:u32 = 0;
pub static mut LENGTH: [i8; 65536] = [0; 65536];
pub const MAX_LENGTH: i8 = 100;

const LONGEST:[i8; 32] = [0,1,1,2,1,1,2,3,1,1,1,2,2,2,3,4,1,1,1,2,1,1,2,3,2,2,2,2,3,3,4,5];


pub fn all () {
  unsafe {
    // This is for benching
    if counter > 0 {
      return;
    }

    println!("COMMENT: BUILDING WON TABLE");
    println!("COMMENT: BUILDING BINARY - TERNARY TABLE");
    for state in 0..65536 {
      let mut shift = 0;
      let mut longest = 0;

      while shift < 11 {
        let cur = (state >> shift) & 0b11111;
        if cur == 0b11111 {
          longest = MAX_LENGTH;
          super::board::WON[state as usize] |= super::board::FIVE_FLAG;
          break;
        }

        longest = cmp::max(LONGEST[cur as usize], longest);
        shift += 1;
      }
      LENGTH[state as usize] = longest;

      super::board::BT[state as usize] = u32::from_str_radix(&format!("{:b}", state),3).unwrap();
      super::board::BT2[state as usize] = 2 * u32::from_str_radix(&format!("{:b}", state),3).unwrap();

    }

    // Build the Move and WON Table.
    // We might want to cache this.
    println!("COMMENT: NO CACHE FOUND. GENERATING...");
    let mut threads = vec![];
    
    threads.push(thread::spawn(move  || {
      let mut Map = HashMap::new();
      binary_recurse(1 << 14,0,13, &mut Map);
    }));
    threads.push(thread::spawn(move || {
      let mut Map = HashMap::new();
      binary_recurse(0,1 << 14,13, &mut Map);
    }));
    threads.push(thread::spawn(move || {
      let mut Map = HashMap::new();
      binary_recurse(0,0,13, &mut Map);
    }));

    for t in threads {
      t.join();
    }

    super::board::MAX_MOVES[0] = [(0,0); 15];
    super::board::MIN_MOVES[0] = [(0,0); 15];
  }
}


/*
 * Recurses through all the places.
 */
unsafe fn binary_recurse(you: u16, opp: u16, depth: i32,mut map: &mut HashMap<(u16,u16),i8>) {
  // Trim :)
  if (super::board::WON[you as usize] | super::board::WON[opp as usize]) != 0 {
    return;
  }
  if depth < 0 {
    counter += 1;
    if counter % 100000 == 0{
      println!("COMMENT: {:}% FINISHED", (((counter as f32) / 14348907f32) * 100f32));
    }
    build_state(you, opp, map);
    return;
  }


  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1, map);
  binary_recurse(you, opp, depth - 1, map);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1, map);
}





fn build_state (you: u16, opp: u16, mut Map: &mut HashMap<(u16,u16),i8>) {
  if (you|opp) == 0b11111_11111_11111 {
    return;
  }
  unsafe {
    let mut mins = get_moves(opp, you, Map);
    let length = mins.len();

    if length > 0 {
      mins.sort_by(|a,b| b.0.cmp(&a.0));

      let state_max: usize = ((2 * (super::board::BT[opp as usize] as u32)) + (super::board::BT[you as usize] as u32)) as usize;
      let state_min: usize = ((super::board::BT[opp as usize] as u32) + (2 * (super::board::BT[you as usize] as u32))) as usize;

      super::board::MIN_ENABLED[state_min] = true;
      super::board::MAX_ENABLED[state_max] = true;

      let mut index = 0;

      while index < length {
        let cur = mins[index];

        super::board::MIN_MOVES[state_min][index] = (cur.0, cur.1);
        super::board::MAX_MOVES[state_max][index] = (cur.0, cur.1);

        super::board::VALUES[state_max] += super::board::MAX_VALS[cur.1 as usize] as i32;
        super::board::VALUES[state_min] += super::board::MIN_VALS[cur.1 as usize] as i32;
        index += 1;
      }
      if index < 15 {
        super::board::MIN_MOVES[state_min][index] = (super::board::EOL, 0);
        super::board::MAX_MOVES[state_max][index] = (super::board::EOL, 0);
      }
    }
  }
}




 









unsafe fn get_moves(you: u16, opp: u16, mut map:  &mut HashMap<(u16,u16),i8>) -> Vec<(u8,u8)> {
  let taken = you | opp;
  let mut moves = vec![];
  for shift in 0..15 {
    let mov = 1 << shift;
    if taken & mov != 0 {
      continue;
    }
    let new_you = you | mov;
    let val = length_depth(new_you, opp, 0, map);
    if val != 0 {
      moves.push((shift, val));
    }
  }
  return moves;
}

unsafe fn length_depth (you: u16, opp: u16, depth: u8, mut map:  &mut HashMap<(u16,u16),i8>) -> u8 {
  let taken = you | opp;
  if LENGTH[you as usize] == 100 || taken == 0b11111_11111_11111 {
    return 1;
  }

  let mut max = 10;

  for shift_1 in 0..15 {
    let you_1 = you | (1 << shift_1);
    if (you_1 & opp) != 0 {
      continue;
    }
    if LENGTH[you_1 as usize] == 100 {
      return 2;
    }
    if (you_1 | opp) == 0b11111_11111_11111 {
      continue;
    }
    for shift_2 in 0..15 {
      let you_2 = you_1 | (1 << shift_2);
      if (you_2 & opp) != 0 {
        continue;
      }
      if LENGTH[you_2 as usize] == 100 {
        max = cmp::min(3, max);
      }
      if (you_2 | opp) == 0b11111_11111_11111 {
        continue;
      }
      for shift_3 in 0..15 {
        let you_3 = you_2 | (1 << shift_3);
        if (you_3 & opp) != 0 {
          continue;
        }
        if LENGTH[you_3 as usize] == 100 {
          max = cmp::min(4, max);
        }/*
        if (you_3 | opp) == 0b11111_11111_11111 || (you_3 & opp) != 0 {
          continue;
        }
        for shift_4 in 0..15 {
          let you_4 = you_3 | (1 << shift_4);
          if you_4 & opp != 0 {
            continue;
          }
          if LENGTH[you_4 as usize] == 100 {
            max = cmp::min(4, max);
          }
        }*/
      }
    }
  }
  if max == 10 {
    return 0;
  }
  return max;
}


















/*

// This is tryna max the move
unsafe fn opp_maxer (you: u16, opp: u16, mut map: &mut HashMap<(u16, u16, bool), i8>) ->  Vec<(u8, i8)> {
  let taken = you | opp;
  let mut moves: Vec<(u8, i8)> = vec![];
  let mut v = 110;

  for shift in 0..15 {
    let mov = 1 << shift;

    if (taken & mov) != 0 {
      continue;
    }

    let val = you_no(you | mov, opp, false, &mut map);

    if val < v {
      v = val;
    }
    if val == v {
      moves.push((shift, val));
    }
  }

  return moves;
}
// This is tryna max the move
unsafe fn you_maxer (you: u16, opp: u16, mut map: &mut HashMap<(u16, u16, bool), i8>) ->  Vec<(u8, i8)> {
  let taken = you | opp;
  let mut moves: Vec<(u8, i8)> = vec![];
  let mut v = -110;

  for shift in 0..15 {
    let mov = 1 << shift;

    if (taken & mov) != 0 {
      continue;
    }

    let val = you_no(you | mov, opp, true, &mut map);

    if val > v {
      v = val;
    }
    if val == v {
      moves.push((shift, val));
    }
  }

  return moves;
}

// This is tryna find the worst move
unsafe fn you_no (you: u16, opp: u16, max: bool, mut map: &mut HashMap<(u16, u16, bool), i8>) -> i8 {
  let taken = you | opp;
  if LENGTH[opp as usize] == 100 {
    return -100;
  }

  if taken == 0b11111_11111_11111 {
    if max {
      return LENGTH[opp as usize];
    } else {
      return -LENGTH[you as usize];
    }
  }
  let mut v = 110;

  match map.get(&(you, opp, max)) {
    Some(x) => {return *x}
    _ => {}
  }


  for shift in 0..15 {
    let mov = 1 << shift;

    if (taken & mov) != 0 {
      continue;
    }

    let value = you_yes(you, opp | mov, max, &mut map);

    if value < v {
      v = value;
    }
  }
  map.insert((you, opp, max), v);
  return v;
}

// This is tryna find the best move
unsafe fn you_yes (you: u16, opp: u16, max: bool, mut map: &mut HashMap<(u16, u16, bool), i8>) -> i8 {
  let taken = you | opp;
  if LENGTH[you as usize] == 100 {
    return 100;
  }

  if taken == 0b11111_11111_11111 {
    if max {
      return LENGTH[you as usize];
    } else {
      return -LENGTH[opp as usize];
    }
  }

  match map.get(&(you, opp, max)) {
    Some(x) => {return *x}
    _ => {}
  }

  let mut v = -110;
  for shift in 0..15 {
    let mov = 1 << shift;

    if (taken & mov) != 0 {
      continue;
    }

    let value = you_no(you | mov, opp, max, &mut map);

    if value > v {
      v = value;
    }
  }
  map.insert((you, opp, max), v);
  return v;
}
*/
// Traverse one level down, with four loop.
// Check to see if value CAN increase. If so, head down.
// Do the same for the opponent.






























































/*

unsafe fn minimize (you: u16, opp: u16, mut map: &mut HashMap<(u16, u16, bool),i8>) -> Vec<(u8, i8)> {
  let taken = you | opp;
  let mut moves: Vec<(u8, i8)> = vec![];

  for mov in 0..15 {
    if (taken >> mov) & 1 == 1 {
      continue;
    }
    let new_opp = opp | (1 << mov);
    let value = minmax(you, new_opp, 1, true, map);
      moves.push((mov, value));
    if value >= 0 {
    }
  }

  return moves;
}

unsafe fn maximize (you: u16, opp: u16, mut map: &mut HashMap<(u16, u16, bool),i8>) -> Vec<(u8, i8)> {
  let taken = you | opp;
  let mut moves: Vec<(u8, i8)> = vec![];

  for mov in 0..15 {
    if (taken >> mov) & 1 == 1 {
      continue;
    }
    let new_you = you | (1 << mov);
    let value = minmax(new_you, opp, 1, false, map);
      moves.push((mov, value));
    if value >= 0 {
    }
  }

  return moves;
}

// tuple is in the form of (MOVE, VALUE, DEPTH)
unsafe fn minmax (you: u16, opp: u16, depth: u8, max: bool, mut map: &mut HashMap<(u16, u16, bool),i8>) -> i8 {
  if super::board::WON[you as usize] != 0 {
    map.insert((you, opp, max), 1);
    return (20-(depth as i8));
  }
  if super::board::WON[opp as usize] != 0 {
    map.insert((you, opp, max), -1);
    return -(20-(depth as i8));
  } 
  let taken = you | opp;
  if taken == 0b11111_11111_11111 {
    map.insert((you, opp, max), 0);
    return 0;
  }

  if depth >= 8 {
    return 0;
  }

  match map.get(&(you, opp, max)) {
    Some(x) => {return *x;}
    _ => {}
  }


  if max {
    let mut v = -200;
    for mov in 0..15 {
      if (taken >> mov) & 1 == 1 {
        continue;
      }
      let new_you = you | (1 << mov);
      let value = minmax(new_you, opp, depth + 1, !max, map);
      if v < value {
        v = value;
      }
    }
    map.insert((you,opp,max), v);
    return v;
  } else {
    let mut v = 200;
    for mov in 0..15 {
      if (taken >> mov) & 1 == 1 {
        continue;
      }
      let new_opp = opp | (1 << mov);
      let value = minmax(you, new_opp, depth + 1, !max, map);
      if v > value {
        v = value;
      }
    }
    map.insert((you,opp,max), v);
    return v;
  }

}















 */