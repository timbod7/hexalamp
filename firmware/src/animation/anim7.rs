use super::{fill, Animation, CellAddr, Frame, XorShift32};
use crate::animation::gamma::gamma;
use crate::animation::phase_color;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::RGB8;

const NUM_PATTERNS: usize = 10;

pub struct Anim {
    rng: XorShift32,
    colors: Colors,
    framei: usize,
    patterns: [Pattern; NUM_PATTERNS],
}

impl Anim {
    pub fn new() -> Self {
        let rng = XorShift32 { a: 456 };
        let bghue = 0u8;
        let bgval = 96u8;
        let fghue = 20u8;
        let fgval = 255u8;
        Anim {
            rng,
            colors: Colors {
                bghue,
                bgval,
                fghue,
                fgval,
                bg: hsv2rgb(Hsv {
                    hue: bghue,
                    sat: 255,
                    val: gamma(bgval),
                }),
            },
            framei: 0,
            patterns: [
                Pattern::new(CellAddr { x: 0, y: 1 }, true),
                Pattern::new(CellAddr { x: 3, y: 1 }, true),
                Pattern::new(CellAddr { x: 6, y: 1 }, true),
                Pattern::new(CellAddr { x: 9, y: 1 }, true),
                Pattern::new(CellAddr { x: 12, y: 1 }, true),
                Pattern::new(CellAddr { x: 0, y: 7 }, true),
                Pattern::new(CellAddr { x: 3, y: 7 }, true),
                Pattern::new(CellAddr { x: 6, y: 7 }, true),
                Pattern::new(CellAddr { x: 9, y: 7 }, true),
                Pattern::new(CellAddr { x: 12, y: 7 }, true),
            ],
        }
    }
}

impl<I> Animation<I> for Anim {
    fn next_frame(&mut self, _inputs: &I, frame: &mut Frame) -> u16 {
        fill(frame, self.colors.bg);
        for pi in 0..NUM_PATTERNS {
            self.patterns[pi].next_frame(frame, self.framei, &mut self.rng, &self.colors);
        }
        self.framei += 1;
        40
    }
}

pub struct Colors {
    bghue: u8,
    bgval: u8,
    fghue: u8,
    fgval: u8,
    bg: RGB8,
}

pub struct Pattern {
    loc: CellAddr,
    orient: bool,
}

impl Pattern {
    fn new(loc: CellAddr, orient: bool) -> Pattern {
        Pattern { loc, orient }
    }

    fn next_frame(
        &mut self,
        frame: &mut Frame,
        framei: usize,
        _rng: &mut XorShift32,
        colors: &Colors,
    ) {
        const PERIOD: usize = 200;
        const PERIOD2: usize = PERIOD * 2;

        let fg = phase_color(
            PERIOD,
            framei,
            colors.bghue,
            colors.bgval,
            colors.fghue,
            colors.fgval,
        );

        if framei % PERIOD == PERIOD - 1 {
            self.loc = self.loc.left();
        }

        let hex0 = self.loc.up().up();
        let hex1 = hex0.right().down();
        let hex2 = hex0.left();
        let hex3 = hex2.down();
        let hex4 = hex2.up().up();
        let hex5 = hex4.up();
        frame[hex0.faddr()] = fg;
        frame[hex1.faddr()] = fg;
        frame[hex2.faddr()] = fg;
        frame[hex3.faddr()] = fg;
        frame[hex4.faddr()] = fg;
        frame[hex5.faddr()] = fg;
    }
}
