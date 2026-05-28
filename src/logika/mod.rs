use macroquad::rand::gen_range; // Generiranje naključnih števil za višino cevi

pub mod ptica;
pub mod ovire;
pub mod konstante;

use crate::logika::konstante::*;
pub use ptica::Ptica;
pub use ovire::Ovire;

#[derive(PartialEq)]
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
    pub ozadje_x: f32,
    pub tla_x: f32,
    pub glasba: bool,
    pub casovnik: f32, // Štejemo koliko časa je preteklo med kalkulacijami
}

impl StanjeIgre {
    // Nova igra
    pub fn new(zacetni_y_ptice: f32) -> Self{
        StanjeIgre {ptica: Ptica::new(zacetni_y_ptice), 
            ovire: Vec::new(), 
            mode: GameMode::Menu, 
            rezultat: 0, 
            ozadje_x: 0.0, 
            tla_x: 0.0, 
            glasba: false, 
            casovnik: 0.0
        }
    }
    
    // Računanje pozicije ozadja in tal
    pub fn bg(&mut self, sirina_ozadja: f32, sirina_zaslona: f32) {

        self.ozadje_x -= HITROST_OVIRE / 10.0;        // Ozadje se premika počasneje kot ovire za občutek globin

        if self.ozadje_x <= -sirina_ozadja {       // Ko pride ena slika ozadja preveč naprej jo prestavimo nazaj
            self.ozadje_x = 0.0
        }
        
        self.tla_x -= HITROST_OVIRE;
        if self.tla_x <= -sirina_zaslona {
            self.tla_x = 0.0
        }
    }

    // Računanje premikov ptice in ovir
    pub fn premikanje_ptice(&mut self) {
        self.ptica.gravitacija();
        self.ptica.rotiranje();
    }

    pub fn premikanje_ovir(&mut self, visina_zaslona: f32, x_ptice: f32, sirina_ptice: f32, visina_ptice: f32, sirina_ovire: f32, visina_ovire: f32, velikost_odprtine: f32, zacetni_x_ovire: f32) {
        self.casovnik += macroquad::time::get_frame_time();

        // Vsakih 340 frameov narišemo nov par cevi
        if self.casovnik >= 1.7 {
            let tla_y = visina_zaslona * 0.915;
            let min_vidna_cev = visina_zaslona * 0.2;
            let max_y_odprtine = tla_y - velikost_odprtine - min_vidna_cev;

            let y_odprtine = gen_range(min_vidna_cev,  max_y_odprtine);

            self.ovire.push(Ovire::new(zacetni_x_ovire, y_odprtine - visina_ovire, true));
            self.ovire.push(Ovire::new(zacetni_x_ovire, y_odprtine + velikost_odprtine, false));

            self.casovnik -= 1.7
        }
        
        // Premik vseh ovir v levo
        for ovira in self.ovire.iter_mut(){
            ovira.premik_cevi();
            
            // Preveri trk
            if ovira.obrnjena == false {
                if ovira.preveri_trk(x_ptice,self.ptica.y, sirina_ptice, visina_ptice, sirina_ovire, velikost_odprtine) {
                    self.mode = GameMode::KonecIgre;
                }
            }

            // Dodajanje točk
            if !ovira.mimo && x_ptice > ovira.x + sirina_ovire {
                ovira.zavrzena_ovira(true);
                self.rezultat += 1;
            }
        }
        // Retain obdrži samo tiste elemente, kjer je bool "true", ostale zavrže
        self.ovire.retain(|ovira| !ovira.mimo_zaslona(sirina_ovire));
            
    }
}