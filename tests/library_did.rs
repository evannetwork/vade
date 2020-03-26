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

extern crate vade;

use vade::Vade;
use vade::traits::DidResolver;
use vade::plugin::rust_storage_cache::RustStorageCache;


#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_library_set() {
    let mut vade = Vade::new();
    let storage1 = RustStorageCache::new();
    vade.register_did_resolver(Box::from(storage1));
    let storage2 = RustStorageCache::new();
    vade.register_did_resolver(Box::from(storage2));

    match vade.set_did_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = vade.get_did_document("example_key").await.unwrap();
    assert!(fetched == "example_value");
}

#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_storage_set() {
    let mut vade = Vade::new();
    
    let mut storage1 = RustStorageCache::new();
    match storage1.set_did_document("example_key1", "example_value1").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    vade.register_did_resolver(Box::from(storage1));

    let mut storage2 = RustStorageCache::new();
    match storage2.set_did_document("example_key2", "example_value2").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    vade.register_did_resolver(Box::from(storage2));

    let fetched = vade.get_did_document("example_key1").await.unwrap();
    assert!(fetched == "example_value1");
    let fetched = vade.get_did_document("example_key2").await.unwrap();
    assert!(fetched == "example_value2");
}

#[tokio::test]
async fn library_did_can_check_dids() {
    let mut vade = Vade::new();
    let storage = RustStorageCache::new();
    vade.register_did_resolver(Box::from(storage));

    match vade.set_did_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = vade.get_did_document("example_key").await.unwrap();
    
    let is_valid = match vade.check_did("test", &fetched).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(is_valid == true);

    let is_valid = match vade.check_did("unknown", &fetched).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(is_valid == false);
}
