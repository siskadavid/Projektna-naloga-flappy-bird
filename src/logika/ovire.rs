use crate::logika::konstante::{HITROST_OVIRE, PRVI_X_OVIRE, SIRINA_OVIRE, SIRINA_ZASLONA};

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
        self.x = PRVI_X_OVIRE;      // to tud se ne vem tocno kako je v rustu kr bo obstajal vec okvir naenkrat tko da ne vem a je to prava ideja
    }

    pub fn premik_cevi(&mut self) {
        self.x -= HITROST_OVIRE;
    }

    pub fn mimo_zaslona(&self) -> bool {
        self.x + SIRINA_OVIRE < SIRINA_ZASLONA
    }
}