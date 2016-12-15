pub static mut EVAL: [u32; 14348907] = [0; 14348907];
pub static mut B_T: [u32; 14348907] = [0; 14348907];
pub static mut MOVE: [[[u32; 15]; 14348907];2] = [[[0; 15]; 14348907];2];



struct Board {
  horiz: [u32; 15],
  verti: [u32; 15],
  diagr: [u32; 21],
  diagl: [u32; 21]
}


impl Move for Board {





  /* Evaluating the move */



  /* Generating moves */
  pub fn gen_moves (&self, you: bool) -> vec<u32> {
    let mut urgency = 0;
    let mut moves: Vec<u32> = Vec::new();
    if you {
      for i in 0..21 {
        if i <= 15 {
          let horiz_state = self.horiz[i].iter();
          let horiz_urgency = horiz_state.next();

          if horiz_urgency == urgency {
            moves.extend(horiz_urgency);
          }
          if horiz_urgency > urgency {
            urgency = horiz_urgency;
            moves.truncate(0);
            moves.extend(horiz_urgency);
          }

          let verti_state = self.verti[i]`.iter();
          let verti_urgency = verti_state.next();

          if verti_urgency == urgency {
            moves.extend(verti_urgency);
          }
          if verti_urgency > urgency {
            urgency = verti_urgency;
            moves.truncate(0);
            moves.extend(verti_urgency);
          }
        }
      }
    } else {
      for i in 0..21 {
        if i <= 15 {
          let horiz_state = self.horiz[i].iter();
          let horiz_urgency = horiz_state.next();

          if horiz_urgency == urgency {
            moves.extend(horiz_urgency);
          }
          if horiz_urgency > urgency {
            urgency = horiz_urgency;
            moves.truncate(0);
            moves.extend(horiz_urgency);
          }

          let verti_state = self.verti[i].iter();
          let verti_urgency = verti_state.next();

          if verti_urgency == urgency {
            moves.extend(verti_urgency);
          }
          if verti_urgency > urgency {
            urgency = verti_urgency;
            moves.truncate(0);
            moves.extend(verti_urgency);
          }

        }
      }
    }
  }

  /* The moving related stuff */
  pub fn make_move (&self, mov: u32, you: bool) {
    self.mov_horiz(mov, you);
    self.mov_verti(mov, you);
    self.mov_diagr(mov, you);
    self.mov_diagl(mov, you);
  }


  fn mov_horiz (&mut self, mov: u32, you: bool) {
    let mov_tup = MOVE_HORIZ[mov];
    if mov_tup.0 == NULL { return; }
    if you {
      self.horiz[mov_tup.0] |= 1 << mov_tup.1; 
    } else {
      self.horiz[mov_tup.0] |= 1 << (mov_tup.1 + 15);
    }
  }

  fn mov_verti (&mut self, mov: u32, you: bool) {
    let mov_tup = MOVE_VERTI[mov];
    if mov_tup.0 == NULL { return; }
    if you {
      self.verti[mov_tup.0] |= 1 << mov_tup.1; 
    } else {
      self.verti[mov_tup.0] |= 1 << (mov_tup.1 + 15);
    }
  }

  fn mov_diagr (&mut self, mov: u32, you: bool) {
    let mov_tup = MOVE_DIAGR[mov];
    if mov_tup.0 == NULL { return; }
    if you {
      self.diagr[mov_tup.0] |= 1 << mov_tup.1; 
    } else {
      self.diagr[mov_tup.0] |= 1 << (mov_tup.1 + 15);
    }
  }

  fn mov_diagl (&mut self, mov: u32, you: bool) {
    let mov_tup = MOVE_DIAGL[mov];
    if mov_tup.0 == NULL { return; }
    if you {
      self.diagl[mov_tup.0] |= 1 << mov_tup.1; 
    } else {
      self.diagl[mov_tup.0] |= 1 << (mov_tup.1 + 15);
    }
  }

}







  