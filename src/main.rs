#![feature(type_ascription)]
#![feature(test)]

extern crate test;
use test::Bencher;

mod board;
mod build;
mod input;

fn main () {
  build::all();
  unsafe { 
    println!("000000111000 {:?}", board::MOVES[0][u32::from_str_radix("000000111000",3).unwrap() as usize]);
    println!("000000110100 {:?}", board::MOVES[0][u32::from_str_radix("000000110100",3).unwrap() as usize]);
    println!("000000111100 {:?}", board::MOVES[0][u32::from_str_radix("000000111100",3).unwrap() as usize]);
    println!("000000110000 {:?}", board::MOVES[0][u32::from_str_radix("000000110000",3).unwrap() as usize]);
  }
}

#[bench]
fn gen_moves(b: &mut test::Bencher) {
  
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

  b.iter(|| {
    let a = test::black_box(10);
    let b = test::black_box(false);
    test::black_box(brd.won(true));
  })
}


#[test]
fn move_place() {
  build::all();
  unsafe { 
    println!("000000111000 {:?}", board::MOVES[0][u32::from_str_radix("000000111000",3).unwrap() as usize]);
    println!("000000110100 {:?}", board::MOVES[0][u32::from_str_radix("000000110100",3).unwrap() as usize]);
    println!("000000111100 {:?}", board::MOVES[0][u32::from_str_radix("000000111100",3).unwrap() as usize]);
    println!("000000110000 {:?}", board::MOVES[0][u32::from_str_radix("000000110000",3).unwrap() as usize]);
  }
}



