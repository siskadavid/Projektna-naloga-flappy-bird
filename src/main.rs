use macroquad::prelude::*;

mod logika;

use logika::{GameMode, StanjeIgre};
use logika::konstante::{SIRINA_ZASLONA, VISINA_ZASLONA, SIRINA_PTICE, VISINA_PTICE};

use crate::logika::konstante::{HITROST_OVIRE};


// Nastavitve za okno
fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_width: SIRINA_ZASLONA as i32,
        window_height: VISINA_ZASLONA as i32,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]     // Funkcija pred main ki nam nariše okno še preden preračunamo vse ostalo

async fn main() {
    // Naložimo slike
    let ptica_texture: Texture2D = load_texture("slike/ptica.png").await.unwrap();
    let ozadje_texture: Texture2D = load_texture("slike/ozadje.png").await.unwrap();
    let tla_texture: Texture2D = load_texture("slike/tla.png").await.unwrap();

    let mut igra = StanjeIgre::new();

    let scaled_x_ozadja = ozadje_texture.width() * (VISINA_ZASLONA / ozadje_texture.height());      // Macroquad nima možnosti nastavitve ki ohrani aspect ratio, zato ga izračunamo
    let mut ozadje_x: f32 = 0.0;                    // Tla in ozadje se bosta premikala za dodatno iluzijo gibanja, to dosežemo z dvema setoma istih slik ki se premikata po ekranu
    let mut tla_x: f32 = 0.0;


    // Glavna zanka, kjer se izvaja igra
    loop {
        // Risanje ozadja
        ozadje_x -= HITROST_OVIRE / 10.0;        // Ozadje se premika počasneje kot ovire za občutek globine

        if ozadje_x <= -scaled_x_ozadja {       // Ko pride ena slika ozadja preveč naprej jo prestavimo nazaj
            ozadje_x = 0.0
        }

        draw_texture_ex(
            &ozadje_texture,
            ozadje_x, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scaled_x_ozadja, VISINA_ZASLONA)),
                ..Default::default()
            },
        );

        draw_texture_ex(                        // Druga kopija slike ozadja z zamikom
            &ozadje_texture,
            ozadje_x + scaled_x_ozadja, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scaled_x_ozadja, VISINA_ZASLONA)),
                ..Default::default()
            },
        );
        
        // Risanje tal
        tla_x -= HITROST_OVIRE;

        if tla_x <= -SIRINA_ZASLONA {
            tla_x = 0.0
        }

        draw_texture_ex(                        // Za tla naredimo podobno kot za ozadje
                &tla_texture,
                tla_x, VISINA_ZASLONA - 100.0, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SIRINA_ZASLONA, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &tla_texture,
                tla_x + SIRINA_ZASLONA, VISINA_ZASLONA - 100.0, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SIRINA_ZASLONA, 100.0)),
                    ..Default::default()
                },
            );
        
        // Računanje pozicije ptice
        if is_key_pressed(KeyCode::Space) {
            igra.ptica.kriljenje();
        }

        igra.premikanje();

        // Risanje ptice
        let (ptica_x, ptica_y) = igra.ptica.pozicija();

        let rotacija = if igra.ptica.trenutna_hitrost() < 1.5 {     // Držimo nos ptice gor še nekaj časa po kriljenju
            -0.5 
        } else if igra.ptica.trenutna_hitrost() < 2.0 {                 // Vmesna faza da nos ne skoči preveč
            0.0
        } else {
            igra.ptica.trenutna_hitrost() * 0.2
        };

        draw_rectangle(         // Za ptico narišemo njen neviden "hitbox" s katerim bomo preverjali ali se je zadela v oviro
            ptica_x, ptica_y, SIRINA_PTICE, VISINA_PTICE,
            Color::new(1.0, 1.0, 0.0, 0.0)  
        );

        draw_texture_ex(
            &ptica_texture,
            ptica_x, ptica_y, WHITE,       // Ne spreminjamo barve na sliki
            DrawTextureParams {
                dest_size: Some(vec2(SIRINA_PTICE, VISINA_PTICE)),
                rotation: rotacija,
                ..Default::default()
            },
        );

        next_frame().await              // Async funkcija se izvede enkrat na frame, počakamo na naslednjega
    }
}

// Preverjanje če je y ptice dovolj velik (se dotika tal) da končamo igro (gamestate)

// Ob zagonu programa je ptica najprej na miru v zraku in se premikajo tla (gamestate menu), ko se prvič klikne presledek se začne igra (spremeni tudi gamestate)
// Ponovni zagon igre ob kliku na space

// Dodat ovire in točke in vse