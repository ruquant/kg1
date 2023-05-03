#[derive(Clone, PartialEq)]
pub enum Item {
    Sword,
    Potion,
}

impl Item {
    #[allow(path_statements)]
    #[allow(dead_code)]
    pub fn new(sword: Item, potion: Item) -> Self {
        sword;
        potion
    }
}
