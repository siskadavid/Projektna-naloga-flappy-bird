use macroquad::prelude::*;
use macroquad::audio::*;

mod logika;
use logika::{GameMode, StanjeIgre};

// Nastavitve za okno
fn okno_konfiguracija() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_string(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}
#[macroquad::main(okno_konfiguracija)]     // Funkcija pred main ki nam nariše okno še preden preračunamo vse ostalo

async fn main() {

    // Naložimo slike in zvok
    let ptica_texture: Texture2D = load_texture("slike/ptica.png").await.unwrap();
    let ozadje_texture: Texture2D = load_texture("slike/ozadje.png").await.unwrap();
    let tla_texture: Texture2D = load_texture("slike/tla.png").await.unwrap();
    let pavza_texture: Texture2D = load_texture("slike/pavza.png").await.unwrap();
    let ovira_texture: Texture2D = load_texture("slike/ovira.png").await.unwrap();
    let flap_sound: Sound = load_sound("zvok/flap.wav").await.unwrap();
    let fall_sound: Sound = load_sound("zvok/fall.wav").await.unwrap();
    let hit_sound: Sound = load_sound("zvok/hit.wav").await.unwrap();
    let point_sound: Sound = load_sound("zvok/point.wav").await.unwrap();
    // let gary: Sound = load_sound("zvok/Gary.ogg").await.unwrap();

    // Spremenljivke na začetku igre
    let zacetni_y_ptice = screen_height() * 0.25;
    let mut igra = StanjeIgre::new(zacetni_y_ptice);
    let mut trenutni_rezultat = 0;

    let mut prilagojeno_okno = false;

    // Glavna zanka, kjer se izvaja igra
    loop {

        // Nastavimo velikost okna
        if !prilagojeno_okno {

            let (zaslon_sirina, zaslon_visina) = miniquad::window::screen_size();

            let okno_sirina = (zaslon_sirina * 0.35) as f32;
            let okno_visina = (zaslon_visina * 0.85) as f32;

            set_fullscreen(false);
            request_new_screen_size(okno_sirina, okno_visina);
            prilagojeno_okno = true;
        }

        // Izračunane razne konstante, ki so odvisne od velikosti zaslona
        let x_ptice = screen_width() * 0.25;
        let zacetni_y_ptice = screen_height() * 0.25;
        let visina_ptice = screen_height() * 0.05;
        let sirina_ptice = ptica_texture.width() * (screen_height() * 0.05 / ptica_texture.height());       // Macroquad nima možnosti nastavitve ki ohrani aspect ratio, zato ga izračunamo

        let zacetni_x_ovire = screen_width() * 1.2;
        let visina_ovire = screen_height() * 0.5;
        let sirina_ovire = ovira_texture.width() * (screen_height() * 0.5 / ovira_texture.height());
        let velikost_odprtine = visina_ptice * 4.0;
        
        let sirina_ozadja = ozadje_texture.width() * (screen_height() / ozadje_texture.height());


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

                // Računanje pozicije ozadja in nihanje ptice
                igra.bg(sirina_ozadja, screen_width());
                igra.ptica.nihanje(get_time(), zacetni_y_ptice);

                // Začetek igre s pritiskom na presledek
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra.mode = GameMode::Igra;
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound);
                }
            }
            
            // Program v stanju Igra
            GameMode::Igra => {

                // Kriljenje
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound)
                }

                // Računanje pozicije ptice in ozadja
                igra.bg(sirina_ozadja, screen_width());

                // Premikanje ptice in ovir
                igra.premikanje_ptice();
                igra.premikanje_ovir(screen_height(), x_ptice, sirina_ptice, visina_ptice, sirina_ovire, visina_ovire, velikost_odprtine, zacetni_x_ovire);

                // Preverimo ali smo dobili točko
                if igra.rezultat > trenutni_rezultat {
                    play_sound_once(&point_sound);
                    trenutni_rezultat += 1;
                }

                // Preverimo ali smo na tleh
                if igra.ptica.y > screen_height() * 0.915 - visina_ptice {
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

                // stop_sound(&gary);

                // Ponoven zagon igre
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    igra = StanjeIgre::new(zacetni_y_ptice);
                }
            }
        }

        // Risanje ozadja
        draw_texture_ex(                                    // Tla in ozadje se bosta premikala za dodatno iluzijo gibanja, to dosežemo z dvema setoma istih slik ki se premikata po ekranu
            &ozadje_texture,
            igra.ozadje_x, 0.0, WHITE,       // Ne spreminjamo barve na sliki
            DrawTextureParams {
                dest_size: Some(vec2(sirina_ozadja, screen_height())),
                ..Default::default()
            },
        );

        draw_texture_ex(                        // Druga kopija slike ozadja z zamikom
            &ozadje_texture,
            igra.ozadje_x + sirina_ozadja, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(sirina_ozadja, screen_height())),
                ..Default::default()
            },
        );

        // Risanje ovir
        for ovira in igra.ovire.iter_mut() {
            draw_texture_ex(
                &ovira_texture,
                ovira.x, ovira.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(sirina_ovire, visina_ovire)),
                    flip_y: ovira.obrnjena,              // Eno od ovir obrnemo na glavo
                    ..Default::default()
                    },
            )
        }
        
        // Risanje tal
        draw_texture_ex(
                &tla_texture,
                igra.tla_x, screen_height() * 0.915, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height() * 0.1)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &tla_texture,
                igra.tla_x + screen_width(), screen_height() * 0.915, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height() * 0.1)),
                    ..Default::default()
                },
            );

        // Risanje ptice
        draw_texture_ex(
            &ptica_texture,
            x_ptice, igra.ptica.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(sirina_ptice, visina_ptice)),
                rotation: igra.ptica.rotacija,
                ..Default::default()
            },
        );

        // Risanje teksta
        match igra.mode {
            GameMode::Menu => {
                draw_text("PRITISNI PRESLEDEK", screen_width() * 0.22, screen_height() * 0.4, screen_height() * 0.048, WHITE);
            }
            GameMode::KonecIgre => {
                draw_text("GAME OVER", screen_width() * 0.3, screen_height() * 0.3, screen_height() * 0.065, RED);
                draw_text("Pritisni za ponovni zacetek", screen_width() * 0.17, screen_height() * 0.35, screen_height() * 0.04, WHITE);
            }
            GameMode::Pavza => {
                draw_texture_ex(
                    &pavza_texture,
                    screen_width() * 0.84, screen_height() * 0.024, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_height() * 0.1, screen_height() * 0.1)),
                        ..Default::default()
                    },
                );
            }
            _ => {}
        }

        next_frame().await              // Async funkcija se izvede enkrat na frame, počakamo na naslednjega
    }
}


// točkovanje, high score, dodat game over pa pavza ekran

// death na ceveh, odvisnost od frame rata, pavza na klik