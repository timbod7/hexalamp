use smart_leds::{RGB8};
use super::{Animation, Frame, fill, faddr};

const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};
const BLUE: RGB8 = RGB8 {r: 0, g: 0, b: 0xff,};
const GREEN: RGB8 = RGB8 {r: 0, g: 0xff, b: 0,};
const RED: RGB8 = RGB8 {r: 0xff, g: 0, b: 0,};
const YELLOW: RGB8 = RGB8 {r: 255, g:128, b: 0,};


pub struct Anim {
  framei: usize
}

impl Anim {
 const BG: RGB8 = BLACK;

  pub fn new() -> Self {
    Anim {framei:0}
  }
}

impl Animation for Anim {

  fn next_frame(&mut self, frame: &mut Frame) -> u16 {

    fill( frame, Anim::BG);

    let b = self.framei as i16;
    frame[faddr(b,0)] = BLUE;
    frame[faddr(b,1)] = GREEN;
    frame[faddr(b,2)] = RED;
    frame[faddr(b,3)] = YELLOW;


    self.framei = self.framei + 1;
    120
  }
}