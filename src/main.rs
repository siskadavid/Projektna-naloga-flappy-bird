use macroquad::prelude::*;

mod logika;

use logika::{GameMode, StanjeIgre};
use logika::konstante::{SIRINA_ZASLONA, VISINA_ZASLONA, SIRINA_PTICE, VISINA_PTICE};


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
    let bird_texture: Texture2D = load_texture("slike/ptica.png").await.unwrap();
    let bg_texture: Texture2D = load_texture("slike/ozadje.png").await.unwrap();

    let mut igra = StanjeIgre::new();

    loop {
        clear_background(SKYBLUE);

        // Risanje ozadja
        draw_texture_ex(
            &bg_texture,
            0.0, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(bg_texture.width() * (VISINA_ZASLONA / bg_texture.height()), VISINA_ZASLONA)),     // Višino ozadja nastavimo na višino zaslona in izračunamo kakšna mora biti širina ozdaja da ohranimo razmerje
                ..Default::default()
            },
        );

        let dt = get_frame_time();              // Frekvenca osveževanja zaslona

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
            ptica_x,
            ptica_y,
            SIRINA_PTICE,
            VISINA_PTICE,
            Color::new(1.0, 1.0, 0.0, 0.0)  
        );

        draw_texture_ex(        // Narišemo ptico
            &bird_texture,
            ptica_x,
            ptica_y,
            WHITE,       // Ne spreminjamo barve na sliki
            DrawTextureParams {
                dest_size: Some(vec2(SIRINA_PTICE, VISINA_PTICE)),
                rotation: rotacija,
                ..Default::default()
            },
        );

        next_frame().await              // Async funkcija se izvede enkrat na frame, počakamo na naslednjega
    }
}

// Dodat teksturo za tla ki se bo navidezno premikala (usklajeno z ovirami)
// Preverjanje če je y ptice dovolj velik (se dotika tal) da končamo igro (gamestate)

// Ob zagonu programa je ptica najprej na miru v zraku in se premikajo tla (gamestate menu), ko se prvič klikne presledek se začne igra (spremeni tudi gamestate)
// Ponovni zagon igre ob kliku na space

// Dodat ovire in točke in vse