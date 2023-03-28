use std::collections::HashMap;

use super::{Database, DatabaseError};
use serde::{Deserialize, Serialize};

/// Database using sled
#[derive(Clone)]
pub struct SledDatabase {
    inner: sled::Db,
}

#[derive(Serialize, Deserialize)]
struct Node {
    key: String,            // More convenient
    value: Option<Vec<u8>>, // Sometime a node does not have any data
    children: Vec<String>,
}

impl SledDatabase {
    /// Open a connection to the sled database
    pub fn new(path: &str) -> Self {
        let inner = sled::open(path).unwrap();
        Self { inner }
    }

    fn read_node(&self, path: &str) -> Result<Option<Node>, DatabaseError> {
        let res = self.inner.get(path).map_err(|_| DatabaseError::IO)?;
        match res {
            None => Ok(None),
            Some(bytes) => {
                let node =
                    bincode::deserialize(&bytes).map_err(|_| DatabaseError::EncodingError)?;
                Ok(Some(node))
            }
        }
    }

    fn write_node<'a>(&self, node: &'a Node) -> Result<&'a Node, DatabaseError> {
        let bytes = bincode::serialize(&node).map_err(|_| DatabaseError::EncodingError)?;
        let _ = self
            .inner
            .insert(&node.key, bytes)
            .map_err(|_| DatabaseError::IO)?;
        Ok(node)
    }

    /// Retrieve all the subkeys of a key
    ///
    /// For instance, let's assume we habe this path: "/a/b/c/d"
    /// Then the returned hashmap will look like:
    /// HashMap {
    ///    "/"       : ["a"],
    ///    "/a"      : ["b"],
    ///    "/a/b"    : ["c"],
    ///    "/a/b/c"  : ["d"],
    /// }
    fn get_all_subkeys(path: &str) -> HashMap<String, String> {
        let subkeys = HashMap::<String, String>::new();
        let mut splitted = path.split('/');
        let _ = splitted.next();

        let (_, subkeys) =
            splitted.fold(("/".to_string(), subkeys), |(path, mut subkeys), subkey| {
                let next_key = if path == "/" {
                    format!("/{}", subkey)
                } else {
                    format!("{}/{}", path, subkey)
                };

                subkeys.insert(path, subkey.to_string());

                (next_key, subkeys)
            });

        subkeys
    }

    fn add_subkey(&self, path: &str, subkey: &str) -> Result<(), DatabaseError> {
        let node = self.read_node(path)?;
        let node = match node {
            None => Node {
                key: path.to_string(),
                value: None,
                children: vec![subkey.to_string()],
            },
            Some(mut node) => {
                node.children.push(subkey.to_string());
                node
            }
        };
        let _ = self.write_node(&node)?;
        Ok(())
    }
}

impl Database for SledDatabase {
    fn write<'a>(&self, path: &str, data: &'a [u8]) -> Result<&'a [u8], DatabaseError> {
        let subkeys = SledDatabase::get_all_subkeys(path);
        // Creates/Update node's subkeys
        for (path, subkey) in subkeys {
            self.add_subkey(&path, &subkey)?;
        }

        let node = Node {
            key: path.to_string(),
            value: Some(data.to_vec()),
            children: vec![],
        };

        let _ = self.write_node(&node)?;
        Ok(data)
    }

    fn read(&self, path: &str) -> Result<Option<Vec<u8>>, DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(None),
            Some(Node { value, .. }) => Ok(value),
        }
    }

    fn get_subkeys(&self, path: &str) -> Result<Vec<String>, DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(Vec::default()),
            Some(Node { children, .. }) => Ok(children),
        }
    }

    fn delete(&self, path: &str) -> Result<(), DatabaseError> {
        let node = self.read_node(path)?;
        match node {
            None => Ok(()),
            Some(node) => {
                let Node { key, children, .. } = node;

                // Deletes each children
                for child_path in children {
                    self.delete(&child_path)?;
                }

                let _ = self.inner.remove(key).map_err(|_| DatabaseError::IO)?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::database::Database;

    use super::SledDatabase;

    /// Wrapper of the SledDatabase to wipe it at then end of the tests
    struct Db {
        inner: SledDatabase,
        path: String,
    }

    impl AsRef<SledDatabase> for Db {
        fn as_ref(&self) -> &SledDatabase {
            &self.inner
        }
    }

    impl Drop for Db {
        fn drop(&mut self) {
            let _ = fs::remove_dir(&self.path);
        }
    }

    impl Default for Db {
        fn default() -> Self {
            let path = format!("/tmp/{}", uuid::Uuid::new_v4());
            let inner = SledDatabase::new(&path);
            Self { inner, path }
        }
    }

    #[test]
    fn test_write() {
        let database = Db::default();
        let database = database.as_ref();
        let data = [0x01, 0x02, 0x03, 0x04];
        let res = database.write("/path", &data);
        assert!(res.is_ok())
    }

    #[test]
    fn test_read() {
        let database = Db::default();
        let database = database.as_ref();
        let data = [0x01, 0x02, 0x03, 0x04];

        let _ = database.write("/path", &data).unwrap();
        let res = database.read("/path").unwrap().unwrap();

        assert_eq!(res, data.to_vec());
    }

    #[test]
    fn test_delete() {
        let database = Db::default();
        let database = database.as_ref();
        let data = [0x01, 0x02, 0x03, 0x04];

        let _ = database.write("/path", &data).unwrap();
        let () = database.delete("/path").unwrap();
        let res = database.read("/path").unwrap();

        assert!(res.is_none());
    }

    #[test]
    fn test_get_subkeys() {
        let database = Db::default();
        let database = database.as_ref();
        let data = [0x01, 0x02, 0x03, 0x04];

        let _ = database.write("/path/sub", &data).unwrap();
        let root_res = database.get_subkeys("/").unwrap();
        let path_res = database.get_subkeys("/path").unwrap();
        let sub_res = database.get_subkeys("/path/sub").unwrap();

        assert_eq!(root_res, vec!["path"]);
        assert_eq!(path_res, vec!["sub"]);
        assert!(sub_res.is_empty());
    }
}
