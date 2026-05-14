use crate::logika::konstante::*;

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

    pub fn preveri_trk(&self, ptica_x: f32, ptica_y: f32) -> bool {
        let p_desno = ptica_x + SIRINA_PTICE;
        let p_levo = ptica_x;
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