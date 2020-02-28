extern crate ssi;

use ssi::library::Library;
use ssi::library::traits::DidResolver;
use ssi::plugin::rust_storage_cache::RustStorageCache;


#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_library_set() {
    let mut library = Library::new();
    let storage1 = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage1));
    let storage2 = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage2));

    match library.set_did_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_did_document("example_key").await.unwrap();
    assert!(fetched == "example_value");
}

#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_storage_set() {
    let mut library = Library::new();
    
    let mut storage1 = RustStorageCache::new();
    match storage1.set_did_document("example_key1", "example_value1").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_did_resolver(Box::from(storage1));

    let mut storage2 = RustStorageCache::new();
    match storage2.set_did_document("example_key2", "example_value2").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_did_resolver(Box::from(storage2));

    let fetched = library.get_did_document("example_key1").await.unwrap();
    assert!(fetched == "example_value1");
    let fetched = library.get_did_document("example_key2").await.unwrap();
    assert!(fetched == "example_value2");
}

#[tokio::test]
async fn library_did_can_check_dids() {
    let mut library = Library::new();
    let storage = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage));

    match library.set_did_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_did_document("example_key").await.unwrap();
    
    let is_valid = match library.check_did("test", &fetched).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(is_valid == true);

    let is_valid = match library.check_did("unknown", &fetched).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert!(is_valid == false);
}
