#![feature(type_ascription)]
#![feature(test)]

extern crate test;
use test::Bencher;
use std::io::{stdin, stdout, BufRead};

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




use std::collections::HashMap;
use std::slice;
use std::str;


#[bench]
fn gen_moves(b: &mut test::Bencher) {
  //build::all();

  let mut brd = board::Board {
    multi: [0; 225],
    
    horiz_y: [0; 21],
    horiz_o: [0; 21],
    verti_y: [0; 21],
    verti_o: [0; 21],
    diagr_y: [0; 21],
    diagr_o: [0; 21],
    diagl_y: [0; 21],
    diagl_o: [0; 21]
  };

  brd.place_piece(014,true);
  brd.place_piece(028,true);
  brd.place_piece(042,true);
  brd.place_piece(056,true);
  brd.place_piece(070,true);
  brd.place_piece(084,true);
  brd.place_piece(154,true);
  brd.place_piece(168,true);
  brd.place_piece(182,true);
  brd.place_piece(196,true);
  brd.place_piece(210,true);
//let mut books = HashMap::new();

  b.iter(|| {
    let mut bord = test::black_box(&brd);
    bord.gen_moves();
  /*
    let a = test::black_box(10);
    let b = test::black_box(false);
    test::black_box(brd.won(true));
    unsafe{
      let mut a = test::black_box([12341u16,5245234u16,1234123u16]);
      books.insert(a,31);
      test::black_box(books.get(&a));
    }*/
  })
}

#[bench]
fn less_moves(b: &mut test::Bencher) {
  build::all();

  let mut brd = board::Board {
    multi: [0; 225],
    
    horiz_y: [0; 21],
    horiz_o: [0; 21],
    verti_y: [0; 21],
    verti_o: [0; 21],
    diagr_y: [0; 21],
    diagr_o: [0; 21],
    diagl_y: [0; 21],
    diagl_o: [0; 21]
  };
  brd.place_piece(070,true);
  brd.place_piece(084,true);
  brd.place_piece(154,true);
  brd.place_piece(168,true);
  brd.place_piece(182,true);

  b.iter(|| {
    let mut bord = test::black_box(&brd);
    bord.gen_moves();
  })
}




















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
