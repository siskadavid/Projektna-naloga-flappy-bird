use crate::logika::konstante::*;

pub struct Ovire {
    pub x: f32,
    pub y: f32,
    pub mimo: bool,
    pub obrnjena: bool,
}

impl Ovire {
    pub fn new(x: f32, y: f32, obrnjena: bool) -> Self {
        Ovire {
            x,
            y,
            mimo: false,
            obrnjena,
        }
    }

    pub fn premik_cevi(&mut self, dt: f32) {
        self.x -= HITROST_OVIRE * dt;
    }

    pub fn mimo_zaslona(&self, sirina_ovire: f32) -> bool {
        self.x + sirina_ovire > 0.0
    }

    pub fn ovira_trk(&self, x_ptice: f32, y_ptice: f32, sirina_ptice: f32, visina_ptice: f32, sirina_ovire: f32, velikost_odprtine: f32) -> bool {

        let ptica_desno = x_ptice + sirina_ptice;
        let ptica_levo = x_ptice;
        let ptica_zgoraj = y_ptice;
        let ptica_spodaj = y_ptice + visina_ptice;

        // Preverimo je x na oviri
        if ptica_desno > self.x && ptica_levo < self.x + sirina_ovire {
            
            // Preverimo ali je y na oviri
            return ptica_zgoraj < self.y - velikost_odprtine || ptica_spodaj > self.y
        }
        false
    }
}