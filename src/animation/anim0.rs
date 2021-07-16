use smart_leds::{RGB8};
use super::{Animation, Frame, fill, faddr, FRAME_YMAX};

const BG: RGB8 = RGB8 {r: 0, g: 0, b: 0,};
const FG: RGB8 = RGB8 {r: 0xff, g: 0xff, b: 0xff};

pub struct Anim {
  framei: usize
}

impl Anim {

  pub fn new() -> Self {
    Anim {framei:0}
  }
}

impl <I> Animation<I> for Anim {

  fn next_frame(&mut self, inputs: &I, frame: &mut Frame) -> u16 {

    fill( frame, BG);
    let x = self.framei / FRAME_YMAX;
    let y = self.framei % FRAME_YMAX;
    frame[faddr(x as i16,y as i16)] = FG;
    self.framei += 1;
    150
  }
}