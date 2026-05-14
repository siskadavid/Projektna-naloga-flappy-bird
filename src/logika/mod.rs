use crate::logika::konstante::*;

pub mod ptica;
pub mod ovire;
pub mod konstante;

pub use ptica::Ptica;
pub use ovire::Ovire;

pub enum GameMode {
    Menu,
    Igra,
    Pavza,
    KonecIgre,
}
pub struct StanjeIgre {
    pub ptica: Ptica,
    pub ovire: Vec<Ovire>,
    pub mode: GameMode,
    pub rezultat: u32,
    pub stevec_slicic: u32, //stevec frameov (idejno)
    pub ozadje_x: f32,
    pub tla_x: f32,
    pub glasba: bool
}

impl StanjeIgre {
    // Nova igra
    pub fn new() -> Self{
        StanjeIgre {ptica: Ptica::new(), ovire: Vec::new(), mode: GameMode::Menu, rezultat: 0, ozadje_x: 0.0, tla_x: 0.0, glasba: false, stevec_slicic: 0}
        }
    
    // Računanje pozicije ozadja in tal
    pub fn bg(&mut self, scaled_x_ozadja: f32) {

        self.ozadje_x -= HITROST_OVIRE / 10.0;        // Ozadje se premika počasneje kot ovire za občutek globin

        if self.ozadje_x <= -scaled_x_ozadja {       // Ko pride ena slika ozadja preveč naprej jo prestavimo nazaj
            self.ozadje_x = 0.0
        }
        
        self.tla_x -= HITROST_OVIRE;
        if self.tla_x <= -SIRINA_ZASLONA {
            self.tla_x = 0.0
        }
    }

    // Računanje premikov ptice in ovir
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