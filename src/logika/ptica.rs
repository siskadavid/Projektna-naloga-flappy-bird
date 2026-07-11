use crate::logika::konstante::*;

pub struct Ptica {
    pub y: f32,
    pub hitrost: f32,
    pub rotacija: f32,
}

impl Ptica {
    pub fn new(zacetni_y_ptice: f32) -> Self {
        Ptica {y: zacetni_y_ptice, hitrost: 0.0, rotacija: 0.0}
    }

    pub fn gravitacija(&mut self, dt: f32) {
        if self.hitrost + GRAVITACIJA * dt <= MAX_HITROST{
            self.hitrost += GRAVITACIJA * dt;
        }
        self.y += self.hitrost * dt;
    }

    pub fn kriljenje(&mut self) {
        self.hitrost = MOC_SKOKA;
    }

    pub fn rotiranje(&mut self) {

        if self.hitrost < 100.0 {
            self.rotacija = -0.4;
        } 

        else if self.hitrost < 300.0 {
            self.rotacija = 0.0;
        } 

        else {
            let padanje_rotacija = (self.hitrost / MAX_HITROST) * 0.9;
            self.rotacija = padanje_rotacija.clamp(0.0, 0.85);      // Dodan clamp da ptica nekaj časa po zamahu gleda gor
        }
    }

    pub fn nihanje(&mut self, cas: f32, zacetni_y_ptice: f32) {
        let bpm = 81.0;
        let frekvenca = (bpm / 60.0) * 6.283;       // 6.283 ~ 2pi
        self.y = zacetni_y_ptice + (cas * (frekvenca / 2.0) + 2.0).sin() * 10.0;
    }
}