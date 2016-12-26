// The array that allows binary to ternary conversion
pub static BT: [u32; 65536] = [0; 65536];

// The arrays for move lookup
pub static MOVES: [[[u8; 15]; 14348907]; 2] = [[[(0, 0); 15]; 14348907]; 2];

pub struct Board {
  pub multi: [[u8; 15]; 15],

  pub horiz_y: [u16; 15],
  pub horiz_o: [u16; 15],
  pub verti_y: [u16; 15],
  pub verti_o: [u16; 15],

  pub diagr_y: [u16; 19],
  pub diagr_o: [u16; 19],
  pub diagl_y: [u16; 19],
  pub diagl_o: [u16; 19]
}


impl Board {
  pub fn gen_moves (&self, you: bool) -> Vec<u8> {
    unsafe {
      
    }
  }

  pub fn place_piece (&mut self, place: usize, you: bool) {
    unsafe {
      if you {
        self.place_horiz_you(place);
        self.place_verti_you(place);
      } else {
        self.place_horiz_opp(place);
        self.place_verti_opp(place);
      }
    }
  }
  pub unsafe fn place_horiz_you (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_y[mov.0 as usize] |= (1 << mov.1); 
  }
  pub unsafe fn place_verti_you (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_y[mov.0 as usize] |= (1 << mov.1); 
  }
  pub unsafe fn place_horiz_opp (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_o[mov.0 as usize] |= (1 << mov.1); 
  }
  pub unsafe fn place_verti_opp (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_o[mov.0 as usize] |= (1 << mov.1); 
  }
}



pub static HORIZ: [(u8, u8); 225] = [
  (0, 14),  (0, 13),  (0, 12),  (0, 11),  (0, 10),  (0, 9),  (0, 8),  (0, 7),  (0, 6),  (0, 5),  (0, 4),  (0, 3),  (0, 2),  (0, 1),  (0, 0),
  (1, 14),  (1, 13),  (1, 12),  (1, 11),  (1, 10),  (1, 9),  (1, 8),  (1, 7),  (1, 6),  (1, 5),  (1, 4),  (1, 3),  (1, 2),  (1, 1),  (1, 0),
  (2, 14),  (2, 13),  (2, 12),  (2, 11),  (2, 10),  (2, 9),  (2, 8),  (2, 7),  (2, 6),  (2, 5),  (2, 4),  (2, 3),  (2, 2),  (2, 1),  (2, 0),
  (3, 14),  (3, 13),  (3, 12),  (3, 11),  (3, 10),  (3, 9),  (3, 8),  (3, 7),  (3, 6),  (3, 5),  (3, 4),  (3, 3),  (3, 2),  (3, 1),  (3, 0),
  (4, 14),  (4, 13),  (4, 12),  (4, 11),  (4, 10),  (4, 9),  (4, 8),  (4, 7),  (4, 6),  (4, 5),  (4, 4),  (4, 3),  (4, 2),  (4, 1),  (4, 0),
  (5, 14),  (5, 13),  (5, 12),  (5, 11),  (5, 10),  (5, 9),  (5, 8),  (5, 7),  (5, 6),  (5, 5),  (5, 4),  (5, 3),  (5, 2),  (5, 1),  (5, 0),
  (6, 14),  (6, 13),  (6, 12),  (6, 11),  (6, 10),  (6, 9),  (6, 8),  (6, 7),  (6, 6),  (6, 5),  (6, 4),  (6, 3),  (6, 2),  (6, 1),  (6, 0),
  (7, 14),  (7, 13),  (7, 12),  (7, 11),  (7, 10),  (7, 9),  (7, 8),  (7, 7),  (7, 6),  (7, 5),  (7, 4),  (7, 3),  (7, 2),  (7, 1),  (7, 0),
  (8, 14),  (8, 13),  (8, 12),  (8, 11),  (8, 10),  (8, 9),  (8, 8),  (8, 7),  (8, 6),  (8, 5),  (8, 4),  (8, 3),  (8, 2),  (8, 1),  (8, 0),
  (9, 14),  (9, 13),  (9, 12),  (9, 11),  (9, 10),  (9, 9),  (9, 8),  (9, 7),  (9, 6),  (9, 5),  (9, 4),  (9, 3),  (9, 2),  (9, 1),  (9, 0),
  (10, 14), (10, 13), (10, 12), (10, 11), (10, 10), (10, 9), (10, 8), (10, 7), (10, 6), (10, 5), (10, 4), (10, 3), (10, 2), (10, 1), (10, 0),
  (11, 14), (11, 13), (11, 12), (11, 11), (11, 10), (11, 9), (11, 8), (11, 7), (11, 6), (11, 5), (11, 4), (11, 3), (11, 2), (11, 1), (11, 0),
  (12, 14), (12, 13), (12, 12), (12, 11), (12, 10), (12, 9), (12, 8), (12, 7), (12, 6), (12, 5), (12, 4), (12, 3), (12, 2), (12, 1), (12, 0),
  (13, 14), (13, 13), (13, 12), (13, 11), (13, 10), (13, 9), (13, 8), (13, 7), (13, 6), (13, 5), (13, 4), (13, 3), (13, 2), (13, 1), (13, 0),
  (14, 14), (14, 13), (14, 12), (14, 11), (14, 10), (14, 9), (14, 8), (14, 7), (14, 6), (14, 5), (14, 4), (14, 3), (14, 2), (14, 1), (14, 0)
];

pub static VERTI: [(u8, u8); 225] = [
 (0, 14), (1, 14), (2, 14), (3, 14), (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14), (12, 14), (13, 14), (14, 14),
 (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13), (14, 13),
 (0, 12), (1, 12), (2, 12), (3, 12), (4, 12), (5, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12),
 (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11),
 (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10),
 (0, 09), (1, 09), (2, 09), (3, 09), (4, 09), (5, 09), (6, 09), (7, 09), (8, 09), (9, 09), (10, 09), (11, 09), (12, 09), (13, 09), (14, 09),
 (0, 08), (1, 08), (2, 08), (3, 08), (4, 08), (5, 08), (6, 08), (7, 08), (8, 08), (9, 08), (10, 08), (11, 08), (12, 08), (13, 08), (14, 08),
 (0, 07), (1, 07), (2, 07), (3, 07), (4, 07), (5, 07), (6, 07), (7, 07), (8, 07), (9, 07), (10, 07), (11, 07), (12, 07), (13, 07), (14, 07),
 (0, 06), (1, 06), (2, 06), (3, 06), (4, 06), (5, 06), (6, 06), (7, 06), (8, 06), (9, 06), (10, 06), (11, 06), (12, 06), (13, 06), (14, 06),
 (0, 05), (1, 05), (2, 05), (3, 05), (4, 05), (5, 05), (6, 05), (7, 05), (8, 05), (9, 05), (10, 05), (11, 05), (12, 05), (13, 05), (14, 05),
 (0, 04), (1, 04), (2, 04), (3, 04), (4, 04), (5, 04), (6, 04), (7, 04), (8, 04), (9, 04), (10, 04), (11, 04), (12, 04), (13, 04), (14, 04),
 (0, 03), (1, 03), (2, 03), (3, 03), (4, 03), (5, 03), (6, 03), (7, 03), (8, 03), (9, 03), (10, 03), (11, 03), (12, 03), (13, 03), (14, 03),
 (0, 02), (1, 02), (2, 02), (3, 02), (4, 02), (5, 02), (6, 02), (7, 02), (8, 02), (9, 02), (10, 02), (11, 02), (12, 02), (13, 02), (14, 02),
 (0, 01), (1, 01), (2, 01), (3, 01), (4, 01), (5, 01), (6, 01), (7, 01), (8, 01), (9, 01), (10, 01), (11, 01), (12, 01), (13, 01), (14, 01),
 (0, 00), (1, 00), (2, 00), (3, 00), (4, 00), (5, 00), (6, 00), (7, 00), (8, 00), (9, 00), (10, 00), (11, 00), (12, 00), (13, 00), (14, 00),
];