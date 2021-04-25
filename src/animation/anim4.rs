use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};
use super::{Animation, Frame, XorShift32, CellAddr, CellType, CellOrientation, faddr, fill, FRAME_XMAX, FRAME_YMAX};



const NUM_PATTERNS: usize = 4;

pub struct Anim {
  rng: XorShift32,
  colors: Colors,
  patterns: [Pattern; NUM_PATTERNS],
}


impl Anim { 
  pub fn new() -> Self {
    let rng = XorShift32{a:456};
    let hue = 2u8;
    let bgval = 32u8;
    Anim {
      rng,
      colors: Colors {
        hue,
        bgval: 32,
        bg:  hsv2rgb(Hsv{hue, sat:255, val:bgval}),
      },
      patterns:[
        Pattern::new(),
        Pattern::new(),
        Pattern::new(),
        Pattern::new(),
      ]
    }
  }
 }
 
 impl Animation for Anim {
   fn init_frame(&self) -> Frame {
     [self.colors.bg; 180]
   }
 
    fn next_frame(&mut self, frame: &mut Frame) -> u16 {
      fill(frame, self.colors.bg);
      for pi in 0..NUM_PATTERNS {
        self.patterns[pi].next_frame(frame, &mut self.rng, &self.colors);
      }
      80
    }
 }

pub struct Colors {
  hue: u8,
  bgval: u8,
  bg: RGB8,
}

pub struct Pattern {
  trail: Trail<12>,
  mode: Mode,
}

#[derive(PartialEq)]
enum Mode {
  Grow,
  Shrink,
  Delay
}

impl Pattern {
  fn new() -> Pattern {
    Pattern{trail:Trail::new(), mode:Mode::Delay}
  }

  fn next_frame(&mut self, frame: &mut Frame, rng: &mut XorShift32, colors: &Colors) {
    match self.mode {
      Mode::Grow => {
        if self.trail.size == 10 {
          self.mode = Mode::Shrink;
        } else {
          let head = self.trail.head();
          match CellOrientation::from(&head) {
            CellOrientation::PointUp => self.trail.push_head(head.down()),
            CellOrientation::PointDown => {
              let r = rng.next();
              if r % 2 == 0 {
                self.trail.push_head(head.down());
              } else {
                self.trail.push_head(head.down().right());
              }
            }
          }
        }
      }
      Mode::Shrink => {
        self.trail.pop_tail();
        if self.trail.size == 0 {
          self.mode = Mode::Delay;
        }
      }

      Mode::Delay => {
        let r = rng.next() % 10;
        if r == 0 {
          self.mode = Mode::Grow;
          let x = (rng.next() as usize) % FRAME_XMAX;
          let addr = CellAddr{x,y:FRAME_YMAX-1};
          self.trail = Trail::new();
          self.trail.push_head(addr);
        }
      }
    }

    for i in 0..self.trail.size {
      let hue = colors.hue;
      let maxval = 255;
      let bgval = colors.bgval;
      let dval = (maxval as usize - bgval as usize) * 128 / 12;
      
      let color = match self.mode {
        Mode::Grow =>
          if i == 0 {
            hsv2rgb(Hsv{hue:0, sat:0, val:maxval}) 
          } else {
            let val = bgval + (dval * (12 - i) / 128) as u8;
            hsv2rgb(Hsv{hue, sat:255, val})
          },
        Mode::Shrink => {
          let val = bgval +  (dval * (self.trail.size - i) / 128) as u8;
          hsv2rgb(Hsv{hue, sat:255, val})
        }
        Mode::Delay => hsv2rgb(Hsv{hue, sat:255, val:128}),
      };
      frame[self.trail.cell(i).faddr()] = color;
    }
  }
}


 struct Trail<const TSIZE: usize> {
   cells: [CellAddr; TSIZE],
   headi: usize,
   size: usize,
 }

impl <const TSIZE: usize> Trail<TSIZE> {
  pub fn new() -> Self {
    Trail {
      cells: [CellAddr{x:0,y:0}; TSIZE],
      headi:0,
      size:0,
    }
  }

  pub fn head(&mut self) -> CellAddr {
    self.cells[self.headi]
  }

  pub fn push_head(&mut self, addr:CellAddr) {
    self.headi = (self.headi + 1) % TSIZE;
    self.cells[self.headi] = addr;
    if self.size < TSIZE {
      self.size += 1;
    }
  }

  pub fn pop_tail(&mut self) {
    if self.size > 0 {
      self.size -= 1;
    }
  }

  pub fn cell(&self, i: usize) -> CellAddr {
    self.cells[(self.headi + TSIZE - i) % TSIZE]
  }
}