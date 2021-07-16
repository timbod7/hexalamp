
use smart_leds::{RGB8};

pub const FRAME_SIZE: usize = 180;
pub const FRAME_XMAX: usize = 15;
pub const FRAME_YMAX: usize = 12; 

pub type Frame = [RGB8; FRAME_SIZE];

pub trait Animation {
  // Generate the next frame of the animation, and return the delay in
  // ms until the next;
  fn next_frame(&mut self, frame: &mut Frame) -> u16;
}

pub fn initFrame() -> Frame {
  [RGB8 {r: 0, g: 0, b: 0,}; FRAME_SIZE]
}

pub mod gamma;
pub mod switcher;
pub mod anim0;
pub mod anim1;
pub mod anim2;
pub mod anim3;
pub mod anim4;
pub mod anim5;
pub mod anim6;
pub mod combo1;

use gamma::GAMMA;

 const ADDR: [[usize; FRAME_YMAX]; FRAME_XMAX] = [
  [0, 1, 30, 31, 62, 63, 92, 93, 124, 125, 154, 155],
  [2, 3, 32, 33, 64, 65, 94 ,95, 126, 127, 156, 157],
  [4, 5, 34, 35, 66, 67, 96, 97, 128, 129, 158, 159],
  [6, 7, 36, 37, 68, 69, 98, 99, 130, 131, 160, 161],
  [8, 9, 38, 39, 70, 71, 100, 101, 132, 133, 162, 163],
  [10, 11, 40, 41, 72, 73, 102, 103, 134, 135, 164, 165],
  [12, 13, 42, 43, 74, 75, 104, 105, 136, 137, 166, 167],
  [14, 15, 44, 45, 76, 77, 106, 107, 138, 139, 168, 169],
  [16, 17, 46, 47, 78, 79, 108, 109, 140, 141, 170, 171],
  [18, 19, 48, 49, 80, 81, 110, 111, 142, 143, 172, 173],
  [20, 21, 50, 51, 82, 83, 112, 113, 144, 145, 174, 175],
  [22, 23, 52, 53, 84, 85, 114, 115, 146, 147, 176, 177],
  [24, 25, 54, 55, 86, 87, 116, 117, 148, 149, 178, 179],
  [26, 27, 56, 57, 88, 89, 118, 119, 120, 121, 150, 151],
  [28, 29, 58, 59, 60, 61, 90, 91, 122, 123, 152, 153],  
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

// Fade the contents of the frame:
//     u/v = 1  => leave as is
//     u/v = 0  => fully black
fn fade(frame: &mut Frame, u0: usize, v0:usize) -> () {
  let u = GAMMA[u0 * (GAMMA.len()-1) / v0] as usize;
  let v = 255;
  for i in 0..FRAME_SIZE {
    frame[i].r = ((frame[i].r as usize * u) / v) as u8;
    frame[i].g = ((frame[i].g as usize * u) / v) as u8;
    frame[i].b = ((frame[i].b as usize * u) / v) as u8;
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


#[derive(Copy, Clone)]
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
 enum CellOrientation {
   PointUp,
   PointDown,
}

impl CellOrientation {
  fn from(cell: &CellAddr) -> CellOrientation {
    if cell.y % 2 == 0 {
     CellOrientation::PointUp 
    } else {
     CellOrientation::PointDown
    }
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