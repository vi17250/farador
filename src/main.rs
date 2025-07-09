mod personnages;
use personnages::{Arme, Classe, Personnage};
mod map;
use map::parcours;

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
    parcours::parcours();
}
