mod personnages;
use emplacements::Emplacement;
use personnages::{Arme, Classe, Personnage};
mod emplacements;

fn main() {
    let mut gardakan = Personnage::new(
        "Gardakan".to_string(),
        66,
        Classe::Paladin,
        0,
        Some(Arme::Epee),
    );

    let mordak = Personnage::new(
        "Mordak".to_string(),
        57,
        Classe::Archimage,
        0,
        Some(Arme::Baton),
    );

    let _boba_fett = Personnage::new(
        "Boba Fett".to_string(),
        1,
        Classe::Voleur,
        0,
        Some(Arme::Dague),
    );

    let _ = mordak.fireball();

    if let Some(Arme::Epee) = gardakan.take_weapon() {
        println!("gardakan vient de se faire voler son épée");
    }

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
    foret.link(colline.clone());
    colline.link(coin_perdu.clone());
    coin_perdu.link(pont.clone());
    pont.link(forteresse);
}
