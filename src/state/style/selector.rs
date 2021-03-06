use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::string::ToString;

use crate::Specificity;

// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// pub enum PseudoClass {
//     None,
//     Hover,
//     Over,
//     Active,
//     Focus,
//     Enabled,
//     Disabled,
//     Checked,
// }

// 0 - Hover
// 1 - Over
// 2 - Active
// 3 - Focus
// 4 - Enabled
// 5 - Disabled
// 6 - Checked
// 7 - Unassigned

#[derive(Debug, Clone)]
pub struct PseudoClasses(u8);

impl Default for PseudoClasses {
    fn default() -> Self {
        PseudoClasses(0)
    }
}

impl PseudoClasses {
    pub fn new() -> Self {
        PseudoClasses(0)
    }

    pub fn set_hover(&mut self, flag: bool) {
        if flag {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }

    pub fn set_over(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 1);
        } else {
            self.0 &= !(1 << 1);
        }
    }

    pub fn set_active(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 2);
        } else {
            self.0 &= !(1 << 2);
        }
    }

    pub fn set_focus(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 3);
        } else {
            self.0 &= !(1 << 3);
        }
    }

    pub fn set_enabled(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 4);
        } else {
            self.0 &= !(1 << 4);
        }
    }

    pub fn set_disabled(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 5);
        } else {
            self.0 &= !(1 << 5);
        }
    }

    pub fn set_checked(&mut self, flag: bool) {
        if flag {
            self.0 |= (1 << 6);
        } else {
            self.0 &= !(1 << 6);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Relation {
    None,
    Ancestor,
    Parent,
}



#[derive(Clone, Debug)]
pub struct Selector {
    pub id: Option<u64>,
    pub element: Option<u64>,
    pub classes: HashSet<String>,
    //pub pseudo_classes: HashSet<PseudoClass>,
    pub pseudo_classes: PseudoClasses,
    pub relation: Relation,
    pub asterisk: bool,
}

impl Default for Selector {
    fn default() -> Selector {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
            relation: Relation::None,
            asterisk: false,
        }
    }
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn from(element: &str) -> Self {
        let mut s = DefaultHasher::new();
        element.hash(&mut s);

        Selector {
            id: None,
            element: Some(s.finish()),
            classes: HashSet::new(),
            //pseudo_classes: HashSet::new(),
            pseudo_classes: PseudoClasses::default(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn matches(&self, entity_selector: &Selector) -> bool {

        // Universal selector always matches
        if self.asterisk {
            return true;
        }

        if self.id.is_some() && self.id != entity_selector.id {
            //println!("id doesn't match");
            return false;
        }

        if self.element.is_some() && self.element != entity_selector.element {
            //println!("element doesn't match");
            return false;
        }

        if !self.classes.is_subset(&entity_selector.classes) {
            //println!("classes doesn't match");
            return false;
        }

        // if !self.pseudo_classes.is_subset(&other.pseudo_classes) {
        //     //println!("classes doesn't match");
        //     return false;
        // }

        //println!("Selector: {:?}  Widget: {:?}  Combined: 0b{:08b}", self.pseudo_classes, other.pseudo_classes, (self.pseudo_classes.0 & other.pseudo_classes.0));

        if self.pseudo_classes.0 != 0 && (self.pseudo_classes.0 & entity_selector.pseudo_classes.0) == 0 {
            return false;
        }

        if self.asterisk != entity_selector.asterisk {
            return false;
        }

        // if !other.classes.is_subset(&self.classes) {
        //     //println!("classes doesn't match");
        //     return false;
        // }

        // if !other.pseudo_classes.is_subset(&self.pseudo_classes) {
        //     //println!("pseudoclasses doesn't match");
        //     return false;
        // }

        true
    }

    // pub fn specificity(&self) -> usize {
    //     (if self.id.is_some() { 1000 } else { 0 })
    //         + (self.classes.len() * 100)
    //         + (self.pseudo_classes.len() * 100)
    //         + (if self.element.is_some() { 1 } else { 0 })
    // }

    pub fn specificity(&self) -> Specificity {
        Specificity([
            if self.id.is_some() { 1 } else { 0 },
            //(self.classes.len() + self.pseudo_classes.len()) as u8,
            (self.classes.len() + self.pseudo_classes.0.count_ones() as usize) as u8,
            if self.element.is_some() { 1 } else { 0 },
        ])
    }

    pub fn id(mut self, id: &str) -> Self {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        self.id = Some(s.finish());
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self
    }

    // pub fn pseudo_class(mut self, pseudo_class: PseudoClass) -> Self {
    //     self.pseudo_classes.insert(pseudo_class);
    //     self
    // }

    pub fn replace_class(&mut self, old: &str, new: &str) -> &mut Self {
        self.classes.remove(old);
        self.classes.insert(new.to_string());

        self
    }

    pub fn set_id(&mut self, id: &str) -> &mut Self {
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        self.id = Some(s.finish());
        self
    }

    pub fn set_element(&mut self, element: &str) -> &mut Self {
        let mut s = DefaultHasher::new();
        element.hash(&mut s);
        self.element = Some(s.finish());
        self
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Selector) -> bool {
        self.matches(other)
    }
}
