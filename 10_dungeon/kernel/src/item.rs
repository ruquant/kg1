#[derive(Clone, PartialEq)]
pub enum Item {
    Sword,
    Potion,
}

impl Item {
    pub fn new_sword() -> Self {
        Self::Sword
    }

    pub fn new_potion() -> Self {
        Self::Potion
    }
}