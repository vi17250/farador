use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct Handle<T>(Rc<RefCell<T>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Emplacement {
    description: &'static str,
    liens: Vec<Handle<Emplacement>>,
    marked: bool,
}

impl Emplacement {
    fn new(description: &'static str) -> Handle<Self> {
        Handle(Rc::new(RefCell::new(Emplacement {
            description,
            liens: vec![],
            marked: false,
        })))
    }
}

impl Handle<Emplacement> {
    fn link(&self, destination: Handle<Emplacement>) {
        self.0.borrow_mut().liens.push(destination.clone());
        destination.clone().0.borrow_mut().liens.push(self.clone());
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
}
