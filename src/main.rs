#![feature(type_ascription)]
#![feature(test)]

extern crate test;
use test::Bencher;


mod board;
mod build;
mod input;

fn main () {
  build::all();
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
  brd.place_piece(84, true);
  println!("{:?}", brd.gen_moves());
  unsafe {
    println!("000000111000 {:?}", board::MOVES[u32::from_str_radix("000000111000",3).unwrap() as usize]);
    println!("000000110100 {:?}", board::MOVES[u32::from_str_radix("000000110100",3).unwrap() as usize]);
    println!("000000111100 {:?}", board::MOVES[u32::from_str_radix("000000111100",3).unwrap() as usize]);
    println!("000000110000 {:?}", board::MOVES[u32::from_str_radix("000000110000",3).unwrap() as usize]);
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
































