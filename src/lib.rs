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

//! This library is intended to be used as a core library for developing own self-sovereign identity based applications.
//!
//! So why the name? "Vade" is an acronym for "VC and DID engine". It focuses on working with VCs and DIDs but does not hold much logic concerning their structure. Confused? Guessed as much, so what this library actually does is:
//!
//! - offering traits that define how actual implementations (aka "plugins") for working with VCs and DIDs should behave
//! - storing those plugins in a [`Vade`] instance
//! - querying against all registered plugins for certain actions (e.g. for getting VCs/DIDs)
//!
//! It has been designed with the idea of offering a consistent interface to work with while supporting to move the actual work into plugins, which also helps to reduce the dependencies.
//!
//! This library is currently under development. Behavior, as well as provided traits, will most probably change over time.
//!
//! ## Usage
//!
//! First add `vade` as a dependency to your `Cargo.toml`. Then you can create new instances in your code (taken from our tests):
//!
//! ```rust
//! extern crate vade;
//!
//! use vade::Vade;
//!
//! #[test]
//! fn library_can_be_created() {
//!     let _vade = Vade::new();
//! }
//! ```
//!
//! Okay, that did not do much yet. The core library also offers a simple in-memory cache, called [`RustStorageCache`], which can be used when working with VC and DID documents, that are available offline and should just be stored and/or retrieved locally. So to use [`RustStorageCache`] as a [`VcResolver`], we just need to add it as a [`VcResolver`] plugin:
//!
//! ```rust
//! extern crate vade;
//!
//! use vade::Vade;
//! use vade::traits::VcResolver;
//! use vade::plugin::rust_storage_cache::RustStorageCache;
//!
//!
//! #[tokio::test]
//! async fn library_vc_can_set_vcs_with_two_resolvers_via_library_set() {
//!     let mut vade = Vade::new();
//!     let storage = RustStorageCache::new();
//!     library.register_vc_resolver(Box::from(storage));
//!
//!     match library.set_vc_document("example_key", "example_value").await {
//!         Ok(()) => (),
//!         Err(e) => panic!(format!("{}", e)),
//!     }
//!     let fetched = library.get_vc_document("example_key").await.unwrap();
//!     assert!(fetched == "example_value");
//! }
//! ```
//!
//! Keep in mind, that the [`RustStorageCache`] resolver can be considered a reference implementation about how resolvers may behave and that it does not validate integrity and validity of given documents. For features like these, you can implement your own resolvers by following the respective traits.
//!
//! ## Plugins
//!
//! Plugins are the modules that perform the actual work in the [`Vade`] module. This project already has one plugin included,  [`RustStorageCache`], which can be used as a refrence for creating own plugins.
//!
//! ### Create new Plugins
//!
//! Developing plugins for `vade` can be done by implementing one or more traits from [`vade::library::traits`], e.g.
//!
//! - [`DidResolver`]
//! - [`VcResolver`]
//! - [`Logger`] (currently unclear if this plugin will be continued, but can be used for implementation tests)
//!
//! An example for a simple plugin is the provided [`RustStorageCache`]. It implements [`DidResolver`] as well as [`VcResolver`] functionalities. For your implementation you can, of course, decide to implement only a single trait in a plugin.
//!
//! ### Basic Behavior
//!
//! This plugin implements the following traits:
//!
//! - [`VcResolver`] - therefore, it can handle VC documents
//! - [`DidResolver`] - therefore, it can handle DID documents
//!
//! This allows us to register it as these resolvers with
//!
//! - [`register_vc_resolver`]
//! - [`register_did_resolver`]
//!
//! respectively.
//!
//! As soon as they are registered as a plugin for a certain type of operation, they are called for related operations (e.g. [`get_vc_document`]) by the [`Vade`] instance they are registered in.
//!
//! ### Library Functions that utilize Plugins
//!
//! This section shows a short overview over the plugin related functions. For more details, have a look at the [`Vade`] documentation.
//!
//! #### Plugin registration
//!
//! These functions can be used to register new resolver plugins:
//!
//! - [`register_did_resolver`]
//! - [`register_vc_resolver`]
//!
//! #### Setters
//!
//! These functions will call all registered plugins respectively and with given arguments (e.g. setting a DID will only call DID resolver functions, etc.):
//!
//! - [`set_did_document`]
//! - [`set_vc_document`]
//!
//! If multiple plugins are registered, awaits completion of all actions. First plugin that fails lets this request fail.
//!
//! #### Getters
//!
//! These functions will call all registered plugins respectively and with given arguments (e.g. getting a DID will only call DID resolver functions, etc.):
//!
//! - [`get_did_document`]
//! - [`get_vc_document`]
//!
//! If multiple plugins are registered, first **successful** response will be used. Request will fail if all plugins failed.
//!
//! #### Validation
//!
//! These functions will call all registered plugins respectively and with given arguments (e.g. getting a DID will only call DID resolver functions, etc.):
//!
//! - [`check_did_document`]
//! - [`check_vc_document`]
//!
//! A document is considered valid if at least one resolver confirms its validity. Resolvers may throw to indicate:
//!
//! - that they are not responsible for this document
//! - that they consider this document invalid
//!
//! The current validation flow offers only a limited way of feedback for invalidity and may undergo further changes in future.
//!
//! ### More Plugins
//!
//! A plugin working with VCs and DIDs on [evan.network] called [`vade-evan`] has been implemented. Its usage is equivalent to the description above, more details can be found on its project page.
//!
//! You can also start writing your own plugin, by following the behavior outlined with the traits in this library.
//!
//! [evan.network]: https://evan.network
//! [`check_did_document`]: traits/trait.DidResolver.html#tymethod.check_did_document
//! [`check_vc_document`]: traits/trait.VcResolver.html#tymethod.check_vc_document
//! [`DidResolver`]: traits/trait.DidResolver.html
//! [`get_did_document`]: traits/trait.DidResolver.html#tymethod.get_did_document
//! [`get_vc_document`]: traits/trait.VcResolver.html#tymethod.get_vc_document
//! [`Logger`]: traits/trait.Logger.html
//! [`register_did_resolver`]: struct.Vade.html#method.register_did_resolver
//! [`register_vc_resolver`]: struct.Vade.html#method.register_did_resolver
//! [`RustStorageCache`]: plugin/rust_storage_cache/struct.RustStorageCache.html
//! [`set_did_document`]: traits/trait.DidResolver.html#tymethod.set_did_document
//! [`set_vc_document`]: traits/trait.VcResolver.html#tymethod.set_vc_document
//! [`vade-evan`]: https://docs.rs/vade-evan
//! [`vade::library::traits`]: traits/index.html
//! [`Vade`]: struct.Vade.html
//! [`VcResolver`]: traits/trait.VcResolver.html

pub mod plugin;
pub mod traits;

#[macro_use]
extern crate simple_error;

use futures::future::{ select_ok, try_join_all };
use simple_error::SimpleError;
use traits::{ DidResolver, Logger, VcResolver };

/// Vade library, that holds plugins and delegates calls to them.
pub struct Vade {
    /// Vector of supported DID resolvers.
    pub did_resolvers: Vec<Box<dyn DidResolver>>,
    /// Vector of supported loggers. Logging will iterate through it and try to
    /// use every logger.
    pub loggers: Vec<Box<dyn Logger>>,
    /// Vector of supported VC resolvers.
    pub vc_resolvers: Vec<Box<dyn VcResolver>>,
}

impl Vade {
    /// Creates new Vade instance, vectors are initialized as empty.
    pub fn new() -> Vade {
        Vade {
            did_resolvers: Vec::new(),
            loggers: Vec::new(),
            vc_resolvers: Vec::new(),
        }
    }

    /// Checks given DID document against registered resolvers.
    /// A DID document is considered as valid if at least one did resolver
    /// confirms its validity.
    /// Resolvers may throw to indicate
    /// - that they are not responsible for this DID
    /// - that they consider this DID as invalid
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to check document for
    /// * `value` - value to check
    pub async fn check_did(&mut self, did_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let futures = self.did_resolvers.iter()
            .map(|resolver| resolver.check_did(did_name, value));
        match select_ok(futures).await {
            Ok(_) => Ok(()),
            Err(_e) => Err(Box::new(SimpleError::new(format!("did document not valid")))),
        }
    }

    /// Checks given VC document against registered resolvers.
    /// A VC document is considered as valid if at least one vc resolver
    /// confirms its validity.
    /// Resolvers may throw to indicate
    /// - that they are not responsible for this VC
    /// - that they consider this VC as invalid
    ///
    /// # Arguments
    ///
    /// * `vc_id` - vc_id to check document for
    /// * `value` - value to check
    pub async fn check_vc(&mut self, vc_id: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let futures = self.vc_resolvers.iter()
            .map(|resolver| resolver.check_vc(vc_id, value));
        match select_ok(futures).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(SimpleError::new(format!("vc document not valid; {}", e)))),
        }
    }

    /// Gets document for given did name.
    /// If multiple plugins are registered, first **successful** response
    /// will be used. Request will fail if all plugins failed.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to fetch
    pub async fn get_did_document(&self, did_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let futures = self.did_resolvers.iter()
            .map(|resolver| resolver.get_did_document(did_name));
        match select_ok(futures).await {
            Ok((r, _)) => Ok(r),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not get did document")))),
        }
    }

    /// Gets document for given vc name.
    /// If multiple plugins are registered, first **successful** response
    /// will be used. Request will fail if all plugins failed.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to fetch
    pub async fn get_vc_document(&self, vc_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let futures = self.vc_resolvers.iter()
            .map(|resolver| resolver.get_vc_document(vc_name));
        match select_ok(futures).await {
            Ok((r, _)) => Ok(r),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not get vc document")))),
        }
    }

    /// Log given message. Logging will iterate through it and try to use
    /// every logger.
    ///
    /// **Note**: This functionality is currently semi-obsoleted. It has been
    /// used for testing purposes and it is currently unclear wheter it will
    /// stay as it is or undergo further changes to allow to to fulfill its
    /// original role.
    ///
    /// # Arguments
    ///
    /// * `message` - message to log
    pub fn log(&self, message: &str, level: Option<&str>) {
        for logger in self.loggers.iter() {
            logger.log(message, level);
        }
    }

    /// Registers new [`DidResolver`] instance. Note, that `did_resolver` is given
    /// as `Box` to support dynamic avadegnment.
    ///
    /// # Arguments
    ///
    /// * `did_resolver` - an instance of a `struct` that implements
    ///                    [`DidResolver`] trait
    pub fn register_did_resolver(&mut self, did_resolver: Box<dyn DidResolver>) {
        self.did_resolvers.push(did_resolver);
    }

    /// Registers new [`Logger`] instance. Note, that `logger` is given as `Box`
    /// to support dynamic avadegnment.
    ///
    /// # Arguments
    ///
    /// * `logger` - an instance of a `struct` that implements [`Logger`] trait
    pub fn register_logger(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }

    /// Registers new `VcdResolver` instance. Note, that `vc_resolver` is given
    /// as `Box` to support dynamic avadegnment.
    ///
    /// # Arguments
    ///
    /// * `vc_resolver` - an instance of a `struct` that implements [`VcResolver`]
    ///                   trait
    pub fn register_vc_resolver(&mut self, vc_resolver: Box<dyn VcResolver>) {
        self.vc_resolvers.push(vc_resolver);
    }

    /// Sets document for given did name.
    /// If multiple plugins are registered, awaits completion of all actions.
    /// First plugin, that fails lets this request fail.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to set value for
    /// * `value` - value to set
    pub async fn set_did_document(&mut self, did_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let futures = self.did_resolvers.iter_mut()
            .map(|resolver| resolver.set_did_document(did_name, value));
        match try_join_all(futures).await {
            Ok(_) => Ok(()),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not set did document")))),
        }
    }

    /// Sets document for given vc name.
    /// If multiple plugins are registered, awaits completion of all actions.
    /// First plugin, that fails lets this request fail.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to set value for
    /// * `value` - value to set
    pub async fn set_vc_document(&mut self, vc_name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let futures = self.vc_resolvers.iter_mut()
            .map(|resolver| resolver.set_vc_document(vc_name, value));
        match try_join_all(futures).await {
            Ok(_) => Ok(()),
            Err(_e) => Err(Box::new(SimpleError::new(format!("could not set vc document")))),
        }
    }
}
