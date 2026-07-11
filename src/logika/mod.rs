use macroquad::{rand::gen_range};

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
    pub casovnik: f32,
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
    pub fn bg(&mut self, sirina_ozadja: f32, sirina_zaslona: f32, dt: f32) {

        self.ozadje_x -= (HITROST_OVIRE / 10.0) * dt;        // Ozadje se premika počasneje kot ovire za občutek globin

        if self.ozadje_x <= -sirina_ozadja {       // Ko pride ena slika ozadja preveč naprej jo prestavimo nazaj
            self.ozadje_x = 0.0
        }
        
        self.tla_x -= HITROST_OVIRE * dt;
        if self.tla_x <= -sirina_zaslona {
            self.tla_x = 0.0
        }
    }

    // Računanje premikov ptice in ovir
    pub fn premikanje_ptice(&mut self, dt: f32) {
        self.ptica.gravitacija(dt);
        self.ptica.rotiranje();
    }

    // Preveri trk
    pub fn preveri_trk(&mut self, x_ptice: f32, sirina_ptice: f32, visina_ptice: f32, sirina_ovire: f32, velikost_odprtine: f32) -> bool {
        for ovira in self.ovire.iter_mut(){
            if !ovira.obrnjena && ovira.ovira_trk(x_ptice,self.ptica.y, sirina_ptice, visina_ptice, sirina_ovire, velikost_odprtine) {
                return true
            }
        }
        false
    }

    pub fn premikanje_ovir(&mut self, visina_zaslona: f32, x_ptice: f32, sirina_ovire: f32, visina_ovire: f32, velikost_odprtine: f32, zacetni_x_ovire: f32, dt: f32) {
        self.casovnik += dt;
        let cev_cooldown = 1.5;

        // Na vsakih cel_cooldown sekund ustvarimo nov par cevi
        if self.casovnik >= cev_cooldown {
            let tla_y = visina_zaslona * 0.915;
            let min_vidna_cev = visina_zaslona * 0.2;
            let max_y_odprtine = tla_y - velikost_odprtine - min_vidna_cev;

            let y_odprtine = gen_range(min_vidna_cev,  max_y_odprtine);

            self.ovire.push(Ovire::new(zacetni_x_ovire, y_odprtine - visina_ovire, true));      // Zgornja cev
            self.ovire.push(Ovire::new(zacetni_x_ovire, y_odprtine + velikost_odprtine, false));   // Spodnja cev

            self.casovnik -= cev_cooldown
        }
        
        // Premik vseh ovir v levo
        for ovira in self.ovire.iter_mut(){
            ovira.premik_cevi(dt);

            // Dodajanje točk
            if !ovira.mimo && x_ptice > ovira.x + sirina_ovire {     // Prištevamo rezultat samo od ene ovire
                ovira.mimo = true;
                if ovira.obrnjena {
                    self.rezultat += 1;
                }
            }
        }
        
        // Retain obdrži samo tiste elemente, kjer je bool = true, ostale zavrže
        self.ovire.retain(|ovira| ovira.mimo_zaslona(sirina_ovire));
            
    }
}