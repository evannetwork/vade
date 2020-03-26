/*
  Copyright (c) 2018-present evan GmbH.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

//! Module for the [`RustStorageCache`] plugin.
//! 
//! [`RustStorageCache`]: crate::plugin::rust_storage_cache::RustStorageCache

use async_trait::async_trait;
use crate::traits::{ DidResolver, VcResolver };
use simple_error::SimpleError;
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
    /// Checks given DID document.
    /// A DID document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this DID
    /// - that it considers this DID as invalid
    /// 
    /// Currently the test `did_name` `"test"` is accepted as valid.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to check document for
    /// * `value` - value to check
    async fn check_did(&self, did_name: &str, _value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if did_name == "test" {
            println!("valid");
            // accept empty did names (for test)
            return Ok(());
        }
        println!("invalid");
        Err(Box::new(SimpleError::new(format!("not responsible for this did"))))
    }

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
    /// Checks given Vc document.
    /// A Vc document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this Vc
    /// - that it considers this Vc as invalid
    /// 
    /// Currently the test `vc_id` `"test"` is accepted as valid.
    ///
    /// # Arguments
    ///
    /// * `vc_id` - vc_id to check document for
    /// * `value` - value to check
    async fn check_vc(&self, vc_id: &str, _value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if vc_id == "test" {
            println!("valid");
            // accept empty vc names (for test)
            return Ok(());
        }
        println!("invalid");
        Err(Box::new(SimpleError::new(format!("not responsible for this vc"))))
    }

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
