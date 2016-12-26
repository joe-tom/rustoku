pub fn all () {
  unsafe {
    // Build the BT Table.
    // Build the Move Table.
    binary_recurse(0,0,14);
  }

}`

/**
 * Recurses through all the places.
 */

unsafe fn binary_recurse(you: u16, opp: u16, depth: i32) {
  if depth < 0 {
    let you_ternary = from_str_radix(&format!("{:b}", you),3);
    let opp_ternary = from_str_radix(&format!("{:b}", opp),3);

    super::board::BT[you] = you_ternary;
    super::board::BT[opp] = opp_ternary;

    let state: u32 = ((2 * (opp as u32)) + (you as u32));

    build_move(you, opp, state);
    return;
  }

  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1);
  binary_recurse(you, opp, depth - 1);
}


/**
 * Builds WON_TABLE and STATE_TABLE together
 */
unsafe fn build_state(you: u16, opp: u16, state: u32) {
  let mut you_movs: Vec<(u8, i8)> = vec![];
  let mut opp_movs: Vec<(u8, i8)> = vec![];
  let you_cur_arr = super::board::MOVES[0][state as usize];
  let opp_cur_arr = super::board::MOVES[0][state as usize];

  for shift in 0..11u16{
    let you_state = (you >> shift) & 0b11111;
    let opp_state = (opp >> shift) & 0b11111;

    if you_state == 0b11111 {
      super::board::WON[state as usize] = 100;
      return;
    }
    if opp_state == 0b11111 {
      super::board::WON[state as usize] = -100;
      return;
    }

    if you_state == 0 {
      opp_movs.extend(get_five(you_state, shift, false));
    }
    if opp_state == 0 {
      you_movs.extend(get_five(opp_state, shift, true));
    }
  }

}

/**
 * Accepts a binary, returns a vector of tuples, with moves and urgency.
 */
fn get_five(binary: u16, cur_shift: u16, you: bool) -> Vec<(u8, i8)>{
  let mut movs: Vec<u8> = vec![];
  let multiple = if you {1} else {-1};

  for shift in 0..5 {
    if (binary >> shift) & 1 == 0 {
      movs.push(shift + cur_shift); 
    }
  }

  let value = movs.len() * 5 * multiple;
  let mov_urg: Vec<(u8, i8)>;

  for mov in &movs {
    mov_urg.push(mov, value);
  }

  return mov_urg;
}