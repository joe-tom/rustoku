
#[test]
fn test__genpiece() {
  
  super::build::all();
  let mut brd = super::board::Board {
    multi: [[0; 15]; 15],
    horiz_y: [0; 21], horiz_o: [0; 21],
    verti_y: [0; 21], verti_o: [0; 21],
    diagr_y: [0; 21], diagr_o: [0; 21],
    diagl_y: [0; 21], diagl_o: [0; 21]
  };


  brd.place_piece(84,true);
  brd.place_piece(79,true);

  let mut go = brd.gen_moves();
  go.retain(|x| x.0 == 84 || x.0 == 79);
  assert_eq!(go.len(), 0);
}


#[test]
fn test_removepiece() {
  
}



























