
use super::{Animation, Frame, fade};
use core::marker::PhantomData;

pub trait AnimationVector<I> {
  fn new() -> Self;
  fn num_animations(&self) -> usize;

  fn ref_anim(&self, i: usize) -> & dyn Animation<I>;
  fn mutref_anim(&mut self, i: usize) -> &mut dyn Animation<I>;
}


pub struct Anim<I, AV : AnimationVector<I>> {
  anims: AV,

  state: State,
  framei: usize,
  ms: usize,
  animi: usize,
  phantom: PhantomData<I>
}

enum State { FadeIn, FadeOut, RUN }

static FADE_IN_FRAMES:usize = 20;
static FADE_OUT_FRAMES:usize = 20;
static RUN_MS:usize = 30000;

impl<I, AV: AnimationVector<I>> Anim<I, AV> { 
  pub fn new() -> Self {
    Anim {
      anims: AV::new(),
      state: State::FadeIn,
      framei: 0,
      ms: 0,
      animi: 0,
      phantom: PhantomData,
    }
  }
 }

 
 impl <I, AV: AnimationVector<I>> Animation<I> for Anim<I,AV> {
    fn next_frame(&mut self, inputs: &I, frame: &mut Frame) -> u16 {

      self.framei += 1;

      match &self.state {
        State::FadeIn => {
          if self.framei >= FADE_IN_FRAMES {
            self.state = State::RUN;
            self.framei = 0;
          }
        }
        State::FadeOut => {
          if self.framei >= FADE_OUT_FRAMES {
            self.state = State::FadeIn;
            self.animi = (self.animi + 1) % self.anims.num_animations();
            self.framei = 0;
            self.ms = 0;
          }
        }
        State::RUN => {
          if self.ms >= RUN_MS {
            self.state = State::FadeOut;
            self.framei = 0;
            self.ms = 0;
          }
        }
      }

      let delayms = self.anims.mutref_anim(self.animi).next_frame(inputs, frame);

      self.ms += delayms as usize;

      match &self.state {
        State::FadeIn => fade(frame, self.framei, FADE_IN_FRAMES),
        State::FadeOut => fade(frame, FADE_OUT_FRAMES - self.framei, FADE_OUT_FRAMES),
        State::RUN => ()
      }

      delayms
    }
 }
