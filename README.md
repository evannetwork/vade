# Vade

[![crates.io](https://img.shields.io/crates/v/vade.svg)](https://crates.io/crates/vade)
[![Documentation](https://docs.rs/vade/badge.svg)](https://docs.rs/vade)
[![Apache-2 licensed](https://img.shields.io/crates/l/vade.svg)](./LICENSE.txt)

## About

This library is intended to be used as a core library for developing own self-sovereign identity based applications.

So why the name? "Vade" is an acronym for "VC and DID engine". It focuses on working with VCs and DIDs but does not hold much logic concerning their structure. Confused? Guessed as much, so what this library actually does is:

- offering traits that define how actual implementations (aka "plugins") for working with VCs and DIDs should behave
- storing those plugins in a [`Vade`] instance
- querying against all registered plugins for certain actions (e.g. for getting VCs/DIDs)

It has been designed with the idea of offering a consistent interface to work with while supporting to move the actual work into plugins, which also helps to reduce the dependencies.

This library is currently under development. Behavior, as well as provided traits, will most probably change over time.

## Basic Flow

When a [`Vade`] instance and its plugins have been set up (see next section), it will delegate calls to its plugins.

For example when fetching a VC document with a [`Vade`] instance with two plugins (a [`RustStorageCache`] and a [`RustVcResolverEvan`]) the flow looks like this:

![vade_plugin_flow](https://user-images.githubusercontent.com/1394421/81380160-b0322c00-910a-11ea-8670-50455650497b.png)

## Usage

### Basic Usage

First add `vade` as a dependency to your `Cargo.toml`. Then you can create new instances in your code (taken from our tests):

```rust
extern crate vade;

use vade::Vade;

#[test]
fn library_can_be_created() {
    let _vade = Vade::new();
}
```

Okay, that did not do much yet. The core library also offers a simple in-memory cache, called [`RustStorageCache`], which can be used when working with VC and DID documents, that are available offline and should just be stored and/or retrieved locally. So to use [`RustStorageCache`] as a [`VcResolver`], we just need to add it as a [`VcResolver`] plugin:

```rust
extern crate vade;

use vade::Vade;
use vade::traits::VcResolver;
use vade::plugin::rust_storage_cache::RustStorageCache;


#[tokio::test]
async fn example() {
    let mut vade = Vade::new();
    let storage = RustStorageCache::new();
    vade.register_vc_resolver(Box::from(storage));

    match vade.set_vc_document("vc:example123", "{}").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = vade.get_vc_document("vc:example123").await.unwrap();
    assert!(fetched == "{}");
}
```

Keep in mind, that the [`RustStorageCache`] resolver can be considered a reference implementation about how resolvers may behave and that it does not validate integrity and validity of given documents. For features like these, you can implement your own resolvers by following the respective traits.

### Examples

In the examples here initialization is omitted to keep the readability up. If you want to see the full flow code, you can have a look at the beginning of the [`vade library file`].

#### VCs

##### Adding a VC

This, of course requires you to have an existing VC, that you can actually set, but this depends on the actual plugins used in your instance. As we use the [`RustStorageCache`] plugin in our examples here, this requirement is not a problem, as it just acts as a key-value store with out any checks around it.

```rust
match vade.set_vc_document("vc:example123", "{}").await {
    Ok(()) => (),
    Err(e) => panic!(format!("{}", e)),
}
```

##### Getting a VC

```rust
      let fetched = vade.get_vc_document("vc:example123").await.unwrap();
```

##### Validating VCs

Note that the outcome of this function heavily depends on the used plugin. [`RustStorageCache`] for example will only accept VCs as valid, if their name is "test".

```rust
let vc_result = vade.check_vc("test", &fetched).await;
match vc_result {
    Ok(_) => (),
    Err(_) => panic!("VC not valid"),
}
```


#### DIDs

##### Adding a DID

If your registered plugin allows you to set DIDs you can set it with:

```rust
match vade.set_did_document("did:example123", "{}").await {
    Ok(()) => (),
    Err(e) => panic!(format!("{}", e)),
}
```

##### Getting a DID

```rust
     let fetched = vade.get_did_document("did:example123").await.unwrap();
```

##### Validating DIDs

Again, note that the outcome of this function heavily depends on the used plugin. [`RustStorageCache`] for example will only accept DIDs as valid, if their name is "test".

```rust
let did_result = vade.check_did("test", &fetched).await;
match did_result {
    Ok(_) => (),
    Err(_) => panic!("VC not valid"),
}
```


## Plugins

Plugins are the modules that perform the actual work in the [`Vade`] module. This project already has one plugin included,  [`RustStorageCache`], which can be used as a reference for creating own plugins.

### Create new Plugins

Developing plugins for `vade` can be done by implementing one or more traits from [`vade::library::traits`], e.g.

- [`VcResolver`]
- [`DidResolver`]
- [`Logger`] (currently unclear if this plugin will be continued, but can be used for implementation tests)

An example for a simple plugin is the provided [`RustStorageCache`]. It implements [`DidResolver`] as well as [`VcResolver`] functionalities. For your implementation you can, of course, decide to implement only a single trait in a plugin.

### Basic Behavior

This plugin implements the following traits:

- [`VcResolver`] - therefore, it can handle VC documents
- [`DidResolver`] - therefore, it can handle DID documents

This allows us to register it as these resolvers with

- [`register_vc_resolver`]
- [`register_did_resolver`]

respectively.

As soon as they are registered as a plugin for a certain type of operation, they are called for related operations (e.g. [`get_vc_document`]) by the [`Vade`] instance they are registered in.

### Library Functions that utilize Plugins

This section shows a short overview over the plugin related functions. For more details, have a look at the [`Vade`] documentation.

#### Plugin registration

These functions can be used to register new resolver plugins:

- [`register_did_resolver`]
- [`register_vc_resolver`]

#### Setters

These functions will call all registered plugins respectively and with given arguments (e.g. setting a DID will only call DID resolver functions, etc.):

- [`set_did_document`]
- [`set_vc_document`]

If multiple plugins are registered, awaits completion of all actions. First plugin that fails lets this request fail.

#### Getters

These functions will call all registered plugins respectively and with given arguments (e.g. getting a DID will only call DID resolver functions, etc.):

- [`get_did_document`]
- [`get_vc_document`]

If multiple plugins are registered, first **successful** response will be used. Request will fail if all plugins failed.

#### Validation

These functions will call all registered plugins respectively and with given arguments (e.g. getting a DID will only call DID resolver functions, etc.):

- [`check_did_document`]
- [`check_vc_document`]

A document is considered valid if at least one resolver confirms its validity. Resolvers may throw to indicate:

- that they are not responsible for this document
- that they consider this document invalid

The current validation flow offers only a limited way of feedback for invalidity and may undergo further changes in future.

### More Plugins

A plugin working with VCs and DIDs on [evan.network] called [`vade-evan`] has been implemented. Its usage is equivalent to the description above, more details can be found on its project page.

You can also start writing your own plugin, by following the behavior outlined with the traits in this library.

## Wasm Support

Vade supports Wasm! ^^

For an example how to use [`Vade`] in Wasm and a how to guide, have a look at our [vade-wasm-example] project.

[evan.network]: https://evan.network
[vade-wasm-example]: https://github.com/evannetwork/vade-wasm-example
[`check_did_document`]: traits/trait.DidResolver.html#tymethod.check_did_document
[`check_vc_document`]: traits/trait.VcResolver.html#tymethod.check_vc_document
[`DidResolver`]: traits/trait.DidResolver.html
[`get_did_document`]: traits/trait.DidResolver.html#tymethod.get_did_document
[`get_vc_document`]: traits/trait.VcResolver.html#tymethod.get_vc_document
[`Logger`]: traits/trait.Logger.html
[`register_did_resolver`]: struct.Vade.html#method.register_did_resolver
[`register_vc_resolver`]: struct.Vade.html#method.register_did_resolver
[`RustStorageCache`]: plugin/rust_storage_cache/struct.RustStorageCache.html
[`RustVcResolverEvan`]: https://docs.rs/vade-evan/*/vade_evan/plugin/rust_vcresolver_evan/struct.RustVcResolverEvan.html
[`set_did_document`]: traits/trait.DidResolver.html#tymethod.set_did_document
[`set_vc_document`]: traits/trait.VcResolver.html#tymethod.set_vc_document
[`vade-evan`]: https://docs.rs/vade-evan
[`vade::library::traits`]: traits/index.html
[`vade library file`]: https://github.com/evannetwork/vade/blob/develop/src/lib.rs
[`Vade`]: struct.Vade.html
[`VcResolver`]: traits/trait.VcResolver.html
