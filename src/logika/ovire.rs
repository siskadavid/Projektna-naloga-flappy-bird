use crate::logika::konstante::VISINA_ZASLONA;

pub struct Ovire {
    x: f32,
    y: f32,
    visina_odprtine: f32,
    sirina: f32,
    hitrost: f32,
}

impl Ovire {
    pub fn new(x: f32, y: f32, visina_odprtine: f32, hitrost: f32) -> Self {
        Ovire {
            x,
            y,
            visina_odprtine,
            sirina: 50.0,
            hitrost, // Hitrost ovire
        }
    }

    pub fn premik_cevi(&mut self) {
        self.x -= self.hitrost;
    }

    pub fn mimo_zaslona(&self) -> bool {
        self.x + self.sirina < 0.0
    }
}