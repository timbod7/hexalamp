use crate::animation::gamma::gamma;
use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};
use super::{Animation, Frame, XorShift32, CellAddr, CellOrientation, Trail, fill, FRAME_XMAX, FRAME_YMAX};



const NUM_PATTERNS: usize = 15;

pub struct Anim {
  rng: XorShift32,
  colors: Colors,
  framei: usize,
  patterns: [Pattern; NUM_PATTERNS],
}


impl Anim { 
  pub fn new() -> Self {
    let rng = XorShift32{a:456};
    let bghue = 220u8;
    let bgval = 64u8;
    let fghue = 250u8;
    let fgval = 255u8;
    Anim {
      rng,
      colors: Colors {
        bghue,
        bgval,
        fghue,
        fgval,
        bg: hsv2rgb(Hsv{hue:bghue, sat:255, val:gamma(bgval)})
      },
      framei: 0,
      patterns:[
        Pattern::new(CellAddr{x:0,y:0}, true),
        Pattern::new(CellAddr{x:0,y:4}, true),
        Pattern::new(CellAddr{x:0,y:8}, true),

        Pattern::new(CellAddr{x:3,y:0}, true),
        Pattern::new(CellAddr{x:3,y:4}, true),
        Pattern::new(CellAddr{x:3,y:8}, true),

        Pattern::new(CellAddr{x:6,y:0}, true),
        Pattern::new(CellAddr{x:6,y:4}, true),
        Pattern::new(CellAddr{x:6,y:8}, true),

        Pattern::new(CellAddr{x:9,y:0}, true),
        Pattern::new(CellAddr{x:9,y:4}, true),
        Pattern::new(CellAddr{x:9,y:8}, true),

        Pattern::new(CellAddr{x:12,y:0}, true),
        Pattern::new(CellAddr{x:12,y:4}, true),
        Pattern::new(CellAddr{x:12,y:8}, true),
      ]
    }
  }
 }
 
 impl <I> Animation<I> for Anim {
    fn next_frame(&mut self, inputs: &I, frame: &mut Frame) -> u16 {
      fill(frame, self.colors.bg);
      for pi in 0..NUM_PATTERNS {
        self.patterns[pi].next_frame(frame, self.framei, &mut self.rng, &self.colors);
      }
      self.framei += 1;
      40
    }
 }

pub struct Colors {
  bghue: u8,
  bgval: u8,
  fghue: u8,
  fgval: u8,
  bg: RGB8,
}

pub fn linterp(x: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
  y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}

pub struct Pattern {
  loc : CellAddr,
  go_left: bool,
}

impl Pattern {
  fn new(loc: CellAddr, go_left: bool) -> Pattern {
    Pattern{loc,go_left}
  }

  fn next_frame(&mut self, frame: &mut Frame, framei: usize, rng: &mut XorShift32, colors: &Colors) {
    let hex0 = self.loc;
    let hex1 = hex0.left().up();
    let hex2 = hex1.up();
    let hex3 = hex2.up();
    let hex4 = hex3.right().down();
    let hex5 = hex4.down();
    
    const PERIOD : usize = 200;    
    const PERIOD2 : usize = PERIOD/2;


    let phase = framei % PERIOD;

    let (hue,val) = if  phase < PERIOD / 2  {
      (
        linterp( phase as i32, 0, PERIOD2 as i32, colors.bghue as i32, colors.fghue as i32),
        linterp( phase as i32, 0, PERIOD2 as i32, colors.bgval as i32, colors.fgval as i32),
      )
    } else {
      (
        linterp( phase as i32, PERIOD2 as i32, PERIOD as i32, colors.fghue as i32, colors.bghue as i32),
        linterp( phase as i32, PERIOD2 as i32, PERIOD as i32, colors.fgval as i32, colors.bgval as i32), 
      )
    };

    let fg = hsv2rgb(Hsv{hue: hue as u8, sat:255, val:gamma(val as u8)});

    if phase == PERIOD - 1 {
      self.loc = if self.go_left {self.loc.left().left()} else  {self.loc.right().right()}
    }


    frame[hex0.faddr()] = fg;
    frame[hex1.faddr()] = fg;
    frame[hex2.faddr()] = fg;
    frame[hex3.faddr()] = fg;
    frame[hex4.faddr()] = fg;
    frame[hex5.faddr()] = fg;
  }
}
