use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};

use super::{Animation, Frame, XorShift32, CellAddr, CellType, FRAME_SIZE, FRAME_XMAX, FRAME_YMAX};
use super::gamma::GAMMA;

const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};

const K1: i32 = 1;
const K2: i32 = 100;
const CLAMP_S: i32 = 8192;

const K3: i32 = 94;
const K4: i32 = 100;

pub struct Anim {
  v : [i32; FRAME_SIZE],
  s : [i32; FRAME_SIZE],
  rng: XorShift32,
}

impl Anim { 
  pub fn new() -> Self {
    let v = [0; FRAME_SIZE];
    let mut s = [0; FRAME_SIZE];
    s[CellAddr{x:4, y:2}.faddr()] = 8000;
    s[CellAddr{x:12, y:3}.faddr()] = -8000;
    Anim {v, s, rng:XorShift32{a:456} }
  }
 }
 
 impl Animation for Anim {
  
  fn init_frame(&self) -> Frame {
    [BLACK; FRAME_SIZE]
  }
 
  fn next_frame(&mut self, frame: &mut Frame) -> u16 {

    // Update velocities
    for x in 0..FRAME_XMAX {
      for y in 0..FRAME_YMAX {
        let cell = CellAddr{x,y};
        let ctype = CellType::from(&cell);
        let nn = ctype.num_neighbours();
        let mut s_avg : i32 = 0;
        for ni in 0..nn {
          let ncell = ctype.neighbour(&cell, ni);
          s_avg += self.s[ncell.faddr()];
        }
        let fi = cell.faddr();
        s_avg = s_avg / (nn as i32);
        // s_avg = 0;
        let s = self.s[fi];
        let mut v = self.v[fi];
        v += (s_avg - s) * K1 / K2;
        self.v[fi] = v;
      }
    }

    // Update displacements, and render
    for fi in 0..FRAME_SIZE {
      let mut s = self.s[fi];
      s += self.v[fi];
      s = s * K3 / K4;
      if s > CLAMP_S {
        s = CLAMP_S;
      }
      if s < -CLAMP_S {
        s = - CLAMP_S;
      }
      self.s[fi] = s;

      const HUE1: u8 = 128;
      const HUE2: u8 = 64;

      let hue : u8 = if s > 0 {HUE1} else {HUE2};
      let sat : u8 = 255;
      let val : u8 = GAMMA[(GAMMA.len() as i32 *  s.abs() / CLAMP_S) as usize] as u8;
      // let val : u8 = (255 *  s.abs() / CLAMP_S) as u8;
      frame[fi] = hsv2rgb(Hsv{hue, sat, val});
    }

    // Random Perterbations
    let r1 = self.rng.next();
    if r1 % 100 > 90 {
      let r2 = self.rng.next();
      self.s[(r2 as usize) % FRAME_SIZE] = 8000;
      self.s[((r2+1) as usize) % FRAME_SIZE] = 8000;
    }

    50
  }
}