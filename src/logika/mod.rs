mod ptica;
mod ovire;
mod konstante;

pub use ptica::Ptica;
pub use ovire::Ovire;

pub enum GameMode {
    Menu,
    Igra,
    KonecIgre,
}
pub struct StanjeIgre {
    ptica: Ptica,
    ovire: Vec<Ovire>,
    pub mode: GameMode,
    pub rezultat: u32,
}

impl StanjeIgre {
    pub fn new() -> Self{
        StanjeIgre { ptica: Ptica::new(50.0, 150.0 ), ovire: Vec::new(), mode: GameMode::Menu, rezultat: 0 }
        }

    pub fn premikanje(&mut self) {
            self.ptica.gravitacija(konstante::GRAVITACIJA);
            
            // premik ovir
    }
}

