pub mod ptica;
pub mod ovire;
pub mod konstante;

pub use ptica::Ptica;
pub use ovire::Ovire;

pub enum GameMode {
    Menu,
    Igra,
    KonecIgre,
}
pub struct StanjeIgre {
    pub ptica: Ptica,
    pub ovire: Vec<Ovire>,
    pub mode: GameMode,
    pub rezultat: u32,
}

impl StanjeIgre {
    pub fn new() -> Self{
        StanjeIgre {ptica: Ptica::new(), ovire: Vec::new(), mode: GameMode::Menu, rezultat: 0}
        }

    pub fn premikanje(&mut self) {
            self.ptica.gravitacija();
            
            // premik ovir
    }
}