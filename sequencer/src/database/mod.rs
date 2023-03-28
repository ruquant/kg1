pub mod sled;

#[derive(Debug)]
pub enum DatabaseError {
    EncodingError,
    IO,
}

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
}
