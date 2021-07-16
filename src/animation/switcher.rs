
use super::{Animation, Frame, fade};


pub trait AnimationVector {
  fn new() -> Self;
  fn num_animations(&self) -> usize;

  fn ref_anim(&self, i: usize) -> & dyn Animation<()>;
  fn mutref_anim(&mut self, i: usize) -> &mut dyn Animation<()>;
}


pub struct Anim<AV : AnimationVector> {
  anims: AV,

  state: State,
  framei: usize,
  ms: usize,
  animi: usize,
}

enum State { FADE_IN, FADE_OUT, RUN }

static FADE_IN_FRAMES:usize = 20;
static FADE_OUT_FRAMES:usize = 20;
static RUN_MS:usize = 30000;

impl<AV: AnimationVector> Anim<AV> { 
  pub fn new() -> Self {
    Anim {
      anims: AV::new(),
      state: State::FADE_IN,
      framei: 0,
      ms: 0,
      animi: 0,
    }
  }
 }

 
 impl <AV: AnimationVector> Animation<()> for Anim<AV> {
    fn next_frame(&mut self, inputs: &(), frame: &mut Frame) -> u16 {

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
            self.animi = (self.animi + 1) % self.anims.num_animations();
            self.framei = 0;
            self.ms = 0;
          }
        }
        State::RUN => {
          if self.ms >= RUN_MS {
            self.state = State::FADE_OUT;
            self.framei = 0;
            self.ms = 0;
          }
        }
      }

      let delayms = self.anims.mutref_anim(self.animi).next_frame(inputs, frame);

      self.ms += delayms as usize;

      match &self.state {
        State::FADE_IN => fade(frame, self.framei, FADE_IN_FRAMES),
        State::FADE_OUT => fade(frame, FADE_OUT_FRAMES - self.framei, FADE_OUT_FRAMES),
        State::RUN => ()
      }

      delayms
    }
 }
