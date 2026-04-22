mod ptica;
mod ovire;

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