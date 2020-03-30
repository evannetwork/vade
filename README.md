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

## Usage

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
async fn library_vc_can_set_vcs_with_two_resolvers_via_library_set() {
    let mut vade = Vade::new();
    let storage = RustStorageCache::new();
    library.register_vc_resolver(Box::from(storage));

    match library.set_vc_document("example_key", "example_value").await {
        Ok(()) => (),
        Err(e) => panic!(format!("{}", e)),
    }
    let fetched = library.get_vc_document("example_key").await.unwrap();
    assert!(fetched == "example_value");
}
```

Keep in mind, that the [`RustStorageCache`] resolver can be considered a reference implementation about how resolvers may behave and that it does not validate integrity and validity of given documents. For features like these, you can implement your own resolvers by following the respective traits.

## Plugins

Plugins are the modules that perform the actual work in the [`Vade`] module. This project already has one plugin included,  [`RustStorageCache`], which can be used as a refrence for creating own plugins.

### Create new Plugins

Developing plugins for `vade` can be done by implementing one or more traits from [`vade::library::traits`], e.g.

- [`DidResolver`]
- [`VcResolver`]
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

A plugin working with VCs and DIDs on [evan.network](https://evan.network/) called [`vade-evan`] has been implemented. Its usage is equivalent to the description above, more details can be found on its project page.

You can also start writing your own plugin, by following the behavior outlined with the traits in this library.

[`check_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.check_did_document
[`check_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.check_vc_document
[`DidResolver`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html
[`get_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.get_did_document
[`get_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.get_vc_document
[`Logger`]: https://docs.rs/vade/*/vade/traits/trait.Logger.html
[`register_did_resolver`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_did_resolver
[`register_vc_resolver`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_did_resolver
[`RustStorageCache`]: https://docs.rs/vade/*/vade/plugin/rust_storage_cache/struct.RustStorageCache.html
[`set_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.set_did_document
[`set_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.set_vc_document
[`vade-evan`]: https://docs.rs/vade_evan
[`vade::library::traits`]: https://docs.rs/vade/*/vade/traits/index.html
[`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
[`VcResolver`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html
