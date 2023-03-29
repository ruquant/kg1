use serde::{Deserialize, Serialize};

pub mod sled;

#[derive(Debug)]
pub enum DatabaseError {
    EncodingError,
    IO,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    key: String,            // More convenient
    value: Option<Vec<u8>>, // Sometime a node does not have any data
    children: Vec<String>,  // Array of subkeys
}

/// The database should act ha a tree
pub trait Database: Clone {
    /// Write the given data to the given path
    fn write<'a>(&self, path: &str, data: &'a [u8]) -> Result<&'a [u8], DatabaseError>;

    /// Read the data from the given path
    fn read(&self, path: &str) -> Result<Option<Vec<u8>>, DatabaseError>;

    /// Returns the subkeys of at the given path
    fn get_subkeys(&self, path: &str) -> Result<Vec<String>, DatabaseError>;

    /// Deletes the data at a given path
    ///
    /// It also deletes all the subkeys
    fn delete(&self, path: &str) -> Result<(), DatabaseError>;

    /// Read a node
    fn read_node(&self, path: &str) -> Result<Option<Node>, DatabaseError>;
}

impl Node {
    /// Get the children of the node
    pub fn children(&self) -> &[String] {
        &self.children
    }

    /// Get the value of the node
    pub fn value(&self) -> &Option<Vec<u8>> {
        &self.value
    }
}
