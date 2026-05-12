use crate::logika::konstante::{GRAVITACIJA, MAX_HITROST, MOC_SKOKA, PRVI_Y_PTICE, X_PTICE};

pub struct Ptica {
    y: f32,
    hitrost: f32, // Navpična hitrost (če je negativna ptica pada, če je pozitivna leti gor, ker je 0 vrh okna)
}

impl Ptica {
    pub fn new() -> Self {
        Ptica {y: PRVI_Y_PTICE, hitrost: 0.0}
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

    pub fn pozicija(&self) -> (f32, f32) {
        (X_PTICE, self.y)   // X pozicija ptice bo ostala ista, ovire se premikajo proti njej
    }

    pub fn trenutna_hitrost(&self) -> f32 {
        self.hitrost
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