
use smart_leds::{RGB8};

pub type Frame = [RGB8; 60];

pub fn new_frame() -> Frame {
  [BLACK; 60]
}


pub trait Animation {
  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}


const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};
const BLUE: RGB8 = RGB8 {r: 0, g: 0, b: 0xff,};
const GREEN: RGB8 = RGB8 {r: 0, g: 0xff, b: 0,};
const RED: RGB8 = RGB8 {r: 0xff, g: 0, b: 0,};
const WHITE: RGB8 = RGB8 {r: 0xff, g: 0xff, b: 0xff,};

const GREEN2: RGB8 = RGB8 {r: 0, g: 0x8, b: 0x0,};


pub struct Blinky {
  framei: usize
}

impl Blinky {
  pub fn new() -> Self {
    Blinky {framei:0}
  }
}

impl Animation for Blinky {
  fn next_frame(&mut self, frame: &mut Frame) -> u16 {
    frame[self.framei] = GREEN2;
    self.framei = (self.framei + 1) % 4;
    frame[self.framei] = BLUE;
    1000
  }
}