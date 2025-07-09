use crate::map::emplacements::{Emplacement, Handle};
const OBJECTIF: &'static str = "Forteresse";
const A_EVITER: &'static str = "Forêt";

pub fn parcours() {
    let village = Emplacement::new("Village");
    let porcherie = Emplacement::new("Porcherie");
    let foret = Emplacement::new("Forêt");
    let colline = Emplacement::new("Colline");
    let coin_perdu = Emplacement::new("Coin perdu");
    let pont = Emplacement::new("Pont");
    let forteresse = Emplacement::new("Forteresse");

    village.link(porcherie.clone());
    village.link(foret.clone());
    village.link(colline.clone());
    foret.link(pont.clone());
    pont.link(forteresse);
    foret.link(colline.clone());
    colline.link(coin_perdu.clone());
    coin_perdu.link(pont.clone());

    let mut parcours: String = String::from("Parcours ");

    path(village.clone(), &mut parcours);
}

fn path(sommet: Handle<Emplacement>, parcours: &mut String) {
    let name = sommet.description();
    if name == OBJECTIF {
        format_path(parcours, name);
        dbg!(parcours);
        std::process::exit(0)
    }
    sommet.clone().mark();
    let cells = sommet.links();
    if cells.len() != 1 {
        format_path(parcours, name);
    }
    for cell in cells {
        if cell.description() == A_EVITER {
            continue;
        }
        if cell.is_marked() {
            continue;
        } else {
            cell.clone().mark();
            path(cell.clone(), parcours);
        }
    }
}

fn format_path(parcours: &mut String, name: &'static str) {
    parcours.push_str(format!("> {name} ").as_str())
}
