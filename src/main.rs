#![feature(test)]
#![feature(type_ascription)]

extern crate test;
use test::Bencher;

mod board;
mod build;



#[bench]
fn gen_moves(b: &mut test::Bencher) {
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

  b.iter(|| {
    let a = test::black_box(10);
    let b = test::black_box(false);
    brd.place_piece(a,b);
  })
}