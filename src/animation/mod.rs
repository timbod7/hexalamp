
use smart_leds::{RGB8};

pub type Frame = [RGB8; 180];

pub trait Animation {
  // Create the frame for the start of the animations
  fn init_frame(&self) -> Frame;

  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}

pub mod anim1;
pub mod anim2;

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
}

fn fill(frame: &mut Frame, color: RGB8) {
  for i in 0..60 {
    frame[i] = color;
  }
}
