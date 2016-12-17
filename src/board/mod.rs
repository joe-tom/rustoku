use std::vec;

pub mod location;

pub static mut WON: [u32; 14348907] = [0; 14348907];
pub static mut B_T: [u32; 14348907] = [0; 14348907];
pub static mut MOV: [[[u8; 15]; 14348907]; 2] = [[[0; 15]; 14348907]; 2];


const M_1: u16 = 0b0000000000011111;
const M_2: u16 = 0b0000000000111110;
const M_3: u16 = 0b0000000001111100;
const M_4: u16 = 0b0000000011111000;
const M_5: u16 = 0b0000000111110000;
const M_6: u16 = 0b0000001111100000;
const M_7: u16 = 0b0000011111000000;
const M_8: u16 = 0b0000111110000000;
const M_9: u16 = 0b0001111100000000;
const M_10: u16 = 0b0011111000000000;
const M_11: u16 = 0b0111110000000000;
const M_12: u16 = 0b1111100000000000;


pub struct Board {
  pub horiz_y: [u16; 15],
  pub horiz_o: [u16; 15],

  pub verti_y: [u16; 15],
  pub verti_o: [u16; 15],
  
  pub diagr_y: [u16; 21],
  pub diagr_o: [u16; 21],
  
  pub diagl_y: [u16; 21],
  pub diagl_o: [u16; 21]
}

impl Board {
  pub fn gen_movs(&self, you: bool) -> Vec<u8> {
    let movs = Vec::new();
    if you {
      let mut urgency = 0;
      for i in 0..21usize {
        if i < 15 {
          let horiz_val = MOV[0][self.horiz_y[i] as usize];
          let verti_val = MOV[0][self.horiz_y[i] as usize];
        }
        let diagl_val = MOV[0][self.horiz_y[i] as usize];
        let diagr_val = MOV[0][self.horiz_y[i] as usize];
      }
    } else {
      let mut urgency = 0;
      for i in 0..21usize {
        if i < 15 {
          let horiz_val = MOV[1][self.horiz_o[i] as usize];
          let verti_val = MOV[1][self.horiz_o[i] as usize];
        }
        let diagl_val = MOV[1][self.horiz_o[i] as usize];
        let diagr_val = MOV[1][self.horiz_o[i] as usize];
      }
    }

    return movs;
  }

  pub fn evaluate (&self) -> i16 {
    unsafe {
      for i in 0..21usize {
        if i < 15 {
          let h_y = self.horiz_y[i];
          let h_o = self.horiz_o[i];

          let v_y = self.verti_y[i];
          let v_o = self.verti_o[i];

          if h_o != 0 && (h_o & M_1 == M_1||h_o & M_2 == M_2||h_o & M_3 == M_3||h_o & M_4 == M_4||h_o & M_5 == M_5||h_o & M_6 == M_6||h_o & M_7 == M_7||h_o & M_8 == M_8||h_o & M_9 == M_9||h_o & M_10 == M_10||h_o & M_11 == M_11||h_o & M_12 == M_12){
            return -100;
          }
          if h_y != 0 && (h_y & M_1 == M_1||h_y & M_2 == M_2||h_y & M_3 == M_3||h_y & M_4 == M_4||h_y & M_5 == M_5||h_y & M_6 == M_6||h_y & M_7 == M_7||h_y & M_8 == M_8||h_y & M_9 == M_9||h_y & M_10 == M_10||h_y & M_11 == M_11||h_y & M_12 == M_12){
            return 100;
          }

          if v_o != 0 && (v_o & M_1 == M_1||v_o & M_2 == M_2||v_o & M_3 == M_3||v_o & M_4 == M_4||v_o & M_5 == M_5||v_o & M_6 == M_6||v_o & M_7 == M_7||v_o & M_8 == M_8||v_o & M_9 == M_9||v_o & M_10 == M_10||v_o & M_11 == M_11||v_o & M_12 == M_12){
            return -100;
          }
          if v_y != 0 && (v_y & M_1 == M_1||v_y & M_2 == M_2||v_y & M_3 == M_3||v_y & M_4 == M_4||v_y & M_5 == M_5||v_y & M_6 == M_6||v_y & M_7 == M_7||v_y & M_8 == M_8||v_y & M_9 == M_9||v_y & M_10 == M_10||v_y & M_11 == M_11||v_y & M_12 == M_12){
            return 100;
          }
        }

        let r_y = self.diagr_y[i];
        let r_o = self.diagr_o[i];

        let l_y = self.diagl_y[i];
        let l_o = self.diagl_o[i];

        if r_o != 0 && (r_o & M_1 == M_1||r_o & M_2 == M_2||r_o & M_3 == M_3||r_o & M_4 == M_4||r_o & M_5 == M_5||r_o & M_6 == M_6||r_o & M_7 == M_7||r_o & M_8 == M_8||r_o & M_9 == M_9||r_o & M_10 == M_10||r_o & M_11 == M_11||r_o & M_12 == M_12){
            return -100;
        }
        if r_y != 0 && (r_y & M_1 == M_1||r_y & M_2 == M_2||r_y & M_3 == M_3||r_y & M_4 == M_4||r_y & M_5 == M_5||r_y & M_6 == M_6||r_y & M_7 == M_7||r_y & M_8 == M_8||r_y & M_9 == M_9||r_y & M_10 == M_10||r_y & M_11 == M_11||r_y & M_12 == M_12){
          return 100;
        }

        if l_o != 0 && (l_o & M_1 == M_1||l_o & M_2 == M_2||l_o & M_3 == M_3||l_o & M_4 == M_4||l_o & M_5 == M_5||l_o & M_6 == M_6||l_o & M_7 == M_7||l_o & M_8 == M_8||l_o & M_9 == M_9||l_o & M_10 == M_10||l_o & M_11 == M_11||l_o & M_12 == M_12){
            return -100;
        }
        if l_y != 0 && (l_y & M_1 == M_1||l_y & M_2 == M_2||l_y & M_3 == M_3||l_y & M_4 == M_4||l_y & M_5 == M_5||l_y & M_6 == M_6||l_y & M_7 == M_7||l_y & M_8 == M_8||l_y & M_9 == M_9||l_y & M_10 == M_10||l_y & M_11 == M_11||l_y & M_12 == M_12){
          return 100;
        }
        
      }
    }
    return 0;
  }

  pub fn make_mov (&mut self, place: u8, you: bool) {
    // Assemble bitwise rep of the board
    unsafe {
      self.mov_horiz(place, you);
      self.mov_verti(place, you);
      self.mov_diagr(place, you);
      self.mov_diagl(place, you);
    }
  }

  unsafe fn mov_horiz (&mut self, place: u8, you: bool) {
    let a = location::HORIZ_LOOKUP[place as usize];
    if you {
      self.horiz_y[a.0 as usize] |= 1 << a.1;
    } else {
      self.horiz_o[a.0 as usize] |= 1 << a.1;
    }
  }
  unsafe fn mov_verti (&mut self, place: u8, you: bool) {
    let a = location::VERTI_LOOKUP[place as usize];
    if you {
      self.verti_y[a.0 as usize] |= 1 << a.1;
    } else {
      self.verti_o[a.0 as usize] |= 1 << a.1;
    }
  }
  unsafe fn mov_diagr (&mut self, place: u8, you: bool) {
    let a = location::DIAGR_LOOKUP[place as usize];
    if a.0 == location::NUL {
      return;
    }
    if you {
      self.diagr_y[a.0 as usize] |= 1 << a.1;
    } else {
      self.diagr_o[a.0 as usize] |= 1 << a.1;
    }
  }
  unsafe fn mov_diagl (&mut self, place: u8, you: bool) {
    let a = location::DIAGL_LOOKUP[place as usize];
    if a.0 == location::NUL {
      return;
    }
    if you {
      self.diagl_y[a.0 as usize] |= 1 << a.1;
    } else {
      self.diagl_o[a.0 as usize] |= 1 << a.1;
    }
  }

  pub fn undo_mov (&mut self, place: u8, you: bool) {
    // Assemble bitwise rep of the board
    unsafe {
      self.undo_horiz(place, you);
      self.undo_verti(place, you);
      self.undo_diagr(place, you);
      self.undo_diagl(place, you);
    }
  }

  unsafe fn undo_horiz (&mut self, place: u8, you: bool) {
    let a = location::HORIZ_LOOKUP[place as usize];
    if you {
      self.horiz_y[a.0 as usize] ^= 1 << a.1;
    } else {
      self.horiz_o[a.0 as usize] ^= 1 << a.1;
    }
  }
  unsafe fn undo_verti (&mut self, place: u8, you: bool) {
    let a = location::VERTI_LOOKUP[place as usize];
    if you {
      self.verti_y[a.0 as usize] ^= 1 << a.1;
    } else {
      self.verti_o[a.0 as usize] ^= 1 << a.1;
    }
  }
  unsafe fn undo_diagr (&mut self, place: u8, you: bool) {
    let a = location::DIAGR_LOOKUP[place as usize];
    if a.0 == location::NUL {
      return;
    }
    if you {
      self.diagr_y[a.0 as usize] ^= 1 << a.1;
    } else {
      self.diagr_o[a.0 as usize] ^= 1 << a.1;
    }
  }
  unsafe fn undo_diagl (&mut self, place: u8, you: bool) {
    let a = location::DIAGL_LOOKUP[place as usize];
    if a.0 == location::NUL {
      return;
    }
    if you {
      self.diagl_y[a.0 as usize] ^= 1 << a.1;
    } else {
      self.diagl_o[a.0 as usize] ^= 1 << a.1;
    }
  }
}





  


