
use super::{Animation, Frame, fade};
use super::switcher;

pub struct Anims  {
  anim2: super::anim2::Anim,
  anim4: super::anim4::Anim,
  anim5: super::anim5::Anim,
  anim6: super::anim6::Anim,
}

impl switcher::AnimationVector for Anims {
  fn new() -> Self {
    Anims {
      anim2: super::anim2::Anim::new(),
      anim4: super::anim4::Anim::new(),
      anim5: super::anim5::Anim::new(),
      anim6: super::anim6::Anim::new(),
    }
  }

  fn num_animations(&self) -> usize { 4 }

  fn ref_anim(&self, i: usize) -> &dyn Animation {
    match i {
      0 => & self.anim2,
      1 => & self.anim4,
      2 => & self.anim5,
      _ => & self.anim6,
    }
  }

  fn mutref_anim(&mut self, i: usize) -> &mut dyn Animation {
    match i {
      0 => &mut self.anim2,
      1 => &mut self.anim4,
      2 => &mut self.anim5,
      _ => &mut self.anim6,
    }
  }
}

pub type Anim = switcher::Anim<Anims>;