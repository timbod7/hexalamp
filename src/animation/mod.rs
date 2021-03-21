
use smart_leds::{RGB8};

pub const FRAME_SIZE: usize = 180;
pub const FRAME_XMAX: usize = 15;
pub const FRAME_YMAX: usize = 6; 

pub type Frame = [RGB8; FRAME_SIZE];

pub trait Animation {
  // Create the frame for the start of the animations
  fn init_frame(&self) -> Frame;

  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}

pub mod anim0;
pub mod anim1;
pub mod anim2;
pub mod anim3;

 const ADDR: [[usize; FRAME_YMAX]; FRAME_XMAX] = [
  [0, 1, 30, 31, 62, 63],
  [2, 3, 32, 33, 64, 65],
  [4, 5, 34, 35, 66, 67],
  [6, 7, 36, 37, 68, 69],
  [8, 9, 38, 39, 70, 71],
  [10, 11, 40, 41, 72, 73],
  [12, 13, 42, 43, 74, 75],
  [14, 15, 44, 45, 76, 77],
  [16, 17, 46, 47, 78, 79],
  [18, 19, 48, 49, 80, 81],
  [20, 21, 50, 51, 82, 83],
  [22, 23, 52, 53, 84, 85],
  [24, 25, 54, 55, 86, 87],
  [26, 27, 56, 57, 88, 89],
  [28, 29, 58, 59, 60, 61],  
 ];


fn faddr(x: i16, y:i16) -> usize {
  ADDR[x.rem_euclid(FRAME_XMAX as i16) as usize][y.rem_euclid(FRAME_YMAX as i16) as usize]
}


// fn faddr(x: i16, y:i16) -> usize {
//   ADDR[(x as usize) % FRAME_XMAX][(y as usize) % FRAME_YMAX]
// }

fn fill(frame: &mut Frame, color: RGB8) {
  for i in 0..FRAME_SIZE {
    frame[i] = color;
  }
}



pub struct XorShift32 {
  a: u32,
}

impl XorShift32 {
  pub fn next(&mut self) -> u32 {
      let mut x = self.a;
      x ^= x << 13;
      x ^= x << 17;
      x ^= x >> 5;
      self.a = x;
      x
  }
}



const NUM_GAMMA: usize = 16;
const GAMMA : [u8; NUM_GAMMA] = [
  0,
  1,
  2,
  6,
  12,
  20,
  31,
  44,
  60,
  79,
  100,
  125,
  153,
  183,
  218,
  255,
];

 struct CellAddr {
   x : usize,
   y : usize,
 }

 impl CellAddr {

  fn faddr(&self) -> usize {
    ADDR[self.x][self.y]
  }

  fn left(&self) -> CellAddr {
    CellAddr{x: if self.x == 0 {FRAME_XMAX-1} else {self.x-1}, y:self.y}
  }
  fn right(&self) -> CellAddr {
    CellAddr{x: if self.x == FRAME_XMAX-1 {0} else {self.x+1}, y:self.y}
  }
  fn down(&self) -> CellAddr {
    CellAddr{x: self.x, y: if self.y == 0 {FRAME_YMAX-1} else {self.y-1}}
  }
  fn up(&self) -> CellAddr {
    CellAddr{x: self.x, y: if self.y == FRAME_YMAX-1 {0} else {self.y+1}}
  }
 }

 #[derive(Copy, Clone)]
 enum CellType {
   BEdge,
   Internal,
   TEdge
}

impl CellType {

  fn from(cell: &CellAddr) -> CellType {
    if cell.y == 0 { 
      CellType::BEdge
    }  else if cell.y == FRAME_YMAX - 1 {
      CellType::TEdge
    } else {
      CellType::Internal
    }
  }
    
  fn num_neighbours(&self) -> usize {
    match self {
      CellType::BEdge => 2,
      CellType::Internal => 3,
      CellType::TEdge => 2
    }
  }

  fn neighbour(&self, cell: &CellAddr, i: usize) -> CellAddr {
    match self {
      CellType::BEdge => {
        match i {
          0 => cell.left().up(),
          _ => cell.up()
        }
      }
      CellType::Internal => {
        if cell.y % 2 == 0 {
          // up pointing
          match i {
            0 => cell.down(),
            1 => cell.left().up(),
            _ => cell.up()  
          }
        } else {
          // down pointing
          match i {
            0 => cell.down(),
            1 => cell.right().down(),
            _ => cell.up()  
          }
        }
      },
      CellType::TEdge => {
        match i {
          0 => cell.down(),
          _ => cell.right().down()
        }
      }
    }
  }
}



