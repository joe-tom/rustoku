






// Accepts a csv of the board. and returns a real board
pub fn parse_board (board: String) -> super::board::Board{
  let mut squares = board.split(',');

  let mut B = super::board::Board{
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

  let mut i = 0;
  loop {
    match squares.next() {
      Some(x) => {
        let val = u32::from_str_radix(x, 10).unwrap();
        if val == 1 {
          B.place_piece(i, true);
        }
        if val == 2 {
          B.place_piece(i, false);
        }
      },
      None => break
    }
    i += 1;
  }

  println!("COMMENT: {:?}", B.horiz_y[0]);

  return B;
}