use async_trait::async_trait;
use crate::library::traits::{ DidResolver, VcResolver };
use std::collections::HashMap;

/// in-memory storage
pub struct RustStorageCache {
    /// key-value mapping to hold data
    storage: HashMap<String, String>,
}

impl RustStorageCache {
    /// Creates new RustStorageCache instance
    pub fn new() -> RustStorageCache {
        RustStorageCache {
            storage: HashMap::new(),
        }
    }

    /// Get value for given key from storage.
    ///
    /// # Arguments
    ///
    /// * `key` - id of value to fetch
    pub async fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        match self.storage.get(&String::from(key)) {
            Some(entry) => Ok(entry.to_string()),
            None => bail!(format!("no entry for '{}'", key)),
        }
    }

    /// Sets given value for given key.
    ///
    /// # Arguments
    ///
    /// * `key` - id of value to set
    /// * `value` - value to set
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.storage.insert(String::from(key), String::from(value)) {
            Some(_) | None => Ok(()),
        }
    }
}

#[async_trait]
impl DidResolver for RustStorageCache {
    /// Gets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to fetch
    async fn get_did_document(&self, did_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.get(did_id).await
    }
    
    /// Sets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to set value for
    /// * `value` - value to set
    async fn set_did_document(&mut self, did_id: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.set(did_id, value).await
    }
}

#[async_trait]
impl VcResolver for RustStorageCache {
    /// Gets document for given vc.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to fetch
    async fn get_vc_document(&self, vc_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.get(vc_name).await
    }
    
    /// Sets document for given vc name.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to set value for
    /// * `value` - value to set
    async fn set_vc_document(&mut self, vc_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.set(vc_name, value).await
    }
}
