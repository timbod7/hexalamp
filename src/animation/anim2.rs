use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};

use super::{Animation, Frame, faddr};

const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};


pub struct Anim {
  framei: usize
}

impl Anim { 
  pub fn new() -> Self {
    Anim {framei:0}
  }
 }
 
 impl Animation for Anim {
 
 
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