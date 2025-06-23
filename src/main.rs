#[derive(Debug, PartialEq, Eq)]
enum Classe {
    Paladin,
    Archimage,
    Voleur,
}

#[derive(Debug, PartialEq, Eq)]
enum Arme {
    Epee,
    Baton,
    Dague,
}

#[derive(Debug, PartialEq, Eq)]
struct Personnage {
    nom: String,
    niveau: usize,
    classe: Classe,
    stock: usize,
    arme: Option<Arme>,
}

impl Personnage {
    fn new(nom: String, niveau: usize, classe: Classe, stock: usize, arme: Option<Arme>) -> Self {
        Personnage {
            nom,
            niveau,
            classe,
            stock,
            arme,
        }
    }

    fn fireball(&self) -> Result<(), &'static str> {
        if self.classe == Classe::Archimage {
            return Ok(());
        }
        Err("Seul le mage peut lancer une boule de feu")
    }

    fn take_weapon(&mut self) -> Option<Arme> {
        self.arme.take()
    }
}

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

    let boba_fett = Personnage::new(
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
}

#[allow(warnings)]
mod test {
    use crate::*;

    #[test]
    fn it_creates_a_character_with_a_weapon() {
        let perso: Personnage = Personnage::new(
            "Gardakan".to_string(),
            66,
            Classe::Paladin,
            0,
            Some(Arme::Epee),
        );
        assert_eq!(
            perso,
            Personnage {
                nom: String::from("Gardakan"),
                niveau: 66,
                classe: Classe::Paladin,
                stock: 0,
                arme: Some(Arme::Epee)
            }
        );
    }

    #[test]
    fn it_creates_a_character_without_a_weapon() {
        let perso: Personnage =
            Personnage::new("Gardakan".to_string(), 66, Classe::Paladin, 0, None);
        assert_eq!(
            perso,
            Personnage {
                nom: String::from("Gardakan"),
                niveau: 66,
                classe: Classe::Paladin,
                stock: 0,
                arme: None
            }
        );
    }

    #[test]
    fn it_launches_a_fireball() {
        let perso: Personnage =
            Personnage::new("Gardakan".to_string(), 66, Classe::Archimage, 0, None);
        let shoot = perso.fireball();
        assert_eq!(shoot, Ok(()));
    }

    #[test]
    fn it_cannot_launches_a_fireball() {
        let perso: Personnage =
            Personnage::new("Gardakan".to_string(), 66, Classe::Paladin, 0, None);
        let shoot = perso.fireball();
        assert_eq!(shoot, Err("Seul le mage peut lancer une boule de feu"));
    }

    #[test]
    fn it_returns_weapon() {
        let mut perso: Personnage = Personnage::new(
            "Gardakan".to_string(),
            66,
            Classe::Paladin,
            0,
            Some(Arme::Epee),
        );
        let taken = perso.take_weapon();
        assert_eq!(taken, Some(Arme::Epee));
        assert_eq!(perso.arme, None);
    }

    #[test]
    fn it_returns_none() {
        let mut perso: Personnage =
            Personnage::new("Gardakan".to_string(), 66, Classe::Paladin, 0, None);
        let taken = perso.take_weapon();
        assert_eq!(taken, None);
        assert_eq!(perso.arme, None);
    }
}
