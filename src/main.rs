mod logika;

use logika::StanjeIgre;
fn main() {

}

// ideja je da je ptica statična na x in se premika samo gor dol po y (kle nastimat maksimum pr vrhu ekrana pa minimum pr tleh) in sicer tko da jo gravitacija ves cas potiska dol razen takrt ko ravno kliknes presledek da skoci (to bo tud
// rabl met cooldown) kjer bos zakrilil tkoda vsak frame se updata y pozicija pa preveri ce smo se zadel v oviro

// ovire nastanejo nekje izven ekrana in se zacnejo premikat levo z konstantno hitrostjo (ko nastane ena bo manjsi delay pol pa nakljucno mal pocakamo pa nastane naslednja), sirina ovir bo konstantna, prov tako vertikalni razmik,
// zato bom rabl samo y (t.j. do kok visoko bo sel spodnji del ovire, spet random med min in max), ki s tem ze natancno doloci kje bo zgornji del ovire, ki bo samo rotirana in zamaknjena vertikalno kopija z istim x
// vsak frame preverjamo ali je ovira prisla mimo ekrana na levi in jo odstrani, ce se je zaletela v ptico (oz. njen hitbox ne celo slikco) in ali je ptica ze mimo ovire (da dodamo 1 rezultatu)

// risanje se ne vem kako zgleda (vrjetn uporabmo nek library)
// risemo ozadje, tla, cevi, model za ptico (ki bo mogoce malo bolj dinamicen tko kt je v pravi igrci)

// ce bo cas lahko naredimo glaven meni (play, settings itd), moznost pavze, highscore, glasba v ozadju (ki se jo izklopi v settings), mogoce vec moznosti za ozadja, ptico