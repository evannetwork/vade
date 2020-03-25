extern crate vade;

use vade::plugin::rust_storage_cache::RustStorageCache;

#[tokio::test]
async fn storage_can_store_data() {
    let mut storage = RustStorageCache::new();
    match storage.set("example_key", "example_value").await {
        Ok(()) => {
            let fetched = storage.get("example_key").await.unwrap();
            assert!(fetched == "example_value");
        },
        Err(e) => panic!(format!("{}", e)),
    }
}

#[tokio::test]
async fn get_an_error_when_trying_to_access_mivadeng_keys() {
    let mut storage = RustStorageCache::new();
    match storage.set("example_key", "example_value").await {
        Ok(()) => {
            match storage.get("undefined_key").await {
                Ok(_x) => panic!("should not get an entry here"),
                Err(e) => assert!(format!("{}", e) == "no entry for 'undefined_key'"),
            }
        },
        Err(e) => panic!(format!("{}", e)),
    }
}