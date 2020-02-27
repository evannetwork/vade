extern crate ssi;

use ssi::library::Library;
use ssi::library::traits::{ DidResolver, VcResolver };
use ssi::plugin::rust_storage_cache::RustStorageCache;

//////////////////////////////////////////////////////////////////// library init
#[test]
fn library_can_be_created() {
    let _library = Library::new();
}

#[tokio::test]
async fn library_did_can_set_dids() {
    let mut library = Library::new();
    let storage = RustStorageCache::new();
    library.register_did_resolver(Box::from(storage));

    match library.set_did_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_did_document("example_key").await.unwrap();
    assert!(fetched == "example_value");
}

//////////////////////////////////////////////////////////////////// did resolver
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

///////////////////////////////////////////////////////////////////// vc resolver
#[tokio::test]
async fn library_vc_can_set_vcs_with_two_resolvers_via_library_set() {
    let mut library = Library::new();
    let storage1 = RustStorageCache::new();
    library.register_vc_resolver(Box::from(storage1));
    let storage2 = RustStorageCache::new();
    library.register_vc_resolver(Box::from(storage2));

    match library.set_vc_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_vc_document("example_key").await.unwrap();
    assert!(fetched == "example_value");
}

#[tokio::test]
async fn library_vc_can_set_vcs_with_two_resolvers_via_storage_set() {
    let mut library = Library::new();
    
    let mut storage1 = RustStorageCache::new();
    match storage1.set_vc_document("example_key1", "example_value1").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_vc_resolver(Box::from(storage1));

    let mut storage2 = RustStorageCache::new();
    match storage2.set_vc_document("example_key2", "example_value2").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    library.register_vc_resolver(Box::from(storage2));

    let fetched = library.get_vc_document("example_key1").await.unwrap();
    assert!(fetched == "example_value1");
    let fetched = library.get_vc_document("example_key2").await.unwrap();
    assert!(fetched == "example_value2");
}