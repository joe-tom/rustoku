
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