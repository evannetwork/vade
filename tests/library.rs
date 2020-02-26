extern crate ssi;

use ssi::library::Library;
use ssi::library::traits::{ DidResolver };
use ssi::plugin::rust_storage_cache::RustStorageCache;

#[test]
fn library_can_be_created() {
    let _library = Library::new();
}

#[tokio::test]
async fn library_did_can_set_dids() {
    let mut library = Library::new();
    let storage = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage));

    match library.set_did_document("asdf", "qwer").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_did_document("asdf").await.unwrap();
    assert!(fetched == "qwer");
}

#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_library_set() {
    let mut library = Library::new();
    let storage1 = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage1));
    let storage2 = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage2));

    match library.set_did_document("asdf", "qwer").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_did_document("asdf").await.unwrap();
    assert!(fetched == "qwer");
}

#[tokio::test]
async fn library_did_can_set_dids_with_two_resolvers_via_storage_set() {
    let mut library = Library::new();
    
    let mut storage1 = RustStorageCache::new();
    match storage1.set_did_document("asdf1", "qwer1").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_did_resolver(Box::from(storage1));

    let mut storage2 = RustStorageCache::new();
    match storage2.set_did_document("asdf2", "qwer2").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_did_resolver(Box::from(storage2));

    let fetched = library.get_did_document("asdf1").await.unwrap();
    assert!(fetched == "qwer1");
    let fetched = library.get_did_document("asdf2").await.unwrap();
    assert!(fetched == "qwer2");
}