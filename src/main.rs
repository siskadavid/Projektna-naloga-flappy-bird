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
#[macroquad::main(okno_konfiguracija)]

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
    let gary: Sound = load_sound("zvok/gary.ogg").await.unwrap();       // Največji file, zaradi njega traja loadanje na zacetku

    // Spremenljivke na začetku igre
    let zacetni_y_ptice = screen_height() * 0.25;
    let mut x_ptice = 0.0;
    let mut visina_ptice = 0.0;
    let mut sirina_ptice = 0.0;

    let mut zacetni_x_ovire = 0.0;
    let mut visina_ovire = 0.0;
    let mut sirina_ovire = 0.0;
    let mut velikost_odprtine = 0.0;

    let mut sirina_ozadja = 0.0;

    let mut igra = StanjeIgre::new(zacetni_y_ptice);
    let mut prilagojeno_okno = false;

    let mut trenutni_rezultat = 0;
    let mut rekord = 0;

    let mut casovnik_smrti = 0.7; 
    let mut cakam_na_restart = true;

    // Glavna zanka
    loop {

        // Uporabimo delta time za neodvisnost od osveževalnega časa
        let dt = get_frame_time();

        // Preverjamo skok in pavzo takoj na zacetku
        let skok = is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left);
        let pavza = is_key_pressed(KeyCode::Escape);

        // Prvič v loopu nastavimo velikost okna
        if !prilagojeno_okno {

            // Pridobimo velikost celotnega ekrana
            let (zaslon_sirina, zaslon_visina) = miniquad::window::screen_size();

            // Nastavimo velikost okna
            let okno_sirina = (zaslon_sirina * 0.35).round();
            let okno_visina = (zaslon_visina * 0.85).round();
            set_fullscreen(false);
            request_new_screen_size(okno_sirina, okno_visina);
            prilagojeno_okno = true;

            // Izračunamo razne konstante, ki so odvisne od velikosti zaslona
            x_ptice = (okno_sirina * 0.25).round();
            visina_ptice = (okno_visina * 0.05).round();
            sirina_ptice = (ptica_texture.width() * (okno_visina * 0.05 / ptica_texture.height())).round();       // Macroquad nima možnosti nastavitve ki ohrani aspect ratio, zato ga izračunamo
            zacetni_x_ovire = (okno_sirina * 1.2).round();
            visina_ovire = (okno_visina * 0.7).round();
            sirina_ovire = (ovira_texture.width() * (okno_visina * 0.7 / ovira_texture.height())).round();
            velikost_odprtine = (visina_ptice * 4.0).round();
            sirina_ozadja = (ozadje_texture.width() * (okno_visina / ozadje_texture.height())).round();
        }

        match igra.mode {
            GameMode::Menu => {

                //Predvajanje menu glasbe
                if !igra.glasba {
                    play_sound(&gary, PlaySoundParams {
                        looped: true,
                        volume: 0.6,
                    });
                    igra.glasba = true;
                }

                // Računanje pozicije ozadja in nihanje ptice
                igra.bg(sirina_ozadja, screen_width(), dt);
                igra.ptica.nihanje(get_time() as f32, zacetni_y_ptice);

                // Začetek igre s pritiskom na presledek
                if skok {
                    igra.mode = GameMode::Igra;
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound);
                }
            }

            GameMode::Igra => {

                // Kriljenje
                if skok {
                    igra.ptica.kriljenje();
                    play_sound_once(&flap_sound)
                }

                // Računanje pozicije ptice, ozadja in ovir
                igra.bg(sirina_ozadja, screen_width(), dt);
                igra.premikanje_ptice(dt);
                igra.premikanje_ovir(screen_height(), x_ptice, sirina_ovire, visina_ovire, velikost_odprtine, zacetni_x_ovire, dt);

                // Preverimo ali smo dobili točko
                if igra.rezultat > trenutni_rezultat {
                    play_sound_once(&point_sound);
                    trenutni_rezultat += 1;
                }

                //Preverimo highscore
                if trenutni_rezultat > rekord {
                    rekord += 1;
                }
                
                // Preverimo ali smo na tleh
                if igra.ptica.y > screen_height() * 0.915 - visina_ptice {
                    trenutni_rezultat = 0;
                    play_sound_once(&hit_sound);

                    casovnik_smrti = 0.7; 
                    cakam_na_restart = true;

                    igra.mode = GameMode::KonecIgre;
                }

                // Preverimo trk z ovirami
                if igra.preveri_trk(x_ptice, sirina_ptice, visina_ptice, sirina_ovire, velikost_odprtine) {
                    trenutni_rezultat = 0;
                    play_sound_once(&hit_sound);
                    play_sound_once(&fall_sound);

                    casovnik_smrti = 0.7; 
                    cakam_na_restart = true;

                    igra.mode = GameMode::KonecIgre;
                }

                // Preverimo ali pavziramo
                if pavza {
                    igra.mode = GameMode::Pavza
                }
            }

            GameMode::Pavza => {

                // Preverimo ali odpavziramo
                if pavza {
                    igra.mode = GameMode::Igra
                }
            }

            GameMode::KonecIgre => {

                // Prenehamo predvajanje glasbe
                stop_sound(&gary);

                //Ptica pade na tla
                if igra.ptica.y < screen_height() * 0.915 - visina_ptice {
                    igra.premikanje_ptice(dt)
                }

                // Cooldown da ne začnemo nove igre takoj ponesreči
                if cakam_na_restart {
                    casovnik_smrti -= dt;
                
                    if casovnik_smrti <= 0.0 {
                        cakam_na_restart = false;
                    }
                }

                // Ponoven zagon igre
                if skok && !cakam_na_restart {
                    igra = StanjeIgre::new(zacetni_y_ptice);
                }
            }
        }

        // Risanje ozadja
        draw_texture_ex(                                    // Tla in ozadje se bosta premikala za dodatno iluzijo gibanja, to dosežemo z dvema setoma istih slik ki se premikata po ekranu
            &ozadje_texture,
            igra.ozadje_x, 0.0, WHITE,
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

            GameMode::Igra => {
                draw_text(&format!("{}", igra.rezultat), screen_width() * 0.5, screen_height() * 0.09, screen_height() * 0.08, WHITE);
            }

            GameMode::KonecIgre => {
                draw_text("GAME OVER", screen_width() * 0.21, screen_height() * 0.3, screen_height() * 0.1, RED);
                draw_text(&format!("TOCKE: {}  REKORD: {}", igra.rezultat, rekord), screen_width() * 0.2, screen_height() * 0.35, screen_height() * 0.05, WHITE);
                draw_text("Pritisni za ponovni zacetek", screen_width() * 0.17, screen_height() * 0.42, screen_height() * 0.04, WHITE);
            }

            GameMode::Pavza => {
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));      // Zatemnitev ekrana
                draw_texture_ex(
                    &pavza_texture,
                    screen_width() * 0.84, screen_height() * 0.024, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_height() * 0.1, screen_height() * 0.1)),
                        ..Default::default()
                    },
                );
            }
        }

        next_frame().await
    }
}


//lepsa koda, music toggle