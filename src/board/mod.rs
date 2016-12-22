use std::vec;

pub mod location;

pub static mut B_T: [u32; 14348907] = [0; 14348907];
pub static mut MOV: [[u8; 15]; 32768] = [[15u8; 15]; 32768];

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
  pub unsafe fn gen_movs(&self, you: bool) -> Vec<u8> {
    let mut movs: Vec<u8> = vec![location::NUL];
    
    for i in 0..15usize { 
        let tree = location::HORIZ[i];
        let mut horiz_val: Vec<u8> = MOV[(self.horiz_y[i] | self.horiz_o[i]) as usize].iter().map(|x| tree[*x as usize]).collect(); 
        movs.extend(horiz_val);
    }

    // Get rid of the duplicates 
    movs.sort();
    movs.dedup();
    movs.pop();

    return movs;
  }

  pub fn evaluate (&self) -> i16 {
    unsafe {
      for i in 0..21usize {
        if i < 15 {
          if location::WON[self.horiz_y[i] as usize] != 0 { return 100; }
          if location::WON[self.horiz_o[i] as usize] != 0 { return -100; }
          if location::WON[self.verti_y[i] as usize] != 0 { return 100; }
          if location::WON[self.verti_o[i] as usize] != 0 { return -100; }
        }

        if location::WON[self.diagr_y[i] as usize] != 0 { return 100; }
        if location::WON[self.diagr_o[i] as usize] != 0 { return -100; }
        if location::WON[self.diagl_y[i] as usize] != 0 { return 100; }
        if location::WON[self.diagl_o[i] as usize] != 0 { return -100; }
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





  


