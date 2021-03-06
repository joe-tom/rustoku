// The array that allows binary to ternary conversion
pub static mut BT: [u32; 65536] = [0; 65536];
pub static mut BT2: [u32; 65536] = [0; 65536];

// The arrays for move lookup
pub static mut MAX_MOVES: [[(u8,u8); 15]; 14348907] = [[(0,0); 15]; 14348907];
pub static mut MIN_MOVES: [[(u8,u8); 15]; 14348907] = [[(0,0); 15]; 14348907];

pub static mut VALUES: [i32; 14348907] = [0; 14348907]; 

// The array for win lookup
pub static mut WON: [u8; 65536] = [0; 65536]; 

// To see if this even has moves
pub static mut ENABLED: [[bool; 2]; 14348907] = [[false; 2]; 14348907];
pub static mut MAX_ENABLED: [bool; 14348907] = [false; 14348907];
pub static mut MIN_ENABLED: [bool; 14348907] = [false; 14348907];

// List of constants
pub const FIVE_FLAG: u8 = 0b100_0000_0000;
pub const FOUR_FLAG: u8 = 0b010_0000_0000;

pub const YOU_WON: i32 = 20000;
pub const OPP_WON: i32 = -20000;

pub const EOL:u8 = 128;

//This is the most important array in the entire game.
// Array positions:
// 0: This is useless. Value can be none
// 1: This means this square completes a 5. Most important!
// 2: This means that this square creates a 4. Not as important as the last one, but still, important.
// 3: This square creates a 3. Not as important, but a little important.
// 4: This square creates a 2.
pub const MAX_VALS: [i16; 5] = [0,1000,100,40,10];
pub const MIN_VALS: [i16; 5] = [0,-1000,-100,-40,-10];

// The board in its current state
pub struct Board {
  pub multi: [char; 225],

  pub horiz_y: [u16; 21],
  pub horiz_o: [u16; 21],
  pub verti_y: [u16; 21],
  pub verti_o: [u16; 21],

  pub diagr_y: [u16; 21],
  pub diagr_o: [u16; 21],
  pub diagl_y: [u16; 21],
  pub diagl_o: [u16; 21]
}


pub const NIL: u8 = 240;
pub static HORIZ: [(usize, usize); 225] = [
  (00,14),(00,13),(00,12),(00,11),(00,10),(00,09),(00,08),(00,07),(00,06),(00,05),(00,04),(00,03),(00,02),(00,01),(00,00),
  (01,14),(01,13),(01,12),(01,11),(01,10),(01,09),(01,08),(01,07),(01,06),(01,05),(01,04),(01,03),(01,02),(01,01),(01,00),
  (02,14),(02,13),(02,12),(02,11),(02,10),(02,09),(02,08),(02,07),(02,06),(02,05),(02,04),(02,03),(02,02),(02,01),(02,00),
  (03,14),(03,13),(03,12),(03,11),(03,10),(03,09),(03,08),(03,07),(03,06),(03,05),(03,04),(03,03),(03,02),(03,01),(03,00),
  (04,14),(04,13),(04,12),(04,11),(04,10),(04,09),(04,08),(04,07),(04,06),(04,05),(04,04),(04,03),(04,02),(04,01),(04,00),
  (05,14),(05,13),(05,12),(05,11),(05,10),(05,09),(05,08),(05,07),(05,06),(05,05),(05,04),(05,03),(05,02),(05,01),(05,00),
  (06,14),(06,13),(06,12),(06,11),(06,10),(06,09),(06,08),(06,07),(06,06),(06,05),(06,04),(06,03),(06,02),(06,01),(06,00),
  (07,14),(07,13),(07,12),(07,11),(07,10),(07,09),(07,08),(07,07),(07,06),(07,05),(07,04),(07,03),(07,02),(07,01),(07,00),
  (08,14),(08,13),(08,12),(08,11),(08,10),(08,09),(08,08),(08,07),(08,06),(08,05),(08,04),(08,03),(08,02),(08,01),(08,00),
  (09,14),(09,13),(09,12),(09,11),(09,10),(09,09),(09,08),(09,07),(09,06),(09,05),(09,04),(09,03),(09,02),(09,01),(09,00),
  (10,14),(10,13),(10,12),(10,11),(10,10),(10,09),(10,08),(10,07),(10,06),(10,05),(10,04),(10,03),(10,02),(10,01),(10,00),
  (11,14),(11,13),(11,12),(11,11),(11,10),(11,09),(11,08),(11,07),(11,06),(11,05),(11,04),(11,03),(11,02),(11,01),(11,00),
  (12,14),(12,13),(12,12),(12,11),(12,10),(12,09),(12,08),(12,07),(12,06),(12,05),(12,04),(12,03),(12,02),(12,01),(12,00),
  (13,14),(13,13),(13,12),(13,11),(13,10),(13,09),(13,08),(13,07),(13,06),(13,05),(13,04),(13,03),(13,02),(13,01),(13,00),
  (14,14),(14,13),(14,12),(14,11),(14,10),(14,09),(14,08),(14,07),(14,06),(14,05),(14,04),(14,03),(14,02),(14,01),(14,00)
];

pub static VERTI: [(usize, usize); 225] = [
  (00,14),(01,14),(02,14),(03,14),(04,14),(05,14),(06,14),(07,14),(08,14),(09,14),(10,14),(11,14),(12,14),(13,14),(14,14),
  (00,13),(01,13),(02,13),(03,13),(04,13),(05,13),(06,13),(07,13),(08,13),(09,13),(10,13),(11,13),(12,13),(13,13),(14,13),
  (00,12),(01,12),(02,12),(03,12),(04,12),(05,12),(06,12),(07,12),(08,12),(09,12),(10,12),(11,12),(12,12),(13,12),(14,12),
  (00,11),(01,11),(02,11),(03,11),(04,11),(05,11),(06,11),(07,11),(08,11),(09,11),(10,11),(11,11),(12,11),(13,11),(14,11),
  (00,10),(01,10),(02,10),(03,10),(04,10),(05,10),(06,10),(07,10),(08,10),(09,10),(10,10),(11,10),(12,10),(13,10),(14,10),
  (00,09),(01,09),(02,09),(03,09),(04,09),(05,09),(06,09),(07,09),(08,09),(09,09),(10,09),(11,09),(12,09),(13,09),(14,09),
  (00,08),(01,08),(02,08),(03,08),(04,08),(05,08),(06,08),(07,08),(08,08),(09,08),(10,08),(11,08),(12,08),(13,08),(14,08),
  (00,07),(01,07),(02,07),(03,07),(04,07),(05,07),(06,07),(07,07),(08,07),(09,07),(10,07),(11,07),(12,07),(13,07),(14,07),
  (00,06),(01,06),(02,06),(03,06),(04,06),(05,06),(06,06),(07,06),(08,06),(09,06),(10,06),(11,06),(12,06),(13,06),(14,06),
  (00,05),(01,05),(02,05),(03,05),(04,05),(05,05),(06,05),(07,05),(08,05),(09,05),(10,05),(11,05),(12,05),(13,05),(14,05),
  (00,04),(01,04),(02,04),(03,04),(04,04),(05,04),(06,04),(07,04),(08,04),(09,04),(10,04),(11,04),(12,04),(13,04),(14,04),
  (00,03),(01,03),(02,03),(03,03),(04,03),(05,03),(06,03),(07,03),(08,03),(09,03),(10,03),(11,03),(12,03),(13,03),(14,03),
  (00,02),(01,02),(02,02),(03,02),(04,02),(05,02),(06,02),(07,02),(08,02),(09,02),(10,02),(11,02),(12,02),(13,02),(14,02),
  (00,01),(01,01),(02,01),(03,01),(04,01),(05,01),(06,01),(07,01),(08,01),(09,01),(10,01),(11,01),(12,01),(13,01),(14,01),
  (00,00),(01,00),(02,00),(03,00),(04,00),(05,00),(06,00),(07,00),(08,00),(09,00),(10,00),(11,00),(12,00),(13,00),(14,00),
];



const NO:usize = 15;
pub static HORIZ_ARRS:[[u8; 15]; 15] = [
  [000,001,002,003,004,005,006,007,008,009,010,011,012,013,014],
  [015,016,017,018,019,020,021,022,023,024,025,026,027,028,029],
  [030,031,032,033,034,035,036,037,038,039,040,041,042,043,044],
  [045,046,047,048,049,050,051,052,053,054,055,056,057,058,059],
  [060,061,062,063,064,065,066,067,068,069,070,071,072,073,074],
  [075,076,077,078,079,080,081,082,083,084,085,086,087,088,089],
  [090,091,092,093,094,095,096,097,098,099,100,101,102,103,104],
  [105,106,107,108,109,110,111,112,113,114,115,116,117,118,119],
  [120,121,122,123,124,125,126,127,128,129,130,131,132,133,134],
  [135,136,137,138,139,140,141,142,143,144,145,146,147,148,149],
  [150,151,152,153,154,155,156,157,158,159,160,161,162,163,164],
  [165,166,167,168,169,170,171,172,173,174,175,176,177,178,179],
  [180,181,182,183,184,185,186,187,188,189,190,191,192,193,194],
  [195,196,197,198,199,200,201,202,203,204,205,206,207,208,209],
  [210,211,212,213,214,215,216,217,218,219,220,221,222,223,224]
];

pub static VERTI_ARRS:[[u8; 15]; 15] = [
  [000,015,030,045,060,075,090,105,120,135,150,165,180,195,210],
  [001,016,031,046,061,076,091,106,121,136,151,166,181,196,211],
  [002,017,032,047,062,077,092,107,122,137,152,167,182,197,212],
  [003,018,033,048,063,078,093,108,123,138,153,168,183,198,213],
  [004,019,034,049,064,079,094,109,124,139,154,169,184,199,214],
  [005,020,035,050,065,080,095,110,125,140,155,170,185,200,215],
  [006,021,036,051,066,081,096,111,126,141,156,171,186,201,216],
  [007,022,037,052,067,082,097,112,127,142,157,172,187,202,217],
  [008,023,038,053,068,083,098,113,128,143,158,173,188,203,218],
  [009,024,039,054,069,084,099,114,129,144,159,174,189,204,219],
  [010,025,040,055,070,085,100,115,130,145,160,175,190,205,220],
  [011,026,041,056,071,086,101,116,131,146,161,176,191,206,221],
  [012,027,042,057,072,087,102,117,132,147,162,177,192,207,222],
  [013,028,043,058,073,088,103,118,133,148,163,178,193,208,223],
  [014,029,044,059,074,089,104,119,134,149,164,179,194,209,224]
];

pub static DIAGL: [(usize, usize); 225] = [
  (NO,NO),(NO,NO),(NO,NO),(NO,NO),(00,00),(01,00),(02,00),(03,00),(04,00),(05,00),(06,00),(07,00),(08,00),(09,00),(10,00),
  (NO,NO),(NO,NO),(NO,NO),(00,01),(01,01),(02,01),(03,01),(04,01),(05,01),(06,01),(07,01),(08,01),(09,01),(10,01),(11,00),
  (NO,NO),(NO,NO),(00,02),(01,02),(02,02),(03,02),(04,02),(05,02),(06,02),(07,02),(08,02),(09,02),(10,02),(11,01),(12,00),
  (NO,NO),(00,03),(01,03),(02,03),(03,03),(04,03),(05,03),(06,03),(07,03),(08,03),(09,03),(10,03),(11,02),(12,01),(13,00),
  (00,04),(01,04),(02,04),(03,04),(04,04),(05,04),(06,04),(07,04),(08,04),(09,04),(10,04),(11,03),(12,02),(13,01),(14,00),
  (01,05),(02,05),(03,05),(04,05),(05,05),(06,05),(07,05),(08,05),(09,05),(10,05),(11,04),(12,03),(13,02),(14,01),(15,00),
  (02,06),(03,06),(04,06),(05,06),(06,06),(07,06),(08,06),(09,06),(10,06),(11,05),(12,04),(13,03),(14,02),(15,01),(16,00),
  (03,07),(04,07),(05,07),(06,07),(07,07),(08,07),(09,07),(10,07),(11,06),(12,05),(13,04),(14,03),(15,02),(16,01),(17,00),
  (04,08),(05,08),(06,08),(07,08),(08,08),(09,08),(10,08),(11,07),(12,06),(13,05),(14,04),(15,03),(16,02),(17,01),(18,00),
  (05,09),(06,09),(07,09),(08,09),(09,09),(10,09),(11,08),(12,07),(13,06),(14,05),(15,04),(16,03),(17,02),(18,01),(19,00),
  (06,10),(07,10),(08,10),(09,10),(10,10),(11,09),(12,08),(13,07),(14,06),(15,05),(16,04),(17,03),(18,02),(19,01),(20,00),
  (07,11),(08,11),(09,11),(10,11),(11,10),(12,09),(13,08),(14,07),(15,06),(16,05),(17,04),(18,03),(19,02),(20,01),(NO,NO),
  (08,12),(09,12),(10,12),(11,11),(12,10),(13,09),(14,08),(15,07),(16,06),(17,05),(18,04),(19,03),(20,02),(NO,NO),(NO,NO),
  (09,13),(10,13),(11,12),(12,11),(13,10),(14,09),(15,08),(16,07),(17,06),(18,05),(19,04),(20,03),(NO,NO),(NO,NO),(NO,NO),
  (10,14),(11,13),(12,12),(13,11),(14,10),(15,09),(16,08),(17,07),(18,06),(19,05),(20,04),(NO,NO),(NO,NO),(NO,NO),(NO,NO)
];

pub static DIAGR: [(usize, usize); 225] = [
  (10,14),(09,13),(08,12),(07,11),(06,10),(05,09),(04,08),(03,07),(02,06),(01,05),(00,04),(NO,NO),(NO,NO),(NO,NO),(NO,NO),
  (11,13),(10,13),(09,12),(08,11),(07,10),(06,09),(05,08),(04,07),(03,06),(02,05),(01,04),(00,03),(NO,NO),(NO,NO),(NO,NO),
  (12,12),(11,12),(10,12),(09,11),(08,10),(07,09),(06,08),(05,07),(04,06),(03,05),(02,04),(01,03),(00,02),(NO,NO),(NO,NO),
  (13,11),(12,11),(11,11),(10,11),(09,10),(08,09),(07,08),(06,07),(05,06),(04,05),(03,04),(02,03),(01,02),(00,01),(NO,NO),
  (14,10),(13,10),(12,10),(11,10),(10,10),(09,09),(08,08),(07,07),(06,06),(05,05),(04,04),(03,03),(02,02),(01,01),(00,00),
  (15,09),(14,09),(13,09),(12,09),(11,09),(10,09),(09,08),(08,07),(07,06),(06,05),(05,04),(04,03),(03,02),(02,01),(01,00),
  (16,08),(15,08),(14,08),(13,08),(12,08),(11,08),(10,08),(09,07),(08,06),(07,05),(06,04),(05,03),(04,02),(03,01),(02,00),
  (17,07),(16,07),(15,07),(14,07),(13,07),(12,07),(11,07),(10,07),(09,06),(08,05),(07,04),(06,03),(05,02),(04,01),(03,00),
  (18,06),(17,06),(16,06),(15,06),(14,06),(13,06),(12,06),(11,06),(10,06),(09,05),(08,04),(07,03),(06,02),(05,01),(04,00),
  (19,05),(18,05),(17,05),(16,05),(15,05),(14,05),(13,05),(12,05),(11,05),(10,05),(09,04),(08,03),(07,02),(06,01),(05,00),
  (20,04),(19,04),(18,04),(17,04),(16,04),(15,04),(14,04),(13,04),(12,04),(11,04),(10,04),(09,03),(08,02),(07,01),(06,00),
  (NO,NO),(20,03),(19,03),(18,03),(17,03),(16,03),(15,03),(14,03),(13,03),(12,03),(11,03),(10,03),(09,02),(08,01),(07,00),
  (NO,NO),(NO,NO),(20,02),(19,02),(18,02),(17,02),(16,02),(15,02),(14,02),(13,02),(12,02),(11,02),(10,02),(09,01),(08,00),
  (NO,NO),(NO,NO),(NO,NO),(20,01),(19,01),(18,01),(17,01),(16,01),(15,01),(14,01),(13,01),(12,01),(11,01),(10,01),(09,00),
  (NO,NO),(NO,NO),(NO,NO),(NO,NO),(20,00),(19,00),(18,00),(17,00),(16,00),(15,00),(14,00),(13,00),(12,00),(11,00),(10,00)
];

pub static DIAGL_ARRS:[[u8; 15]; 21] = [
  [004,018,032,046,060,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [005,019,033,047,061,075,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [006,020,034,048,062,076,090,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [007,021,035,049,063,077,091,105,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [008,022,036,050,064,078,092,106,120,NIL,NIL,NIL,NIL,NIL,NIL],
  [009,023,037,051,065,079,093,107,121,135,NIL,NIL,NIL,NIL,NIL],
  [010,024,038,052,066,080,094,108,122,136,150,NIL,NIL,NIL,NIL],
  [011,025,039,053,067,081,095,109,123,137,151,165,NIL,NIL,NIL],
  [012,026,040,054,068,082,096,110,124,138,152,166,180,NIL,NIL],
  [013,027,041,055,069,083,097,111,125,139,153,167,181,195,NIL],
  [014,028,042,056,070,084,098,112,126,140,154,168,182,196,210],
  [029,043,057,071,085,099,113,127,141,155,169,183,197,211,NIL],
  [044,058,072,086,100,114,128,142,156,170,184,198,212,NIL,NIL],
  [059,073,087,101,115,129,143,157,171,185,199,213,NIL,NIL,NIL],
  [074,088,102,116,130,144,158,172,186,200,214,NIL,NIL,NIL,NIL],
  [089,103,117,131,145,159,173,187,201,215,NIL,NIL,NIL,NIL,NIL],
  [104,118,132,146,160,174,188,202,216,NIL,NIL,NIL,NIL,NIL,NIL],
  [119,133,147,161,175,189,203,217,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [134,148,162,176,190,204,218,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [149,163,177,191,205,219,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [164,178,192,206,220,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
];

pub static DIAGR_ARRS:[[u8; 15]; 21] = [
  [074,058,042,026,010,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [089,073,057,041,025,009,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [104,088,072,056,040,024,008,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [119,103,087,071,055,039,023,007,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [134,118,102,086,070,054,038,022,006,NIL,NIL,NIL,NIL,NIL,NIL],
  [149,133,117,101,085,069,053,037,021,005,NIL,NIL,NIL,NIL,NIL],
  [164,148,132,116,100,084,068,052,036,020,004,NIL,NIL,NIL,NIL],
  [179,163,147,131,115,099,083,067,051,035,019,003,NIL,NIL,NIL],
  [194,178,162,146,130,114,098,082,066,050,034,018,002,NIL,NIL],
  [209,193,177,161,145,129,113,097,081,065,049,033,017,001,NIL],
  [224,208,192,176,160,144,128,112,096,080,064,048,032,016,000],
  [223,207,191,175,159,143,127,111,095,079,063,047,031,015,NIL],
  [222,206,190,174,158,142,126,110,094,078,062,046,030,NIL,NIL],
  [221,205,189,173,157,141,125,109,093,077,061,045,NIL,NIL,NIL],
  [220,204,188,172,156,140,124,108,092,076,060,NIL,NIL,NIL,NIL],
  [219,203,187,171,155,139,123,107,091,075,NIL,NIL,NIL,NIL,NIL],
  [218,202,186,170,154,138,122,106,090,NIL,NIL,NIL,NIL,NIL,NIL],
  [217,201,185,169,153,137,121,105,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [216,200,184,168,152,136,120,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [215,199,183,167,151,135,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL],
  [214,198,182,166,150,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL,NIL]
];

impl Board {
  pub fn evaluate (&self) -> i32 {
    let mut value = 0;
    unsafe {
      for i in 0..21usize {
        value += VALUES[((BT2[self.verti_o[i] as usize]) + BT[self.verti_y[i] as usize]) as usize];
        value += VALUES[((BT2[self.horiz_o[i] as usize]) + BT[self.horiz_y[i] as usize]) as usize];
        value += VALUES[((BT2[self.diagl_o[i] as usize]) + BT[self.diagl_y[i] as usize]) as usize];
        value += VALUES[((BT2[self.diagr_o[i] as usize]) + BT[self.diagr_y[i] as usize]) as usize];
      }
    }

    return value;
  }

  pub fn won (&self, you: bool) -> i32 {
    unsafe {
      if you {
        for i in 0..21usize {
          let val_y = (WON[self.verti_y[i] as usize] | WON[self.horiz_y[i] as usize] | WON[self.diagr_y[i] as usize] | WON[self.diagl_y[i] as usize]);
          let val_o = (WON[self.verti_o[i] as usize] | WON[self.horiz_o[i] as usize] | WON[self.diagr_o[i] as usize] | WON[self.diagl_o[i] as usize]);
          
          // Check for five
          if val_y & FIVE_FLAG != 0 { return YOU_WON;}
          if val_o & FIVE_FLAG != 0 { return OPP_WON;}
        }
      } else {
        for i in 0..21usize {
          let val_y = (WON[self.verti_y[i] as usize] | WON[self.horiz_y[i] as usize] | WON[self.diagr_y[i] as usize] | WON[self.diagl_y[i] as usize]);
          let val_o = (WON[self.verti_o[i] as usize] | WON[self.horiz_o[i] as usize] | WON[self.diagr_o[i] as usize] | WON[self.diagl_o[i] as usize]);

          // Check for five
          if val_y & FIVE_FLAG != 0 { return YOU_WON;}
          if val_o & FIVE_FLAG != 0 { return OPP_WON;}
        }
      }
    }
    return 0;
  }

  pub fn gen_moves (&mut self, max: bool) ->  Vec<(u8, i16)>{
      
    let mut movs: Vec<(u8, u8)> = vec![];
    let mut i = 0;


    unsafe {
      if max {
        // THIS IS FOR MAX MOVES
        loop {
          if i >= 21 {
            break;
          }
          let mut l_val = ((BT2[self.diagl_o[i] as usize]) + BT[self.diagl_y[i] as usize]) as usize;
          let mut r_val = ((BT2[self.diagr_o[i] as usize]) + BT[self.diagr_y[i] as usize]) as usize;
          let mut l_more = MAX_ENABLED[l_val];
          let mut r_more = MAX_ENABLED[r_val];
          let mut l_state = MAX_MOVES[l_val].iter();
          let mut r_state = MAX_MOVES[r_val].iter();


          if i < 15 {
            let mut v_val = ((BT2[self.verti_o[i] as usize]) + BT[self.verti_y[i] as usize]) as usize;
            let mut h_val = ((BT2[self.horiz_o[i] as usize]) + BT[self.horiz_y[i] as usize]) as usize;
            let mut v_more = MAX_ENABLED[v_val];
            let mut h_more = MAX_ENABLED[h_val];
            let mut v_state = MAX_MOVES[v_val].iter();
            let mut h_state = MAX_MOVES[h_val].iter();

            while v_more || h_more || l_more || r_more {
              if v_more {
                if let Some(v_val) = v_state.next() {
                  if v_val.0 == EOL { v_more = false; } else { movs.push((VERTI_ARRS[i][14 - v_val.0 as usize], v_val.1)); }
                } else { v_more = false; }
              }
              if h_more {
                if let Some(h_val) = h_state.next() {
                  if h_val.0 == EOL { h_more = false; } else { movs.push((HORIZ_ARRS[i][14 - h_val.0 as usize], h_val.1)); }
                } else { h_more = false; }
              }
              if l_more {
                if let Some(l_val) = l_state.next() {
                  let row = &DIAGL_ARRS[i];
                  if l_val.0 == EOL { l_more = false; } else { if row[l_val.0 as usize] != NIL { movs.push((row[l_val.0 as usize], l_val.1)); } }
                } else { l_more = false; }
              }
              if r_more {
                if let Some(r_val) = r_state.next() {
                  let row = &DIAGR_ARRS[i];
                  if r_val.0 == EOL { r_more = false; } else { if row[r_val.0 as usize] != NIL { movs.push((row[r_val.0 as usize], r_val.1)); } }
                } else { r_more = false; }
              }
            }
          } else {
            while l_more || r_more {
              if l_more {
                if let Some(l_val) = l_state.next() {
                  let row = &DIAGL_ARRS[i];
                  if l_val.0 == EOL { l_more = false; } else { if row[l_val.0 as usize] != NIL { movs.push((row[l_val.0 as usize], l_val.1)); } }
                } else { l_more = false; }
              }
              if r_more {
                if let Some(r_val) = r_state.next() {
                  let row = &DIAGR_ARRS[i];
                  if r_val.0 == EOL { r_more = false; } else { if row[r_val.0 as usize] != NIL { movs.push((row[r_val.0 as usize], r_val.1)); } }
                } else { r_more = false; }
              }
            }
          }
          i += 1;
        }
      } else {
        // THIS IS FOR MINMOVES
        loop {
          if i >= 21 {
            break;
          }
          let mut l_val = ((BT2[self.diagl_o[i] as usize]) + BT[self.diagl_y[i] as usize]) as usize;
          let mut r_val = ((BT2[self.diagr_o[i] as usize]) + BT[self.diagr_y[i] as usize]) as usize;
          let mut l_more = MIN_ENABLED[l_val];
          let mut r_more = MIN_ENABLED[r_val];
          let mut l_state = MIN_MOVES[l_val].iter();
          let mut r_state = MIN_MOVES[r_val].iter();


          if i < 15 {
            let mut v_val = ((BT2[self.verti_o[i] as usize]) + BT[self.verti_y[i] as usize]) as usize;
            let mut h_val = ((BT2[self.horiz_o[i] as usize]) + BT[self.horiz_y[i] as usize]) as usize;
            let mut v_more = MIN_ENABLED[v_val];
            let mut h_more = MIN_ENABLED[h_val];
            let mut v_state = MIN_MOVES[v_val].iter();
            let mut h_state = MIN_MOVES[h_val].iter();

            while v_more || h_more || l_more || r_more {
              if v_more {
                if let Some(v_val) = v_state.next() {
                  if v_val.0 == EOL { v_more = false; } else { movs.push((VERTI_ARRS[i][14 - v_val.0 as usize], v_val.1)); }
                } else { v_more = false; }
              }
              if h_more {
                if let Some(h_val) = h_state.next() {
                  if h_val.0 == EOL { h_more = false; } else { movs.push((HORIZ_ARRS[i][14 - h_val.0 as usize], h_val.1)); }
                } else { h_more = false; }
              }
              if l_more {
                if let Some(l_val) = l_state.next() {
                  let row = &DIAGL_ARRS[i];
                  if l_val.0 == EOL { l_more = false; } else { if row[l_val.0 as usize] != NIL { movs.push((row[l_val.0 as usize], l_val.1)); } }
                } else { l_more = false; }
              }
              if r_more {
                if let Some(r_val) = r_state.next() {
                  let row = &DIAGR_ARRS[i];
                  if r_val.0 == EOL { r_more = false; } else { if row[r_val.0 as usize] != NIL { movs.push((row[r_val.0 as usize], r_val.1)); } }
                } else { r_more = false; }
              }
            }
          } else {
            while l_more || r_more {
              if l_more {
                if let Some(l_val) = l_state.next() {
                  let row = &DIAGL_ARRS[i];
                  if l_val.0 == EOL { l_more = false; } else { if row[l_val.0 as usize] != NIL { movs.push((row[l_val.0 as usize], l_val.1)); } }
                } else { l_more = false; }
              }
              if r_more {
                if let Some(r_val) = r_state.next() {
                  let row = &DIAGR_ARRS[i];
                  if r_val.0 == EOL { r_more = false; } else { if row[r_val.0 as usize] != NIL { movs.push((row[r_val.0 as usize], r_val.1)); } }
                } else { r_more = false; }
              }
            }
          }
          i += 1;
        }

      }
    }

    let _NEG = if max {1} else {-1};
    let _LEN = movs.len();

    if _LEN == 0 {
      return vec![(0,0)];
    }
    if _LEN == 1 {
      return vec![(movs[0].0, _NEG * MAX_VALS[movs[0].1 as usize])];
    }

    movs.sort_by(|a,b| (b.0).cmp(&a.0));


    let mut good_moves = vec![];
    let mut cur = (movs[0].0, _NEG * MAX_VALS[movs[0].1 as usize]);
    let mut index = 1;

    while index < _LEN {
      let mov = movs[index];

      if mov.0 == cur.0 {
        cur.1 += (_NEG * MAX_VALS[mov.1 as usize])
      } else {
        good_moves.push(cur);
        cur = (mov.0, (_NEG * MAX_VALS[mov.1 as usize]));
      }

      index += 1;
    }
    good_moves.push(cur);

    good_moves.sort_by(|a,b| (b.1).cmp(&a.1));
    
    return good_moves;
  }

  pub fn place_piece (&mut self, place: usize, you: bool) {
    unsafe {
      if you {
        self.multi[place] = '1';
        self.place_horiz_you(place);
        self.place_verti_you(place);
        self.place_diagl_you(place);
        self.place_diagr_you(place);
      } else {
        self.multi[place] = '2';
        self.place_horiz_opp(place);
        self.place_verti_opp(place);
        self.place_diagl_opp(place);
        self.place_diagr_opp(place);
      }
    }
  }

  pub fn remove_piece (&mut self, place: usize, you: bool) {
    unsafe {
      self.multi[place] = '0';
      if you {
        self.remove_horiz_you(place);
        self.remove_verti_you(place);
        self.remove_diagl_you(place);
        self.remove_diagr_you(place);
      } else {
        self.remove_horiz_opp(place);
        self.remove_verti_opp(place);
        self.remove_diagl_opp(place);
        self.remove_diagr_opp(place);
      }
    }
  }

  pub unsafe fn place_horiz_you (&mut self, place: usize) {let mov = HORIZ[place]; self.horiz_y[mov.0] |= (1 << mov.1);}
  pub unsafe fn place_verti_you (&mut self, place: usize) {let mov = VERTI[place]; self.verti_y[mov.0] |= (1 << mov.1);}
  pub unsafe fn remove_horiz_you (&mut self, place: usize) {let mov = HORIZ[place]; self.horiz_y[mov.0] ^= (1 << mov.1);}
  pub unsafe fn remove_verti_you (&mut self, place: usize) {let mov = VERTI[place]; self.verti_y[mov.0] ^= (1 << mov.1);}
  pub unsafe fn place_diagl_you (&mut self, place: usize) {let mov = DIAGL[place]; if mov.0 == NO {return;} self.diagl_y[mov.0] |= (1 << mov.1);}
  pub unsafe fn place_diagr_you (&mut self, place: usize) {let mov = DIAGR[place]; if mov.0 == NO {return;} self.diagr_y[mov.0] |= (1 << mov.1);}
  pub unsafe fn remove_diagl_you (&mut self, place: usize) {let mov = DIAGL[place]; if mov.0 == NO {return;} self.diagl_y[mov.0] ^= (1 << mov.1);}
  pub unsafe fn remove_diagr_you (&mut self, place: usize) {let mov = DIAGR[place]; if mov.0 == NO {return;} self.diagr_y[mov.0] ^= (1 << mov.1);}

  pub unsafe fn place_horiz_opp (&mut self, place: usize) {let mov = HORIZ[place]; self.horiz_o[mov.0] |= (1 << mov.1);}
  pub unsafe fn place_verti_opp (&mut self, place: usize) {let mov = VERTI[place]; self.verti_o[mov.0] |= (1 << mov.1);}
  pub unsafe fn remove_horiz_opp (&mut self, place: usize) {let mov = HORIZ[place]; self.horiz_o[mov.0] ^= (1 << mov.1);}
  pub unsafe fn remove_verti_opp (&mut self, place: usize) {let mov = VERTI[place]; self.verti_o[mov.0] ^= (1 << mov.1);}
  pub unsafe fn place_diagl_opp (&mut self, place: usize) {let mov = DIAGL[place]; if mov.0 == NO {return;} self.diagl_o[mov.0] |= (1 << mov.1);}
  pub unsafe fn place_diagr_opp (&mut self, place: usize) {let mov = DIAGR[place]; if mov.0 == NO {return;} self.diagr_o[mov.0] |= (1 << mov.1);}
  pub unsafe fn remove_diagl_opp (&mut self, place: usize) {let mov = DIAGL[place]; if mov.0 == NO {return;} self.diagl_o[mov.0] ^= (1 << mov.1);}
  pub unsafe fn remove_diagr_opp (&mut self, place: usize) {let mov = DIAGR[place]; if mov.0 == NO {return;} self.diagr_o[mov.0] ^= (1 << mov.1);}
}

































































/*

  pub fn gen_moves (&self) -> Vec<(u8,i8)> {
    let mut movs:Vec<(u8,i8)> = vec![];
    unsafe {
      let mut i = 0usize;
      loop {

        if i >= 21{
          break;
        }

        
        let mut h_done = true;
        let mut v_done = true;
        let mut l_done = true;
        let mut r_done = true;

        let mut v_state = MOVES[((BT2[self.verti_o[i] as usize]) + BT[self.verti_y[i] as usize]) as usize].iter();
        let mut h_state = MOVES[((BT2[self.horiz_o[i] as usize]) + BT[self.horiz_y[i] as usize]) as usize].iter();
        let mut l_state = MOVES[((BT2[self.diagl_o[i] as usize]) + BT[self.diagl_y[i] as usize]) as usize].iter();
        let mut r_state = MOVES[((BT2[self.diagr_o[i] as usize]) + BT[self.diagr_y[i] as usize]) as usize].iter();

        if i < 15 {

          loop {
            if v_done {
              match v_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !h_done && !l_done && !r_done {break;} v_done = false;
                  } else {
                    movs.push((VERTI_ARRS[i][14usize - mov.0 as usize], mov.1));
                  }
                }
                None => {
                  if !h_done && !l_done && !r_done { break;} v_done = false;
                }
              }
            }
            if h_done {
              match h_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !l_done && !r_done{break;} h_done = false;
                  } else {
                    movs.push((HORIZ_ARRS[i][14usize - mov.0 as usize], mov.1));
                  }
                },
                None => {
                  if !v_done && !l_done && !r_done {break;} h_done = false;
                },
              }
            }
            if l_done {
              match l_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !h_done && !r_done {break;} l_done = false;
                  } else {
                    let a = DIAGL_ARRS[i][mov.0 as usize];
                    if a != NIL {
                      movs.push((a, mov.1));
                    }
                  }
                },
                None => {
                  if !v_done && !h_done && !r_done {break;} l_done = false;
                },
              }
            }
            if r_done {
              match r_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !h_done && !l_done {break;} r_done = false;
                  } else {
                    let a = DIAGR_ARRS[i][mov.0 as usize];
                    if a != NIL {
                      movs.push((a, mov.1));
                    }
                  }
                },
                None => {
                  if !v_done && !h_done && !l_done {break;} r_done = false;
                },
              }
            }

          }
        } else {
          loop {
            match l_state.next() {
              Some(mov) => {
                if mov.1 == 0 {
                  if !r_done {break;} l_done = false;
                } else {
                  let a = DIAGL_ARRS[i][mov.0 as usize];
                  if a != NIL {
                    movs.push((a, mov.1));
                  }
                }
              },
              None => {
                if !r_done {break;} l_done = false;
              },
            }
            match r_state.next() {
              Some(mov) => {
                if mov.1 == 0 {
                  if !l_done {break;} r_done = false;
                } else {
                  let a = DIAGR_ARRS[i][mov.0 as usize];
                  if a != NIL {
                    movs.push((a, mov.1));
                  }
                }
              },
              None => {
                if !l_done {break;} r_done = false;
              },
            }
          }
        }

        i += 1;
      }
    }

    let mut real_movs: Vec<(u8, i8)> = vec![];
    if movs.len() == 1 {
      real_movs.push((movs[0].0,movs[0].1 as i8));
    }
    else if movs.len() > 0 {
      movs.sort_by(|a,b| (a.0).cmp(&b.0));
      let mut mov_iter = movs.into_iter();

      let mut cur_val = 0;
      let mut cur_mov = 0;

      loop {
        match mov_iter.next() {
          Some(mov) => {
            if mov.0 != cur_mov {
              real_movs.push((cur_mov, cur_val));
              cur_mov = mov.0;
              cur_val = mov.1 as i8;
            } else  {
              cur_val += mov.1 as i8;
            }
          }
          None => {
            break;
          }
        }
      }
    
      real_movs[0] = (cur_mov, cur_val);
    }


    
    real_movs.sort_by(|a,b| (b.1).cmp(&a.1));
    real_movs.truncate(super::MAX_MOVES);
    return real_movs;
    
    let mut i:usize = 0;

    for mov in real_movs.clone() {
      if mov.1 <= super::THRESHOLD {
        if i <= super::MAX_MOVES {
          real_movs.truncate(super::MAX_MOVES);
          return real_movs;
        } else {
          real_movs.truncate(i);
          return real_movs;
        }
      }
      i += 1;
    }

    return real_movs;
  }
 */











/*
  pub fn gen_moves (&self) -> Vec<(u8,i16)> {
    let mut movs:Vec<(u8,i8)> = vec![];
    unsafe {
      let mut i = 0usize;
      loop {

        if i >= 21{
          break;
        }

        
        let mut h_done = true;
        let mut v_done = true;
        let mut l_done = true;
        let mut r_done = true;

        let mut v_state = MOVES[((BT2[self.verti_o[i] as usize]) + BT[self.verti_y[i] as usize]) as usize].iter();
        let mut h_state = MOVES[((BT2[self.horiz_o[i] as usize]) + BT[self.horiz_y[i] as usize]) as usize].iter();
        let mut l_state = MOVES[((BT2[self.diagl_o[i] as usize]) + BT[self.diagl_y[i] as usize]) as usize].iter();
        let mut r_state = MOVES[((BT2[self.diagr_o[i] as usize]) + BT[self.diagr_y[i] as usize]) as usize].iter();

        if i < 15 {

          loop {
            if v_done {
              match v_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !h_done && !l_done && !r_done {break;} v_done = false;
                  } else {
                    movs.push((VERTI_ARRS[i][14usize - mov.0 as usize], mov.1));
                  }
                }
                None => {
                  if !h_done && !l_done && !r_done { break;} v_done = false;
                }
              }
            }
            if h_done {
              match h_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !l_done && !r_done{break;} h_done = false;
                  } else {
                    movs.push((HORIZ_ARRS[i][14usize - mov.0 as usize], mov.1));
                  }
                },
                None => {
                  if !v_done && !l_done && !r_done {break;} h_done = false;
                },
              }
            }
            if l_done {
              match l_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !h_done && !r_done {break;} l_done = false;
                  } else {
                    let a = DIAGL_ARRS[i][mov.0 as usize];
                    if a != NIL {
                      movs.push((a, mov.1));
                    }
                  }
                },
                None => {
                  if !v_done && !h_done && !r_done {break;} l_done = false;
                },
              }
            }
            if r_done {
              match r_state.next() {
                Some(mov) => {
                  if mov.1 == 0 {
                    if !v_done && !h_done && !l_done {break;} r_done = false;
                  } else {
                    let a = DIAGR_ARRS[i][mov.0 as usize];
                    if a != NIL {
                      movs.push((a, mov.1));
                    }
                  }
                },
                None => {
                  if !v_done && !h_done && !l_done {break;} r_done = false;
                },
              }
            }

          }
        } else {
          loop {
            match l_state.next() {
              Some(mov) => {
                if mov.1 == 0 {
                  if !r_done {break;} l_done = false;
                } else {
                  let a = DIAGL_ARRS[i][mov.0 as usize];
                  if a != NIL {
                    movs.push((a, mov.1));
                  }
                }
              },
              None => {
                if !r_done {break;} l_done = false;
              },
            }
            match r_state.next() {
              Some(mov) => {
                if mov.1 == 0 {
                  if !l_done {break;} r_done = false;
                } else {
                  let a = DIAGR_ARRS[i][mov.0 as usize];
                  if a != NIL {
                    movs.push((a, mov.1));
                  }
                }
              },
              None => {
                if !l_done {break;} r_done = false;
              },
            }
          }
        }

        i += 1;
      }
    }

    let mut real_movs: Vec<(u8, i16)> = vec![];

    movs.sort_by(|a,b| (b.1.abs()).cmp(&a.1.abs()));

    let mut cur_mov = 0;
    let mut cur_val: i16 = 0;

      movs.sort_by(|a,b| (b.0).cmp(&a.0));
    if movs.len() == 0 {
      return vec![];
    }
      let mut cur_mov = movs[0].0;
      let mut cur_val = movs[0].1.abs() as i16;

      let mut i = 1;
      while i < movs.len() {
        let cur = movs[i];

        if cur.0 == cur_mov {
          cur_val += cur.1 as i16;
        } else {
          real_movs.push((cur_mov, cur_val.abs()));
          cur_mov = cur.0;
          cur_val = cur.1 as i16;
        }

        i += 1;
      }

      real_movs.push((cur_mov, cur_val));
      real_movs.sort_by(|a,b| (b.1.abs()).cmp(&a.1.abs()));

      return real_movs;
    

    /*let mut real_movs: Vec<(u8, i8)> = vec![];
    if movs.len() == 1 {
      real_movs.push((movs[0].0,movs[0].1 as i8));
    }
    else if movs.len() > 0 {
      movs.sort_by(|a,b| (a.0).cmp(&b.0));
      let mut mov_iter = movs.into_iter();

      let mut cur_val = 0;
      let mut cur_mov = 0;

      loop {
        match mov_iter.next() {
          Some(mov) => {
            if mov.0 != cur_mov {
              real_movs.push((cur_mov, cur_val));
              cur_mov = mov.0;
              cur_val = mov.1 as i8;
            } else  {
              cur_val += mov.1 as i8;
            }
          }
          None => {
            break;
          }
        }
      }
    
      real_movs[0] = (cur_mov, cur_val);
    }

*/
    
    //return movs.into_iter().map(|x| (x.0, x.1 as i16)).collect();
    /*
    real_movs.truncate(super::MAX_MOVES);
    return real_movs;
    
    let mut i:usize = 0;

    for mov in real_movs.clone() {
      if mov.1 <= super::THRESHOLD {
        if i <= super::MAX_MOVES {
          real_movs.truncate(super::MAX_MOVES);
          return real_movs;
        } else {
          real_movs.truncate(i);
          return real_movs;
        }
      }
      i += 1;
    }*/

  }
  */