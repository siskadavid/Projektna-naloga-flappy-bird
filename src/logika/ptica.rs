pub struct Ptica {
    x: f32,
    y: f32,
    hitrost: f32, // Navpična hitrost (če je pozitivna ptica pada, če pa je negativna leti gor)
}

impl Ptica {
    pub fn new(x: f32, y: f32) -> Self {
        Ptica { x, y, hitrost: 0.0 }
    }

    pub fn gravitacija(&mut self, gravitacija: f32) {
        self.hitrost += gravitacija;
        self.y += self.hitrost;
    }

    pub fn kriljenje(&mut self, moc: f32) {
        self.hitrost = moc;
    }

    pub fn pozicija(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

// Test gravitacije
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gravitacije() {
        let mut b = Ptica::new(0.0, 0.0);
        b.gravitacija(1.0);
        assert!(b.y > 0.0);
    }
}