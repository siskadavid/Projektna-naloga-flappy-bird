use macroquad::prelude::*;
use macroquad::audio::*;

mod logika;

use logika::{GameMode, StanjeIgre, konstante::*};

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
    // Naložimo slike in zvok
    let ptica_texture: Texture2D = load_texture("slike/ptica.png").await.unwrap();
    let ozadje_texture: Texture2D = load_texture("slike/ozadje.png").await.unwrap();
    let tla_texture: Texture2D = load_texture("slike/tla.png").await.unwrap();
    let pavza_texture: Texture2D = load_texture("slike/pavza.png").await.unwrap();
    let flap_sound: Sound = load_sound("zvok/flap.wav").await.unwrap();
    let fall_sound: Sound = load_sound("zvok/fall.wav").await.unwrap();
    let hit_sound: Sound = load_sound("zvok/hit.wav").await.unwrap();
    let point_sound: Sound = load_sound("zvok/point.wav").await.unwrap();
    // let gary: Sound = load_sound("zvok/Gary.ogg").await.unwrap();

    let mut igra = StanjeIgre::new();

    let scaled_x_ozadja = ozadje_texture.width() * (VISINA_ZASLONA / ozadje_texture.height());      // Macroquad nima možnosti nastavitve ki ohrani aspect ratio, zato ga izračunamo

    // Glavna zanka, kjer se izvaja igra
    loop {
        // Preverimo v katerem stanju je program
        match igra.mode {

            // Program v stanju Menu
            GameMode::Menu => {

                // Predvajanje menu glasbe
                // if !igra.glasba {
                //     play_sound(&gary, PlaySoundParams {
                //         looped: true,
                //         volume: 0.6,
                //     });
                //     igra.glasba = true;
                // }

                // Računanje poziceije ozadja in nihanje ptice
                igra.bg(scaled_x_ozadja);
                igra.ptica.nihanje(get_time());

                // Začetek igre s pritiskom na presledek
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra.mode = GameMode::Igra;
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound);
                }
            }
            
            // Program v stanju Igra
            GameMode::Igra => {

                // Računanje pozicije ptice in ozadja
                igra.bg(scaled_x_ozadja);
                igra.ptica.rotiranje();

                // Kriljenje
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound)
                }

                // Premikanje ptice in ovir
                igra.premikanje();

                // Tukaj še manjka koda za ovire

                // Preverimo ali smo na tleh
                if igra.ptica.pozicija() > VISINA_ZASLONA - 200.0 {
                    igra.mode = GameMode::KonecIgre;
                    play_sound_once(&hit_sound);
                }

                // Preverimo ali pavziramo
                if is_key_pressed(KeyCode::Escape) {
                    igra.mode = GameMode::Pavza
                }
            }

            // Program v stanju pavze
            GameMode::Pavza => {

                // Preverimo ali odpavziramo
                if is_key_pressed(KeyCode::Escape) {
                    igra.mode = GameMode::Igra
                }
            }

            // Program v stanju KonecIgre
            GameMode::KonecIgre => {

                // Tukaj manjka še game over ekran

                // Ponoven zagon igre
                //stop_sound(&gary);

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra = StanjeIgre::new();
                }
            }
        }

        // Risanje ozadja
        draw_texture_ex(                                    // Tla in ozadje se bosta premikala za dodatno iluzijo gibanja, to dosežemo z dvema setoma istih slik ki se premikata po ekranu
            &ozadje_texture,
            igra.ozadje_x, 0.0, WHITE,       // Ne spreminjamo barve na sliki
            DrawTextureParams {
                dest_size: Some(vec2(scaled_x_ozadja, VISINA_ZASLONA)),
                ..Default::default()
            },
        );

        draw_texture_ex(                        // Druga kopija slike ozadja z zamikom
            &ozadje_texture,
            igra.ozadje_x + scaled_x_ozadja, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scaled_x_ozadja, VISINA_ZASLONA)),
                ..Default::default()
            },
        );
        
        // Risanje tal
        draw_texture_ex(
                &tla_texture,
                igra.tla_x, VISINA_ZASLONA - 100.0, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SIRINA_ZASLONA, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &tla_texture,
                igra.tla_x + SIRINA_ZASLONA, VISINA_ZASLONA - 100.0, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SIRINA_ZASLONA, 100.0)),
                    ..Default::default()
                },
            );

        // Risanje ptice
        draw_rectangle(         // Za ptico narišemo njen neviden "hitbox" s katerim bomo preverjali ali se je zadela v oviro
            X_PTICE, igra.ptica.pozicija() + 16.0, SIRINA_PTICE, VISINA_PTICE - 20.0,
            Color::new(1.0, 1.0, 0.0, 0.0)  
        );

        draw_texture_ex(
            &ptica_texture,
            X_PTICE, igra.ptica.pozicija(), WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SIRINA_PTICE, VISINA_PTICE)),
                rotation: igra.ptica.trenutna_rotacija(),
                ..Default::default()
            },
        );

        // Risanje teksta
        match igra.mode {
            GameMode::Menu => {
                draw_text("PRITISNI PRESLEDEK", SIRINA_ZASLONA / 2.0 - 230.0, VISINA_ZASLONA / 2.0 - 50.0, 60.0, WHITE);
            }
            GameMode::KonecIgre => {
                draw_text("GAME OVER", SIRINA_ZASLONA / 2.0 - 125.0, VISINA_ZASLONA / 2.0 - 150.0, 60.0, RED);
                draw_text("Pritisni za ponovni zacetek", SIRINA_ZASLONA / 2.0 - 240.0, VISINA_ZASLONA / 2.0 - 50.0, 40.0, WHITE);
            }
            GameMode::Pavza => {
                draw_texture_ex(
                    &pavza_texture,
                    SIRINA_ZASLONA - 120.0, 20.0, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(100.0, 100.0)),
                        ..Default::default()
                    },
                );
            }
            _ => {}
        }

        next_frame().await              // Async funkcija se izvede enkrat na frame, počakamo na naslednjega
    }
}


// Dodat ovire in točkovanje, high score

// Dodat game over ekran, smrt animacija