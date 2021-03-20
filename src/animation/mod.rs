
use smart_leds::{RGB8};

pub const FRAME_SIZE: usize = 180;
pub const FRAME_XMAX: usize = 15;
pub const FRAME_YMAX: usize = 6; 

pub type Frame = [RGB8; FRAME_SIZE];

pub trait Animation {
  // Create the frame for the start of the animations
  fn init_frame(&self) -> Frame;

  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}

pub mod anim0;
pub mod anim1;
pub mod anim2;

 const ADDR: [[usize; FRAME_YMAX]; FRAME_XMAX] = [
  [0, 1, 30, 31, 62, 63],
  [2, 3, 32, 33, 64, 65],
  [4, 5, 34, 35, 66, 67],
  [6, 7, 36, 37, 68, 69],
  [8, 9, 38, 39, 70, 71],
  [10, 11, 40, 41, 72, 73],
  [12, 13, 42, 43, 74, 75],
  [14, 15, 44, 45, 76, 77],
  [16, 17, 46, 47, 78, 79],
  [18, 19, 48, 49, 80, 81],
  [20, 21, 50, 51, 82, 83],
  [22, 23, 52, 53, 84, 85],
  [24, 25, 54, 55, 86, 87],
  [26, 27, 56, 57, 88, 89],
  [28, 29, 58, 59, 60, 62],  
 ];


fn faddr(x: i16, y:i16) -> usize {
  ADDR[x.rem_euclid(FRAME_XMAX as i16) as usize][y.rem_euclid(FRAME_YMAX as i16) as usize]
}

// fn faddr(x: i16, y:i16) -> usize {
//   ADDR[(x as usize) % FRAME_XMAX][(y as usize) % FRAME_YMAX]
// }

fn fill(frame: &mut Frame, color: RGB8) {
  for i in 0..FRAME_SIZE {
    frame[i] = color;
  }
}
