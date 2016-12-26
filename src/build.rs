pub fn all () {
  unsafe {
    // Build the BT Table.
    println!("Building the Binary - Ternary Table");
    for state in 0..65536{
      super::board::BT[state as usize] = u32::from_str_radix(&format!("{:b}", state),3).unwrap();
    }
    // Build the Move Table.
    binary_recurse(0,0,14);
  }
}


/**
 * Recurses through all the places.
 */
pub static mut counter:u32 = 0;
unsafe fn binary_recurse(you: u16, opp: u16, depth: i32) {
  
  if depth < 0 {
    counter += 1;
    if counter % 100000 == 0{
      println!("{:?}", counter);
    }
    let state: u32 = ((2 * (super::board::BT[opp as usize] as u32)) + (super::board::BT[you as usize] as u32));
    build_state(you, opp, state as usize);
    return;
  }

  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1);
  binary_recurse(you, opp, depth - 1);
}


/**
 * Builds WON_TABLE and STATE_TABLE together
 */
unsafe fn build_state(you: u16, opp: u16, state: usize) {
  let mut you_movs: Vec<(u8, i8)> = vec![];
  let mut opp_movs: Vec<(u8, i8)> = vec![];
  let mut you_cur_arr = &mut super::board::MOVES[0][state];
  let mut opp_cur_arr = &mut super::board::MOVES[1][state];

  for shift in 0..11u16{
    let you_state = (you >> shift) & 0b11111;
    let opp_state = (opp >> shift) & 0b11111;
    if you_state == 0 && opp_state == 0 {
      continue;
    }
    if you_state == 0b11111 {
      super::board::WON[state] = 100;
      return;
    }
    if opp_state == 0b11111 {
      super::board::WON[state] = -100;
      return;
    }

    if you_state == 0 {
      opp_movs.extend(get_five(opp_state, shift, false));
    }
    if opp_state == 0 {
      you_movs.extend(get_five(you_state, shift, true));
    }
  }
  you_movs.dedup();
  opp_movs.dedup();
  
  let mut i:u8 = 0;
  for mov in &you_movs {
    if i == 30 {
      break;
    }
    you_cur_arr[i as usize] = (*mov).0 as i16;
    you_cur_arr[(i + 1) as usize] = (*mov).1 as i16;
    i += 2;
  }
  let mut i:u8 = 0;
  for mov in &opp_movs {
    if i == 30 {
      break;
    }
    opp_cur_arr[i as usize] = (*mov).0 as i16;
    opp_cur_arr[(i + 1) as usize] = (*mov).1 as i16;
    i += 2;
  }
}


/**
 * Accepts a binary, returns a vector of tuples, with moves and urgency.
 */
fn get_five(binary: u16, cur_shift: u16, you: bool) -> Vec<(u8, i8)>{
  let mut movs: Vec<u8> = vec![];
  let multiple: i8 = if you {1} else {-1};

  for shift in 0..5u16 {
    if (binary >> shift) & 1 == 0 {
      movs.push((shift + cur_shift) as u8); 
    }
  }
  
  let value = (movs.len() as i8) * 5 * multiple;
  let mut mov_urg: Vec<(u8, i8)> = vec![];

  for mov in &movs {
    mov_urg.push((*mov, value));
  }

  return mov_urg;
}