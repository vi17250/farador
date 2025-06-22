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
    arme: Arme,
}

impl Personnage {
    fn new(nom: String, niveau: usize, classe: Classe, stock: usize, arme: Arme) -> Self {
        Personnage {
            nom,
            niveau,
            classe,
            stock,
            arme,
        }
    }
}

fn main() {
    let personnage = Personnage::new("Gardakan".to_string(), 66, Classe::Paladin, 0, Arme::Epee);
    dbg!(personnage);
}

#[allow(warnings)]
mod test {
    use crate::*;

    #[test]
    fn it_creates_a_character() {
        let perso: Personnage =
            Personnage::new("Gardakan".to_string(), 66, Classe::Paladin, 0, Arme::Epee);
        assert_eq!(
            perso,
            Personnage {
                nom: String::from("Gardakan"),
                niveau: 66,
                classe: Classe::Paladin,
                stock: 0,
                arme: Arme::Epee
            }
        );
    }
}
