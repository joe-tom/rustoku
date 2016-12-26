pub fn all () {
  unsafe {
    // Build the BT Table.
    // Build the Move Table.
    binary_recurse(0,0,14);
  }

}

unsafe fn binary_recurse(you: u16, opp: u16, depth: i32) {
  if depth < 0 {
    let you_ternary = from_str_radix(&format!("{:b}", you),3);
    let opp_ternary = from_str_radix(&format!("{:b}", opp),3);

    super::board::BT[you] = you_ternary;
    super::board::BT[opp] = opp_ternary;

    let state = ((2 * (opp as u32)) + (you as u32));

    build_move(you, opp, state);
    return;
  }

  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1);
  binary_recurse(you, opp, depth - 1);
}

unsafe fn build_state(you: u16, opp: u16, state: u32) {

}