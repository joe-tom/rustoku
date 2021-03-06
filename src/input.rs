
// Accepts a csv of the board. and returns a real board
pub fn parse_board (board: String) -> super::board::Board{
  let mut squares = board.split(',');

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

  B.multi = [
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
  ];
  for i in 0..21usize {
    B.horiz_y[i] = 0;
    B.horiz_o[i] = 0;
    B.verti_y[i] = 0;
    B.verti_o[i] = 0;
    B.diagr_y[i] = 0;
    B.diagr_o[i] = 0;
    B.diagl_y[i] = 0;
    B.diagl_o[i] = 0;
  }
  for i in 0..225usize{
    B.multi[i] = '0';
  }

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
  println!("DON!");

  return B;
}