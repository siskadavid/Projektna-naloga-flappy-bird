use crate::logika::konstante::X_PTICE;

pub struct Ptica {
    y: f32,
    hitrost: f32, // Navpična hitrost (če je pozitivna ptica pada, če pa je negativna leti gor)
}

impl Ptica {
    pub fn new(y: f32) -> Self {
        Ptica {y, hitrost: 0.0 }
    }

    pub fn gravitacija(&mut self, gravitacija: f32) {
        self.hitrost += gravitacija;
        self.y += self.hitrost;
    }

    pub fn kriljenje(&mut self, moc: f32) {
        self.hitrost = moc; // to bo treba spremenit tko da se bo to poklicalo ob kliku in najprej nastimal hitrost na 0 in pol par frameov dodajamo moc, pol pa pride nazaj gravitacija
    }

    pub fn pozicija(&self) -> (f32, f32) {
        (X_PTICE, self.y)   // x pozicija ptice bo ostala ista, ovire se premikajo proti njej
    }
}

// Test gravitacije
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gravitacije() {
        let mut b = Ptica::new(150.0);
        b.gravitacija(1.0);
        assert!(b.y > 0.0);
    }
}