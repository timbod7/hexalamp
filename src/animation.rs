
use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};

pub type Frame = [RGB8; 180];

pub trait Animation {
  // Create the frame for the start of the animations
  fn init_frame(&self) -> Frame;

  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}


const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};
const BLUE: RGB8 = RGB8 {r: 0, g: 0, b: 0xff,};
const GREEN: RGB8 = RGB8 {r: 0, g: 0xff, b: 0,};
const RED: RGB8 = RGB8 {r: 0xff, g: 0, b: 0,};
const WHITE: RGB8 = RGB8 {r: 0xff, g: 0xff, b: 0xff,};
const YELLOW: RGB8 = RGB8 {r: 255, g:128, b: 0,};


const GREEN2: RGB8 = RGB8 {r: 0, g: 0x40, b: 0x0,};
const WHITE2: RGB8 = RGB8 {r: 255, g:100, b: 100,};

pub struct Anim1 {
  framei: usize
}

impl Anim1 {
 const BG: RGB8 = BLACK;

  pub fn new() -> Self {
    Anim1 {framei:0}
  }
}

impl Animation for Anim1 {


  fn init_frame(&self) -> Frame {
    [Anim1::BG; 180]
  }

  fn next_frame(&mut self, frame: &mut Frame) -> u16 {

    fill( frame, Anim1::BG);

    let b = self.framei as i16;
    frame[faddr(b,0)] = BLUE;
    frame[faddr(b,1)] = GREEN;
    frame[faddr(b,2)] = RED;
    frame[faddr(b,3)] = YELLOW;


    self.framei = self.framei + 1;
    120
  }
}


pub struct Anim2 {
  framei: usize
}

impl Anim2 { 
  pub fn new() -> Self {
    Anim2 {framei:0}
  }
 }
 
 impl Animation for Anim2 {
 
 
   fn init_frame(&self) -> Frame {
     [BLACK; 180]
   }
 
   fn next_frame(&mut self, frame: &mut Frame) -> u16 {
     self.framei = self.framei + 1;

     for ci in 0..15 {
       let hue: u8 = (( (ci*8-self.framei) as u32 *255)/120) as u8;
       let c = hsv2rgb(Hsv{hue:hue, sat:255, val:255});
       let x : i16 = ci as i16;
       frame[faddr(x, 0)] = c;
       frame[faddr(x, 1)] = c;
       frame[faddr(x, 2)] = c;
       frame[faddr(x, 3)] = c;
       frame[faddr(x, 4)] = c;
       frame[faddr(x, 5)] = c;
     }
     50
   }
 }

 const ADDR: [[usize; 6]; 15] = [
  [0, 1, 32, 33, 64, 65],
  [2, 3, 34, 35, 66, 67],
  [4, 5, 36, 37, 68, 69],
  [6, 7, 38, 39, 70, 72],
  [8, 9, 40, 41, 72, 73],
  [10, 11, 42, 43, 74, 75],
  [12, 13, 44, 45, 76, 77],
  [14, 15, 46, 47, 78, 79],
  [16, 17, 48, 49, 80, 81],
  [18, 19, 50, 51, 82, 83],
  [20, 21, 52, 53, 84, 85],
  [22, 23, 54, 55, 86, 87],
  [24, 25, 56, 57, 88, 89],
  [26, 27, 58, 59, 60, 61],
  [28, 29, 30, 31, 62, 63],
 ];


fn faddr(x: i16, y:i16) -> usize {
  ADDR[((x +15) % 15) as usize][(y as usize)%6]
  // (x * 2 + (y % 2)) % 30  + (y >> 1) * 30 + (if y % 4 >= 2 {2} else {0})
}

fn fill(frame: &mut Frame, color: RGB8) {
  for i in 0..60 {
    frame[i] = color;
  }
}
