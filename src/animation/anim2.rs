use smart_leds::hsv::{Hsv, hsv2rgb};

use super::{Animation, Frame, faddr, FRAME_XMAX, FRAME_YMAX};

pub struct Anim {
  framei: usize
}

impl Anim { 
  pub fn new() -> Self {
    Anim {framei:0}
  }
 }
 
 impl <I> Animation<I> for Anim {
 
   fn next_frame(&mut self, _inputs: &I, frame: &mut Frame) -> u16 {
     self.framei = self.framei + 1;

     for ci in 0..FRAME_XMAX {
       let hue: u8 = (( (ci*8-self.framei) as u32 *255)/120) as u8;
       let c = hsv2rgb(Hsv{hue:hue, sat:255, val:255});
       let x : i16 = ci as i16;
       for y in 0..FRAME_YMAX {
        frame[faddr(x, y as i16 )] = c;
       }
     }
     50
   }
 }