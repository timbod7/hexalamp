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

type InputType  = u16;

impl Animation<InputType> for Anim {

  fn next_frame(&mut self, inputs: &InputType, frame: &mut Frame) -> u16 {

    fill( frame, Anim::BG);

    let v = *inputs;

    hex_digit(frame, 0, 2, &BLUE, ((v & 0xf00) >> 8) as u8);
    hex_digit(frame, 4, 2, &BLUE, ((v & 0x0f0) >> 4) as u8);
    hex_digit(frame, 8, 2, &BLUE, ((v & 0x00f) >> 0) as u8);

    self.framei += 1;

    120
  }
}

const SEG_A : u8 = 1 << 0;
const SEG_B : u8 = 1 << 1;
const SEG_C : u8 = 1 << 2;
const SEG_D : u8 = 1 << 3;
const SEG_E : u8 = 1 << 4;
const SEG_F : u8 = 1 << 5;
const SEG_G : u8 = 1 << 6;

const SEGMENTS: [u8;16] = [
  SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F,
  SEG_B | SEG_C,
  SEG_A | SEG_B | SEG_D | SEG_E | SEG_G,
  SEG_A | SEG_B | SEG_C | SEG_D | SEG_G,
  SEG_B | SEG_C | SEG_F | SEG_G,
  SEG_A | SEG_C | SEG_D | SEG_F | SEG_G,      
  SEG_A | SEG_C | SEG_D | SEG_E | SEG_F | SEG_G,      
  SEG_A | SEG_B | SEG_C,       
  SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F | SEG_G,
  SEG_A | SEG_B | SEG_C | SEG_D | SEG_F | SEG_G,
  SEG_A | SEG_B | SEG_C | SEG_E | SEG_F | SEG_G,
  SEG_C | SEG_D | SEG_E | SEG_F | SEG_G,
  SEG_A | SEG_D | SEG_E | SEG_F,
  SEG_B | SEG_C | SEG_D | SEG_E | SEG_G,
  SEG_A | SEG_D | SEG_E | SEG_F | SEG_G,
  SEG_A | SEG_E | SEG_F | SEG_G,
];

fn hex_digit(frame: &mut Frame, x: i16, y: i16, color: &RGB8,  val:u8) {
  let segments = SEGMENTS[if val >= 16 {15} else {val as usize}];
  if segments & SEG_A != 0 {
    hbar(frame, x, y+8, color);
  }
  if segments & SEG_B != 0 {
    vbar(frame, x+2, y+4, color);
  }
  if segments & SEG_C != 0 {
    vbar(frame, x+2, y, color);
  }
  if segments & SEG_D != 0 {
    hbar(frame, x, y, color);
  }
  if segments & SEG_E != 0 {
    vbar(frame, x, y, color);
  }
  if segments & SEG_F != 0 {
    vbar(frame, x, y+4, color);
  }
  if segments & SEG_G != 0 {
    hbar(frame, x, y+4, color);
  }
}

fn hbar(frame: &mut Frame, x: i16, y: i16, color: &RGB8) {
  frame[faddr(x,y)] = *color;
  frame[faddr(x,y+1)] = *color;
  frame[faddr(x+1,y)] = *color;
  frame[faddr(x+1,y+1)] = *color;
  frame[faddr(x+2,y)] = *color;
  frame[faddr(x+2,y+1)] = *color;
}

fn vbar(frame: &mut Frame, x: i16, y: i16, color: &RGB8) {
  frame[faddr(x,y)] = *color;
  frame[faddr(x,y+1)] = *color;
  frame[faddr(x,y+2)] = *color;
  frame[faddr(x,y+3)] = *color;
  frame[faddr(x,y+4)] = *color;
  frame[faddr(x,y+5)] = *color;
}