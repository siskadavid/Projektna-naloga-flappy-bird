use crate::logika::konstante::*;

pub struct Ovire {
    pub x: f32,
    pub y: f32,
    pub mimo: bool,     // Preverjamo a smo s ptico že mimo ovire da predvajamo zvok in jo lahko čez nekaj časa izbrišemo
    pub obrnjena: bool,     // V paru je ena ovira obrnjena na glavo
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

    pub fn premik_cevi(&mut self) {
        self.x -= HITROST_OVIRE;
    }

    pub fn mimo_zaslona(&self, sirina_ovire: f32) -> bool {
        self.x + sirina_ovire < 0.0
    }

    pub fn zavrzena_ovira(&mut self, vrednost : bool) {
        self.mimo = vrednost
    }

    pub fn preveri_trk(&self, x_ptice: f32, y_ptice: f32, sirina_ptice: f32, visina_ptice: f32, sirina_ovire: f32, velikost_odprtine: f32) -> bool {
        
        let ptica_desno = x_ptice + sirina_ptice;
        let ptica_levo = x_ptice;
        let ptica_zgoraj = y_ptice;
        let ptica_spodaj = y_ptice + visina_ptice;

        if ptica_desno > self.x && ptica_levo < self.x + sirina_ovire {

            // Preveri trg za zgornjo cev
            if ptica_zgoraj < self.y - velikost_odprtine {
                return true;
            }

            // Preveri trk za spodnjo cev
            if ptica_spodaj > self.y {
                return true;
            }
        }
        false
    }
}