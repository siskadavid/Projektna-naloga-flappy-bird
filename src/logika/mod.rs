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
    stevec_slicic: u32, //stevec frameov (idejno)
}

impl StanjeIgre {
    pub fn new() -> Self{
        StanjeIgre { ptica: Ptica::new(), ovire: Vec::new(), mode: GameMode::Menu, rezultat: 0, stevec_slicic: 0,}
        }

    pub fn premikanje(&mut self) {
            self.ptica.gravitacija();
            
            // premik vseh ovir v levo
            for ovira in self.ovire.iter_mut(){
                ovira.premik_cevi();
            }
            // retain obdrži samo tiste elemente, kjer je bool 'true', druge zavrže
            self.ovire.retain(|ovira| {
                let je_na_zaslonu = !ovira.mimo_zaslona();
                je_na_zaslonu}); 
    }
}