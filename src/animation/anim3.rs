use smart_leds::{RGB8};
use smart_leds::hsv::{Hsv, hsv2rgb};

use super::{Animation, Frame, fill, ADDR, FRAME_XMAX, FRAME_YMAX};

const BLACK: RGB8 = RGB8 {r: 0, g: 0, b: 0,};
const WHITE: RGB8 = RGB8 {r: 255, g: 255, b: 155,};


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
     fill(frame, BLACK);
     let cell = CellAddr{x:(self.framei / FRAME_YMAX) % FRAME_XMAX, y: self.framei % FRAME_YMAX};
     let ctype = cell_type(&cell);
     frame[cell.faddr()] = WHITE;
     for ni in 0..num_neighbours(ctype) {
       let ncell = neighbour(&cell, ctype, ni);
       frame[ncell.faddr()] = WHITE;
     }
     self.framei += 1;
     200
   }
 }



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


fn cell_type(cell: &CellAddr) -> CellType {
  if cell.y == 0 { 
    CellType::BEdge
  }  else if cell.y == FRAME_YMAX - 1 {
    CellType::TEdge
  } else {
    CellType::Internal
  }
}
  
fn num_neighbours(ctype: CellType) -> usize {
  match ctype {
    CellType::BEdge => 2,
    CellType::Internal => 3,
    CellType::TEdge => 2
  }
}

fn neighbour(cell: &CellAddr, ctype: CellType, i: usize) -> CellAddr {
  match ctype {
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
