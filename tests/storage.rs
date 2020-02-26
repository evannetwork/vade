extern crate ssi;

use ssi::plugin::rust_storage_cache::RustStorageCache;

#[tokio::test]
async fn storage_can_store_data() {
    let mut storage = RustStorageCache::new();
    match storage.set("asdf", "qwer").await {
        Ok(()) => {
            let fetched = storage.get("asdf").await.unwrap();
            assert!(fetched == "qwer");
        },
        Err(e) => panic!(format!("{}", e)),
    }
}
