
extern crate test;
use test::Bencher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;
use std::string::String;
/*
#[bench]
fn hash_vec(b: &mut test::Bencher) {
  let mut map: HashMap<String,u8> = HashMap::new();

  b.iter(|| {
    let v = test::black_box(vec!['6','1','2','3']);
    let s = test::black_box(v.into_iter().collect());
    map.insert(s, test::black_box(123));
  })
}

#[bench]
fn hash_vec_now(b: &mut test::Bencher) {
  let mut map: HashMap<String,u8> = HashMap::new();

  b.iter(|| {
    let v = test::black_box(vec![1,2,3,4,5]);
    let s = test::black_box(String::from_utf8(v.clone()).unwrap());
    map.insert(s, test::black_box(123));
  })
}

*/


#[bench]
fn pointer(b: &mut test::Bencher) {
  super::build::all();

  let mut B = super::board::Board{
    multi: [
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0',
      '0','0','0','0','0','0','0','0','0','0','0','0','0','0','0'
    ],

    horiz_y: [0; 21],
    horiz_o: [0; 21],
    verti_y: [0; 21],
    verti_o: [0; 21],
    diagr_y: [0; 21],
    diagr_o: [0; 21],
    diagl_y: [0; 21],
    diagl_o: [0; 21]
  };
  B.place_piece(32, true);
  B.place_piece(12, true);
  b.iter(|| {
    //let mut c = test::black_box(B.gen_moves());
  })
}

/*

use std::collections::HashMap;
pub static mut WON: [u8; 65536] = [0; 65536]; 

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
  if WON[you as usize] != 0 {
    map.insert((you, opp, max), 1);
    return (20-(depth as i8));
  }
  if WON[opp as usize] != 0 {
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



pub fn main() {
unsafe{
    
    for state in 0..65536 {
      let mut shift = 0;
      while shift < 11 {
        if (state >> shift) & 0b11111 == 0b11111 {
          WON[state as usize] |= 10;
          break;
        }
        shift += 1;
      }
    }
let mut a = HashMap::new();
println!("{:?}", maximize(0b0011110,0,&mut a));
}
    
}
 */