use macroquad::rand::gen_range; // naključno višino odprtin ovir

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
    pub stevec_slicic: u32, //stevec frameov (idejno)
    pub ozadje_x: f32,
    pub tla_x: f32,
    pub glasba: bool
}

impl StanjeIgre {
    // Nova igra
    pub fn new() -> Self{
        StanjeIgre {ptica: Ptica::new(), 
            ovire: Vec::new(), 
            mode: GameMode::Menu, 
            rezultat: 0, 
            ozadje_x: 0.0, 
            tla_x: 0.0, 
            glasba: false, 
            stevec_slicic: 0
        }
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
        if self.mode != GameMode::Igra{
            return;
        }
            self.ptica.gravitacija();
            self.ptica.rotiranje();

            let p_y = self.ptica.pozicija(); // nova pozicija
            if p_y + VISINA_PTICE > VISINA_ZASLONA - 100.0 {
                self.mode = GameMode::KonecIgre;            //če trči v tla se igra konča
            }

            self.stevec_slicic += 1;
            if self.stevec_slicic % 120 == 0 {
                let y_odprtine = gen_range(300.0,  VISINA_ZASLONA - 300.0);
                self.ovire.push(Ovire::new(ZACETNI_X_OVIRE, y_odprtine));
            }
            
            // premik vseh ovir v levo
            for ovira in self.ovire.iter_mut(){
                ovira.premik_cevi();
                
            // preveri trk
                if ovira.preveri_trk(p_y) {
                    self.mode = GameMode::KonecIgre;
                }

                if! ovira.mimo_ptice() && X_PTICE > ovira.x + SIRINA_OVIRE {
                    ovira.zavrzena_ovira(true);
                    self.rezultat += 1;
                }
            }

            // retain obdrži samo tiste elemente, kjer je bool 'true', druge zavrže
            self.ovire.retain(|ovira| !ovira.mimo_zaslona());
            
    }
}