// Handle 5 buttons attached to a single ADC input in the style
// of FPV camera control.
//
// see: https://github.com/betaflight/betaflight/wiki/FPV-Camera-Control-(Joystick-Emulation)


// Control pad resistors, with expected 12 bit ADC output given 47k series resistor
//
// BUTTON  RES     ADC  THRESHOLD
//-------------------------------
//      -  inf     9096
//  enter  45k     2003      5549
//   left  27k     1494      1748
//     up  15k      991      1243
//  right 6.8k      518       754     
//   down   0k        0       259

pub type ButtonState = Option<Button>;

pub enum Button {
  DOWN,
  RIGHT,
  UP,
  LEFT,
  ENTER,
}

impl Button {
  pub fn fromAdc(adc:u16) -> Option<Button> {
    if adc < 259 {
      Option::from(Button::DOWN)
    } else if adc < 754 {
      Option::from(Button::RIGHT)
    } else if adc < 1243 {
      Option::from(Button::UP)
    } else if adc < 1748 {
      Option::from(Button::LEFT)
    } else if adc < 5549 {
      Option::from(Button::ENTER)
    } else {
      Option::None
    }
  }
}