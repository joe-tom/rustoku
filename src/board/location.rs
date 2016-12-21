pub static mut WON: [i16; 65536] = [0; 65536];
/* FUNCTION FOR DIAGONAL MOVES */
pub const NUL: u8 = 240;

pub static mut HORIZ_LOOKUP: [(u8,u8); 230] = [(NUL,0); 230];
pub static mut VERTI_LOOKUP: [(u8,u8); 230] = [(NUL,0); 230];
pub static mut DIAGR_LOOKUP: [(u8,u8); 230] = [(NUL,0); 230];
pub static mut DIAGL_LOOKUP: [(u8,u8); 230] = [(NUL,0); 230];

pub static DIAGL: [[u8; 16]; 21] = [
    [060, 046, 032, 018, 004, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [075, 061, 047, 033, 019, 005, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [090, 076, 062, 048, 034, 020, 006, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [105, 091, 077, 063, 049, 035, 021, 007, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [120, 106, 092, 078, 064, 050, 036, 022, 008, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [135, 121, 107, 093, 079, 065, 051, 037, 023, 009, NUL, NUL, NUL, NUL, NUL, NUL],
    [150, 136, 122, 108, 094, 080, 066, 052, 038, 024, 010, NUL, NUL, NUL, NUL, NUL],
    [165, 151, 137, 123, 109, 095, 081, 067, 053, 039, 025, 011, NUL, NUL, NUL, NUL],
    [180, 166, 152, 138, 124, 110, 096, 082, 068, 054, 040, 026, 012, NUL, NUL, NUL],
    [195, 181, 167, 153, 139, 125, 111, 097, 083, 069, 055, 041, 027, 013, NUL, NUL],
    [210, 196, 182, 168, 154, 140, 126, 112, 098, 084, 070, 056, 042, 028, 014, NUL],
    [211, 197, 183, 169, 155, 141, 127, 113, 099, 085, 071, 057, 043, 029, NUL, NUL],
    [212, 198, 184, 170, 156, 142, 128, 114, 100, 086, 072, 058, 044, NUL, NUL, NUL],
    [213, 199, 185, 171, 157, 143, 129, 115, 101, 087, 073, 059, NUL, NUL, NUL, NUL],
    [214, 200, 186, 172, 158, 144, 130, 116, 102, 088, 074, NUL, NUL, NUL, NUL, NUL],
    [215, 201, 187, 173, 159, 145, 131, 117, 103, 089, NUL, NUL, NUL, NUL, NUL, NUL],
    [216, 202, 188, 174, 160, 146, 132, 118, 104, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [217, 203, 189, 175, 161, 147, 133, 119, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [218, 204, 190, 176, 162, 148, 134, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [219, 205, 191, 177, 163, 149, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [220, 206, 192, 178, 164, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL]
];

pub static DIAGR: [[u8; 16]; 21] = [
    [010, 026, 042, 058, 074, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [009, 025, 041, 057, 073, 089, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [008, 024, 040, 056, 072, 088, 104, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [007, 023, 039, 055, 071, 087, 103, 119, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [006, 022, 038, 054, 070, 086, 102, 118, 134, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [005, 021, 037, 053, 069, 085, 101, 117, 133, 149, NUL, NUL, NUL, NUL, NUL, NUL],
    [004, 020, 036, 052, 068, 084, 100, 116, 132, 148, 164, NUL, NUL, NUL, NUL, NUL],
    [003, 019, 035, 051, 067, 083, 099, 115, 131, 147, 163, 179, NUL, NUL, NUL, NUL],
    [002, 018, 034, 050, 066, 082, 098, 114, 130, 146, 162, 178, 194, NUL, NUL, NUL],
    [001, 017, 033, 049, 065, 081, 097, 113, 129, 145, 161, 177, 193, 209, NUL, NUL],
    [000, 016, 032, 048, 064, 080, 096, 112, 128, 144, 160, 176, 192, 208, 224, NUL],
    [015, 031, 047, 063, 079, 095, 111, 127, 143, 159, 175, 191, 207, 223, NUL, NUL],
    [030, 046, 062, 078, 094, 110, 126, 142, 158, 174, 190, 206, 222, NUL, NUL, NUL],
    [045, 061, 077, 093, 109, 125, 141, 157, 173, 189, 205, 221, NUL, NUL, NUL, NUL],
    [060, 076, 092, 108, 124, 140, 156, 172, 188, 204, 220, NUL, NUL, NUL, NUL, NUL],
    [075, 091, 107, 123, 139, 155, 171, 187, 203, 219, NUL, NUL, NUL, NUL, NUL, NUL],
    [090, 106, 122, 138, 154, 170, 186, 202, 218, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [105, 121, 137, 153, 169, 185, 201, 217, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [120, 136, 152, 168, 184, 200, 216, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [135, 151, 167, 183, 199, 215, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL],
    [150, 166, 182, 198, 214, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL]
];


pub static HORIZ: [[u8; 16]; 15] = [
    [000, 001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, NUL],
    [015, 016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, NUL],
    [030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044, NUL],
    [045, 046, 047, 048, 049, 050, 051, 052, 053, 054, 055, 056, 057, 058, 059, NUL],
    [060, 061, 062, 063, 064, 065, 066, 067, 068, 069, 070, 071, 072, 073, 074, NUL],
    [075, 076, 077, 078, 079, 080, 081, 082, 083, 084, 085, 086, 087, 088, 089, NUL],
    [090, 091, 092, 093, 094, 095, 096, 097, 098, 099, 100, 101, 102, 103, 104, NUL],
    [105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, NUL],
    [120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, NUL],
    [135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, NUL],
    [150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, NUL],
    [165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, NUL],
    [180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, NUL],
    [195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, NUL],
    [210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, NUL]
];

pub static VERTI: [[u8; 16]; 15] = [
    [000, 015, 030, 045, 060, 075, 090, 105, 120, 135, 150, 165, 180, 195, 210, NUL],
    [001, 016, 031, 046, 061, 076, 091, 106, 121, 136, 151, 166, 181, 196, 211, NUL],
    [002, 017, 032, 047, 062, 077, 092, 107, 122, 137, 152, 167, 182, 197, 212, NUL],
    [003, 018, 033, 048, 063, 078, 093, 108, 123, 138, 153, 168, 183, 198, 213, NUL],
    [004, 019, 034, 049, 064, 079, 094, 109, 124, 139, 154, 169, 184, 199, 214, NUL],
    [005, 020, 035, 050, 065, 080, 095, 110, 125, 140, 155, 170, 185, 200, 215, NUL],
    [006, 021, 036, 051, 066, 081, 096, 111, 126, 141, 156, 171, 186, 201, 216, NUL],
    [007, 022, 037, 052, 067, 082, 097, 112, 127, 142, 157, 172, 187, 202, 217, NUL],
    [008, 023, 038, 053, 068, 083, 098, 113, 128, 143, 158, 173, 188, 203, 218, NUL],
    [009, 024, 039, 054, 069, 084, 099, 114, 129, 144, 159, 174, 189, 204, 219, NUL],
    [010, 025, 040, 055, 070, 085, 100, 115, 130, 145, 160, 175, 190, 205, 220, NUL],
    [011, 026, 041, 056, 071, 086, 101, 116, 131, 146, 161, 176, 191, 206, 221, NUL],
    [012, 027, 042, 057, 072, 087, 102, 117, 132, 147, 162, 177, 192, 207, 222, NUL],
    [013, 028, 043, 058, 073, 088, 103, 118, 133, 148, 163, 178, 193, 208, 223, NUL],
    [014, 029, 044, 059, 074, 089, 104, 119, 134, 149, 164, 179, 194, 209, 224, NUL]
];







pub const MASKS: [u32; 12] = [
    0b0000000000011111,
    0b0000000000111110,
    0b0000000001111100,
    0b0000000011111000,
    0b0000000111110000,
    0b0000001111100000,
    0b0000011111000000,
    0b0000111110000000,
    0b0001111100000000,
    0b0011111000000000,
    0b0111110000000000,
    0b1111100000000000
];


pub fn build () {
    unsafe {
        // For Verti and Horiz
        for r in 0..15usize {
            for c in 0..15usize {
                HORIZ_LOOKUP[HORIZ[r][c] as usize] = (r as u8,c as u8);
                VERTI_LOOKUP[VERTI[r][c] as usize] = (r as u8,c as u8);
            }
        }
        
        // For Diagr and Diagl
        for r in 0..21usize {
            for c in 0..15usize {
                if DIAGR[r][c] != NUL {
                    DIAGR_LOOKUP[DIAGR[r][c] as usize] = (r as u8,c as u8);
                }
                if DIAGL[r][c] != NUL {
                    DIAGL_LOOKUP[DIAGL[r][c] as usize] = (r as u8,c as u8);
                }
            }
        }


        // Create the WON table
        for i in 0..65535u32{
            for m in 0..12 {
                if (i&MASKS[m as usize]) == MASKS[m as usize] {
                    WON[i as usize] = 10;
                    break;
                }
            }
        }
        // Create the Move table
        move_recurse(0,0,15);
    }
}



pub fn move_recurse(you: u32, opp: u32, depth: u32) {
    if depth == 0 {
        unsafe{
            eval(you, opp);
        }
        return;
    }
    move_recurse(you | 1 << (depth - 1), opp , depth - 1);
    move_recurse(you , opp | 1 << (depth - 1), depth - 1);
    move_recurse(you , opp, depth - 1);
}

unsafe fn eval(you: u32, opp: u32) {
    // Let's build the first part
    let you_val = u32::from_str_radix(&format!("{:b}", you), 3).unwrap();
    super::B_T[you as usize] = you_val;
    let opp_val = u32::from_str_radix(&format!("{:b}", opp), 3).unwrap();
    super::B_T[opp as usize] = opp_val;

    // This is for the next array;
    let state = (you_val + (2 * opp_val));
    
    // Let's build the movepart
    let mut you_moves = vec![];
    let mut opp_moves = vec![];

    // Get rid of WON positions
    if WON[you as usize] != 0 || WON[opp as usize] != 0 {
        super::MOV[0][state as usize] = [15u8 ; 15];
        super::MOV[1][state as usize] = [15u8 ; 15];
        return;
    }

    // First you:
    let mut shift:u32 = 0;

/*
    THE SCANNING THING....
 
    0b 0000000000[00000] i = 0
    0b 000000000[00000]0 i = 1
    0b 00000000[00000]00 i = 2
    0b 0000000[00000]000 i = 3
    0b 000000[00000]0000 i = 4
    0b 00000[00000]00000 i = 5
    0b 0000[00000]000000 i = 6
    0b 000[00000]0000000 i = 7
    0b 00[00000]00000000 i = 8
    0b 0[00000]000000000 i = 9
    0b [00000]0000000000 i = 10
    0b 0000]00000000000 i = 11
    0b 000]000000000000 i = 12
    0b 00]0000000000000 i = 13
    0b 0]00000000000000 i = 14

*/


    let mut you_max = 0;
    let mut opp_max = 0;

    for shift in 0..15u8 {
        let you_state = (you >> shift) & 0b11111;
        let opp_state = (opp >> shift) & 0b11111;

        if opp_state == 0 {
            match you_state {
                0b01111 => {
                    if you_max < 4 { if shift + 4 < 15 { you_max = 4;you_moves.truncate(0);you_moves.push(shift + 4);if shift + 4 >= 15 {you_moves.push(shift + 4);} }}
                    if you_max == 4 { if shift + 4 < 15 { you_moves.push(shift + 4); } }
                }
                0b11110 => {
                    if you_max < 4 { if shift < 15 { you_max = 4;you_moves.truncate(0);you_moves.push(shift);if shift >= 15 {you_moves.push(shift);} }}
                    if you_max == 4 { if shift < 15 { you_moves.push(shift); } }
                }
                0b00111 => {
                    if you_max < 3 { if shift + 3 < 15 { you_max = 3;you_moves.truncate(0);you_moves.push(shift + 3);if shift + 3 >= 15 {you_moves.push(shift + 3);} }}
                    if you_max == 3 { if shift + 3 < 15 { opp_moves.push(shift + 3); } }
                }
                0b01110 => {
                    if you_max < 3 { if shift < 15 { you_max = 3;you_moves.truncate(0);you_moves.push(shift);if shift >= 15 {you_moves.push(shift);} }}
                    if you_max == 3 { if shift < 15 { you_moves.push(shift); } }
                }
                0b00001 => {
                    if you_max <= 1 {you_moves.push(shift);you_max = 1;}
                }
                _ => {}
            }
        }

        if you_state == 0 {
            match opp_state {
                0b01111 => {
                    if opp_max < 4 { if  shift + 4 < 15 { opp_max = 4;opp_moves.truncate(0);opp_moves.push(shift + 4);if shift + 4 >= 15 {opp_moves.push(shift + 4);}  }}
                    if opp_max == 4 { if  shift + 4 < 15 { opp_moves.push(shift + 4); } }
                }
                0b11110 => {
                    if opp_max < 4 { if shift < 15 { opp_max = 4;opp_moves.truncate(0);opp_moves.push(shift);if shift >= 15 {opp_moves.push(shift);}  }}
                    if opp_max == 4 { if shift < 15 { opp_moves.push(shift); } }
                }
                0b00111 => {
                    if opp_max < 3 { if shift + 3 < 15 { opp_max = 3;opp_moves.truncate(0);opp_moves.push(shift + 3);if shift + 3 >= 15 {opp_moves.push(shift + 3);}  }}
                    if opp_max == 3 { if shift + 3 < 15 { opp_moves.push(shift + 3); } }
                }
                0b01110 => {
                    if opp_max < 3 { if shift < 15 { opp_max = 3;opp_moves.truncate(0);opp_moves.push(shift);if shift >= 15 {opp_moves.push(shift);}  }}
                    if opp_max == 3 { if shift < 15 { opp_moves.push(shift); } }
                }
                0b00001 => {
                    if opp_max <= 1 {opp_moves.push(shift);opp_max = 1;}
                }
                _ => {}
            }
        }
    }


    super::MOV[0][state as usize] = to_arr(you_moves);
    super::MOV[1][state as usize] = to_arr(opp_moves);
}

fn to_arr (moves: Vec<u8>) -> [u8; 15] {
    let mut arr = [15u8 ; 15];

    for (i, mov) in moves.iter().enumerate() {
        unsafe {
            arr[i] = *mov;
        }
    }

    return arr;
}