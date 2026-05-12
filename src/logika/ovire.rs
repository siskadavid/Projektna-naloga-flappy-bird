use crate::logika::konstante::*;

pub struct Ovire {
    x: f32,
    y: f32,
    mimo: bool,
}

impl Ovire {
    pub fn new(x: f32, y: f32, mimo: bool) -> Self {
        Ovire {
            x,
            y,
            mimo: false,
        }
    }

    pub fn nova_ovira(&mut self) {
        self.x = ZACETNI_X_OVIRE;
    }

    pub fn premik_cevi(&mut self) {
        self.x -= HITROST_OVIRE;
    }

    pub fn mimo_zaslona(&self) -> bool {
        self.x + SIRINA_OVIRE < SIRINA_ZASLONA
    }
}