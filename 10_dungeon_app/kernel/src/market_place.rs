use crate::item::Item;
use std::collections::HashMap;

// Define State
#[derive(Clone, PartialEq)]
pub struct MarketPlace {
    // FA2 contract
    // key:(player_address, item_id), value
    pub inner: HashMap<(String, Item), usize>,
}

impl Default for MarketPlace {
    fn default() -> Self {
        // From the beginning the Marketplace is empty
        let inner = HashMap::new();
        MarketPlace { inner }
    }
}

impl MarketPlace {
    pub fn new() -> Self {
        Self::default()
    }
}
