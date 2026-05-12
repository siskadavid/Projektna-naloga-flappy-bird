use crate::logika::konstante::{HITROST_OVIRE, SIRINA_OVIRE};

pub struct Ovire {
    x: f32,
    y: f32,
    mimo: bool,
}

impl Ovire {
    pub fn new(x: f32, y: f32) -> Self {
        Ovire {
            x,
            y,
            mimo: false,
        }
    }

    pub fn premik_cevi(&mut self) {
        self.x -= HITROST_OVIRE;
    }

    pub fn mimo_zaslona(&self) -> bool {
        self.x + SIRINA_OVIRE < 0.0 // ko gre mimo izhodišča (izhodišče je levo gor)
    }
}