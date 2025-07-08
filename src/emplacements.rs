use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Handle<T>(Rc<RefCell<T>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Emplacement {
    description: &'static str,
    liens: Vec<Handle<Emplacement>>,
    marked: bool,
}

impl Emplacement {
    pub fn new(description: &'static str) -> Handle<Self> {
        Handle(Rc::new(RefCell::new(Emplacement {
            description,
            liens: vec![],
            marked: false,
        })))
    }
}

impl Handle<Emplacement> {
    pub fn link(&self, destination: Handle<Emplacement>) {
        self.0.borrow_mut().liens.push(destination.clone());
        destination.0.borrow_mut().liens.push(self.clone());
    }

    pub fn description(&self) -> &'static str {
        &self.0.borrow().description
    }

    pub fn mark(&mut self) {
        *&mut self.0.borrow_mut().marked = true;
    }

    pub fn is_marked(&self) -> bool {
        self.0.borrow().marked
    }

    pub fn links(&self) -> Vec<Self> {
        self.0.borrow().liens.clone()
    }

    fn next(&mut self) {
        self.0.borrow_mut().marked = true;
        let binding = self.0.borrow().clone();
        let destinations = binding.liens.iter().find(|lien| !lien.0.borrow().marked);
        match destinations {
            Some(destination) => *self = destination.clone(),
            None => (),
        }
    }
}

#[allow(warnings)]
mod test {
    use emplacements::{Emplacement, Handle};
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::*;

    #[test]
    fn it_creates_an_emplacement() {
        let source = Emplacement::new("Source");
        let source_ = Rc::new(RefCell::new(Emplacement {
            description: "Source",
            liens: vec![],
            marked: false,
        }));
        assert_eq!(source, Handle(source_));
    }

    #[test]
    fn it_links_two_emplacements() {
        let source = Emplacement::new("Source");
        let destination = Emplacement::new("Destination");
        source.link(destination.clone());
        assert_eq!(
            source.0.borrow().liens[0].clone().0.borrow().description,
            "Destination"
        );
        assert_eq!(
            destination.0.borrow().liens[0]
                .clone()
                .0
                .borrow()
                .description,
            "Source"
        );
    }

    #[test]
    fn it_runs_through_a_link() {
        let source = Emplacement::new("Source");
        let destination_1 = Emplacement::new("Dest_1");
        let destination_2 = Emplacement::new("Dest_2");
        let mut current_position = source.clone();
        source.link(destination_1.clone());
        destination_1.link(destination_2);
        current_position.next();
        assert_eq!(source.0.borrow().marked, true);
        assert_eq!(current_position.0.borrow().description, "Dest_1");
        current_position.next();
        assert_eq!(destination_1.0.borrow().marked, true);
        assert_eq!(current_position.0.borrow().description, "Dest_2");
    }
}
