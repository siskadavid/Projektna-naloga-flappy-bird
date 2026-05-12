use crate::logika::konstante::*;

pub struct Ptica {
    y: f32,
    hitrost: f32, // Navpična hitrost (če je negativna ptica pada, če je pozitivna leti gor, ker je 0 vrh okna)
    rotacija: f32,
}

impl Ptica {
    pub fn new() -> Self {
        Ptica {y: ZACETNI_Y_PTICE, hitrost: 0.0, rotacija: 0.0}
    }

    pub fn gravitacija(&mut self) {
        if self.hitrost + GRAVITACIJA <= MAX_HITROST{           // Računamo gravitacijo kjer upoštevamo maksimalno hitrost ptice
            self.hitrost += GRAVITACIJA;
        }
        self.y += self.hitrost;
    }

    pub fn kriljenje(&mut self) {
        self.hitrost = MOC_SKOKA; 
    }

    pub fn pozicija(&self) -> f32 {
        self.y                // X pozicija ptice bo ostala ista, ovire se premikajo proti njej
    }

    pub fn trenutna_rotacija(&self) -> f32 {
        self.rotacija
    }

    pub fn rotiranje(&mut self) {

        if self.hitrost < 1.5 {
            self.rotacija = -0.5
        } else if self.hitrost < 2.0 {
            self.rotacija = 0.0
        } else {
            self.rotacija = self.hitrost * 0.2
        }
    }

    pub fn nihanje(&mut self, cas: f64) {
        self.y = ZACETNI_Y_PTICE + (cas * 4.0).sin() as f32 * 10.0;
    }
}

// Test gravitacije
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gravitacije() {
        let mut b = Ptica::new();
        b.gravitacija();
        assert!(b.y > 0.0);
    }
}