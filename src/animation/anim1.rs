use smart_leds::{RGB8};
use super::{Animation, Frame, fill, faddr};
use super::super::adcbuttons::{Button, ButtonState};

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

impl Animation<ButtonState> for Anim {

  fn next_frame(&mut self, inputs: &ButtonState, frame: &mut Frame) -> u16 {

    fill( frame, Anim::BG);

    let r = match inputs {
      Option::None => 0,
      Option::Some(Button::UP) => 1,
      Option::Some(Button::DOWN) => 2,
      Option::Some(Button::LEFT) => 3,
      Option::Some(Button::RIGHT) => 4,
      Option::Some(Button::ENTER) => 5,
    };

    frame[faddr(r,0)] = BLUE;
    frame[faddr(r,1)] = GREEN;
    frame[faddr(r,2)] = RED;
    frame[faddr(r,3)] = YELLOW;


    self.framei = self.framei + 1;
    120
  }
}