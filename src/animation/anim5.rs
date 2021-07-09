use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};
use super::{Animation, Frame, XorShift32, CellAddr, CellOrientation, Trail, fill, FRAME_XMAX, FRAME_YMAX};



const NUM_PATTERNS: usize = 3;
const TRAIL_LENGTH: usize = 12;

pub struct Anim {
  rng: XorShift32,
  colors: Colors,
  patterns: [Pattern; NUM_PATTERNS],
}


impl Anim { 
  pub fn new() -> Self {
    let rng = XorShift32{a:456};
    let hue = 96u8;
    let bgval = 8u8;
    Anim {
      rng,
      colors: Colors {
        hue,
        bgval: 32,
        bg:  hsv2rgb(Hsv{hue, sat:255, val:bgval}),
      },
      patterns:[
        Pattern::new(CellAddr{x:0,y:5}),
        Pattern::new(CellAddr{x:5,y:6}),
        Pattern::new(CellAddr{x:10,y:5})

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
  trail: Trail<TRAIL_LENGTH>,
}



impl Pattern {
  fn new(start: CellAddr) -> Pattern {
    let mut trail = Trail::new();
    trail.push_head(start);
    Pattern{trail}
  }

  fn next_frame(&mut self, frame: &mut Frame, rng: &mut XorShift32, colors: &Colors) {
    if self.trail.size == TRAIL_LENGTH {
      self.trail.pop_tail();
    }
    let head = &self.trail.head();
    let r = rng.next() % 4;
    // r == 0 => change lane
    // otherwise    => stay at current level

    let head1  = match CellOrientation::from(head) {
      CellOrientation::PointUp => {
        if r == 0 && head.y > 0  {
          head.down()
        } else {
          head.up()
        }
      }
      CellOrientation::PointDown => {
        if r == 0 && head.y < FRAME_YMAX - 1 {
          head.up()
        } else {
          head.right().down()
        }
        
      }
    };
    self.trail.push_head(head1);

    for i in 0..self.trail.size {
      let hue = colors.hue;
      let maxval = 255;
      let bgval = colors.bgval;
      let dval = (maxval as usize - bgval as usize) * 128 / TRAIL_LENGTH;
      
      let color = 
          if i == 0 {
            hsv2rgb(Hsv{hue:0, sat:0, val:maxval}) 
          } else {
            let val = bgval + (dval * (TRAIL_LENGTH - i) / 128) as u8;
            hsv2rgb(Hsv{hue, sat:255, val})
          };

      frame[self.trail.cell(i).faddr()] = color;
    }
  }
}
