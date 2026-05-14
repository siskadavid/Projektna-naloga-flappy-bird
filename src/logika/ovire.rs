use crate::logika::konstante::{self, *};

pub struct Ovire {
    pub x: f32,
    pub y: f32,
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
        self.x + SIRINA_OVIRE < 0.0
    }
    pub fn mimo_ptice(&self) -> bool {
        self.mimo
    }

    pub fn zavrzena_ovira(&mut self, vrednost : bool) {
        self.mimo = vrednost
    }

    pub fn preveri_trk(&self, ptica_y: f32) -> bool {
        let p_desno = X_PTICE + SIRINA_PTICE;
        let p_levo = X_PTICE;
        let p_zgoraj = ptica_y;
        let p_spodaj = ptica_y + VISINA_PTICE;

        if p_desno > self.x && p_levo < self.x + SIRINA_OVIRE {
            // preveri zgornjo cev
            if p_zgoraj < self.y - VISINA_ODPRTINE {
                return true;
            }
            // spodnjo cev
            if p_spodaj > self.y {
                return true;
            }
        }
        false
    }
}