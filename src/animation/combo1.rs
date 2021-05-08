
use super::{Animation, Frame, fade};



pub struct Anim {
  anim2: super::anim2::Anim,
  anim4: super::anim4::Anim,

  state: State,
  framei: usize,
  animi: usize,
}

enum State { FADE_IN, FADE_OUT, RUN }

impl Anim { 
  pub fn new() -> Self {
    Anim {
      anim2: super::anim2::Anim::new(),
      anim4: super::anim4::Anim::new(),
      state: State::FADE_IN,
      framei: 0,
      animi: 0
    }
  }
 }

 static FADE_IN_FRAMES:usize = 20;
 static FADE_OUT_FRAMES:usize = 20;
 static RUN_FRAMES:usize = 200;
 
 impl Animation for Anim {
   fn init_frame(&self) -> Frame {
     self.anim2.init_frame()
   }
 
    fn next_frame(&mut self, frame: &mut Frame) -> u16 {

      self.framei += 1;

      match &self.state {
        State::FADE_IN => {
          if self.framei >= FADE_IN_FRAMES {
            self.state = State::RUN;
            self.framei = 0;
          }
        }
        State::FADE_OUT => {
          if self.framei >= FADE_OUT_FRAMES {
            self.state = State::FADE_IN;
            self.animi = (self.animi + 1) % 2;
            self.framei = 0;
          }
        }
        State::RUN => {
          if self.framei >= RUN_FRAMES {
            self.state = State::FADE_OUT;
            self.framei = 0;
          }
        }
      }

      let delayms = if self.animi == 0 {
        self.anim2.next_frame(frame)
      } else {
        self.anim4.next_frame(frame)
      };

      match &self.state {
        State::FADE_IN => fade(frame, self.framei, FADE_IN_FRAMES),
        State::FADE_OUT => fade(frame, FADE_OUT_FRAMES - self.framei, FADE_OUT_FRAMES),
        State::RUN => ()
      }

      delayms
    }
 }
