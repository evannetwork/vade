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
//! ## Basic Flow
//!
//! When a [`Vade`] instance and its plugins have been set up (see next section), it will delegate calls to its plugins.
//!
//! For example when fetching a VC document with a [`Vade`] instance with two plugins (a [`RustStorageCache`] and a [`RustVcResolverEvan`]) the flow looks like this:
//!
//! ![vade_plugin_flow](https://user-images.githubusercontent.com/1394421/81380160-b0322c00-910a-11ea-8670-50455650497b.png)
//!
//! ## Usage
//!
//! ### Basic Usage
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
//! async fn example() {
//!     let mut vade = Vade::new();
//!     let storage = RustStorageCache::new();
//!     vade.register_vc_resolver(Box::from(storage));
//!
//!     match vade.set_vc_document("vc:example123", "{}").await {
//!         Ok(()) => (),
//!         Err(e) => panic!(format!("{}", e)),
//!     }
//!     let fetched = vade.get_vc_document("vc:example123").await.unwrap();
//!     assert!(fetched == "{}");
//! }
//! ```
//!
//! Keep in mind, that the [`RustStorageCache`] resolver can be considered a reference implementation about how resolvers may behave and that it does not validate integrity and validity of given documents. For features like these, you can implement your own resolvers by following the respective traits.
//!
//! ### Examples
//!
//! In the examples here initialization is omitted to keep the readability up. If you want to see the full flow code, you can have a look at the beginning of the [`vade library file`].
//!
//! #### VCs
//!
//! ##### Adding a VC
//!
//! This, of course requires you to have an existing VC, that you can actually set, but this depends on the actual plugins used in your instance. As we use the [`RustStorageCache`] plugin in our examples here, this requirement is not a problem, as it just acts as a key-value store with out any checks around it.
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::VcResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_vc_resolver(Box::from(storage));
//! # 
//! match vade.set_vc_document("vc:example123", "{}").await {
//!     Ok(()) => (),
//!     Err(e) => panic!(format!("{}", e)),
//! }
//! #     let fetched = vade.get_vc_document("vc:example123").await.unwrap();
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! ##### Getting a VC
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::VcResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_vc_resolver(Box::from(storage));
//! # 
//! #      match vade.set_vc_document("vc:example123", "{}").await {
//! #          Ok(()) => (),
//! #          Err(e) => panic!(format!("{}", e)),
//! #      }
//! let fetched = vade.get_vc_document("vc:example123").await.unwrap();
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! ##### Validating VCs
//!
//! Note that the outcome of this function heavily depends on the used plugin. [`RustStorageCache`] for example will only accept VCs as valid, if their name is "test".
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::VcResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_vc_resolver(Box::from(storage));
//! # 
//! #     match vade.set_vc_document("test", "{}").await {
//! #        Ok(()) => (),
//! #        Err(e) => panic!(format!("{}", e)),
//! #     }
//! #     let fetched = vade.get_vc_document("test").await.unwrap();
//! let vc_result = vade.check_vc("test", &fetched).await;
//! match vc_result {
//!     Ok(_) => (),
//!     Err(_) => panic!("VC not valid"),
//! }
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! #### DIDs
//!
//! ##### Adding a DID
//!
//! If your registered plugin allows you to set DIDs you can set it with:
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::DidResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_did_resolver(Box::from(storage));
//! # 
//! match vade.set_did_document("did:example123", "{}").await {
//!     Ok(()) => (),
//!     Err(e) => panic!(format!("{}", e)),
//! }
//! #     let fetched = vade.get_did_document("did:example123").await.unwrap();
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! ##### Getting a DID
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::DidResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_did_resolver(Box::from(storage));
//! # 
//! #     match vade.set_did_document("did:example123", "{}").await {
//! #         Ok(()) => (),
//! #         Err(e) => panic!(format!("{}", e)),
//! #     }
//! let fetched = vade.get_did_document("did:example123").await.unwrap();
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! ##### Validating DIDs
//!
//! Again, note that the outcome of this function heavily depends on the used plugin. [`RustStorageCache`] for example will only accept DIDs as valid, if their name is "test".
//!
//! ```rust
//! # extern crate vade;
//! # 
//! # use vade::Vade;
//! # use vade::traits::VcResolver;
//! # use vade::plugin::rust_storage_cache::RustStorageCache;
//! # 
//! # 
//! # #[tokio::test]
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let storage = RustStorageCache::new();
//! #     vade.register_did_resolver(Box::from(storage));
//! # 
//! #     match vade.set_did_document("test", "{}").await {
//! #        Ok(()) => (),
//! #        Err(e) => panic!(format!("{}", e)),
//! #     }
//! #     let fetched = vade.get_did_document("test").await.unwrap();
//! let did_result = vade.check_did("test", &fetched).await;
//! match did_result {
//!     Ok(_) => (),
//!     Err(_) => panic!("VC not valid"),
//! }
//! #     assert!(fetched == "{}");
//! # }
//! ```
//!
//! #### Generic Messages
//!
//! [`Vade`] also supports a more open type of message pipelining instead of message bound directly to VCs and DIDs. Plugins can be registered for generic messages with
//!
//! ```rust
//! extern crate vade;
//!
//! use async_trait::async_trait;
//! use vade::Vade;
//! use vade::traits::MessageConsumer;
//!
//! pub struct TestMessageConsumer {
//!     message_count: u64,
//! }
//!
//! impl TestMessageConsumer {
//!     pub fn new() -> TestMessageConsumer {
//!         TestMessageConsumer {
//!             message_count: 0,
//!         }
//!     }
//! }
//!
//! #[async_trait(?Send)]
//! impl MessageConsumer for TestMessageConsumer {
//!     async fn handle_message(
//!         &mut self,
//!         _message_type: &str,
//!         message_data: &str,
//!         ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//!         self.message_count = self.message_count + 1;
//!         Ok(Option::from(format!(r###"{{ "type": "response", "data": {{ "count": {}, "lastMessage": {} }} }}"###, self.message_count, &message_data).to_string()))
//!     }
//! }
//!
//! async fn example() {
//!     let mut vade = Vade::new();
//!     let tmc = TestMessageConsumer::new();
//!     vade.register_message_consumer(
//!         &vec!["message1", "message2"].iter().map(|&x| String::from(x)).collect(),
//!         Box::from(tmc),
//!     );
//! }
//! ```
//!
//! After this registered plugins will be called when related messages arrive and all registered plugins have to provide a response as `Option<String>`. All responses are collected and returned to caller, e.g.:
//!
//! ```rust
//! # extern crate vade;
//! #
//! # use async_trait::async_trait;
//! # use vade::Vade;
//! # use vade::traits::MessageConsumer;
//! #
//! # pub struct TestMessageConsumer {
//! #     message_count: u64,
//! # }
//! #
//! # impl TestMessageConsumer {
//! #     pub fn new() -> TestMessageConsumer {
//! #         TestMessageConsumer {
//! #             message_count: 0,
//! #         }
//! #     }
//! # }
//! #
//! # #[async_trait(?Send)]
//! # impl MessageConsumer for TestMessageConsumer {
//! #     async fn handle_message(
//! #         &mut self,
//! #         _message_type: &str,
//! #         message_data: &str,
//! #         ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//! #         self.message_count = self.message_count + 1;
//! #         Ok(Option::from(format!(r###"{{ "type": "response", "data": {{ "count": {}, "lastMessage": {} }} }}"###, self.message_count, &message_data).to_string()))
//! #     }
//! # }
//! #
//! # async fn example() {
//! #     let mut vade = Vade::new();
//! #     let tmc = TestMessageConsumer::new();
//! #     vade.register_message_consumer(
//! #         &vec!["message1", "message2"].iter().map(|&x| String::from(x)).collect(),
//! #         Box::from(tmc),
//! #     );
//! #
//! let responses = vade.send_message(r###"{ "type": "message1", "data": {} }"###).await.unwrap();
//! # }
//! ```
//!
//! Note, that input is provided as a `String` that can be parsed to [`VadeMessage`], so the properties `type` and `data` are mandatory, while `type` controls the plugins, that will receive the values of `data`. `data` is usually formatted as JSON, but can have any format, that is convenient and can be included in JSON documents.
//!
//! This rather open approach opens up a lot of possibilities for implementing own plugins and workflows.
//!
//! ## Plugins
//!
//! Plugins are the modules that perform the actual work in the [`Vade`] module. This project already has one plugin included,  [`RustStorageCache`], which can be used as a reference for creating own plugins.
//!
//! ### Create new Plugins
//!
//! Developing plugins for `vade` can be done by implementing one or more traits from [`vade::library::traits`], e.g.
//!
//! - [`VcResolver`]
//! - [`DidResolver`]
//! - [`MessageConsumer`]
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
//! - [`register_message_consumer`]
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
//! #### Generic Messages]
//!
//! Messaging is rather straight forward. After registering a [`MessageConsumer`] with [`register_message_consumer`] it will receive messages for all registered `type`s of messages. A [`MessageConsumer`] only hast to implement [`send_message`] to be able to consume and respond to messages from [`Vade`].
//! 
//! ### More Plugins
//!
//! A plugin working with VCs and DIDs on [evan.network] called [`vade-evan`] has been implemented. Its usage is equivalent to the description above, more details can be found on its project page.
//!
//! You can also start writing your own plugin, by following the behavior outlined with the traits in this library.
//!
//! ## Wasm Support
//!
//! Vade supports Wasm! ^^
//!
//! For an example how to use [`Vade`] in Wasm and a how to guide, have a look at our [vade-wasm-example] project.
//!
//! [`check_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.check_did_document
//! [`check_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.check_vc_document
//! [`DidResolver`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html
//! [`get_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.get_did_document
//! [`get_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.get_vc_document
//! [`Logger`]: https://docs.rs/vade/*/vade/traits/trait.Logger.html
//! [`MessageConsumer`]: https://docs.rs/vade/*/vade/traits/trait.MessageConsumer.html
//! [`register_did_resolver`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_did_resolver
//! [`register_message_consumer`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_message_consumer
//! [`register_vc_resolver`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_did_resolver
//! [`RustStorageCache`]: https://docs.rs/vade/*/vade/plugin/rust_storage_cache/struct.RustStorageCache.html
//! [`RustVcResolverEvan`]: https://docs.rs/vade-evan/*/vade_evan/plugin/rust_vcresolver_evan/struct.RustVcResolverEvan.html
//! [`send_message`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.send_message
//! [`set_did_document`]: https://docs.rs/vade/*/vade/traits/trait.DidResolver.html#tymethod.set_did_document
//! [`set_vc_document`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html#tymethod.set_vc_document
//! [`vade library file`]: https://github.com/evannetwork/vade/blob/develop/src/lib.rs
//! [`vade-evan`]: https://docs.rs/vade-evan
//! [`vade::library::traits`]: https://docs.rs/vade/*/vade/traits/index.html
//! [`Vade`]: https://docs.rs/vade//vade/struct.Vade.html
//! [`VadeMessage`]: https://docs.rs/vade/*/vade/struct.VadeMessage.html
//! [`VcResolver`]: https://docs.rs/vade/*/vade/traits/trait.VcResolver.html
//! [evan.network]: https://evan.network
//! [vade-wasm-example]: https://github.com/evannetwork/vade-wasm-example

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

pub mod plugin;
pub mod traits;

use futures::future::{select_ok, try_join_all};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use simple_error::SimpleError;
use traits::{DidResolver, Logger, MessageConsumer, VcResolver, VadePlugin, VadePluginResultValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct VadeMessage<'a> {
    r#type: String,
    #[serde(borrow)]
    data: &'a RawValue,
}

/// Vade library, that holds plugins and delegates calls to them.
pub struct Vade {
    /// Vector of supported DID resolvers.
    pub did_resolvers: Vec<Box<dyn DidResolver>>,
    /// Vector of supported loggers. Logging will iterate through it and try to
    /// use every logger.
    pub loggers: Vec<Box<dyn Logger>>,
    /// Vector of supported VC resolvers.
    pub vc_resolvers: Vec<Box<dyn VcResolver>>,
    /// plugins, that subscribed for generic messages
    pub message_consumers: Vec<Box<dyn MessageConsumer>>,
    /// subscribed message types for each consumer
    pub message_subscriptions: Vec<Vec<String>>,
    /// registered plugins
    pub plugins: Vec<Box<dyn VadePlugin>>,
    /// registered plugins' functions
    pub plugins_functions: Vec<Option<Vec<String>>>,
}

impl Vade {
    /// Creates new Vade instance, vectors are initialized as empty.
    pub fn new() -> Vade {
        match env_logger::try_init() {
            Ok(_) | Err(_) => (),
        };
        Vade {
            did_resolvers: Vec::new(),
            loggers: Vec::new(),
            vc_resolvers: Vec::new(),
            message_consumers: Vec::new(),
            message_subscriptions: Vec::new(),
            plugins: Vec::new(),
            plugins_functions: Vec::new(),
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
    /// as `Box` to support dynamic assignment.
    ///
    /// # Arguments
    ///
    /// * `did_resolver` - an instance of a `struct` that implements
    ///                    [`DidResolver`] trait
    pub fn register_did_resolver(&mut self, did_resolver: Box<dyn DidResolver>) {
        self.did_resolvers.push(did_resolver);
    }

    /// Registers new [`Logger`] instance. Note, that `logger` is given as `Box`
    /// to support dynamic assignment.
    ///
    /// # Arguments
    ///
    /// * `logger` - an instance of a `struct` that implements [`Logger`] trait
    pub fn register_logger(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }

    /// Registers new `VcdResolver` instance. Note, that `vc_resolver` is given
    /// as `Box` to support dynamic assignment.
    ///
    /// # Arguments
    ///
    /// * `vc_resolver` - an instance of a `struct` that implements [`VcResolver`]
    ///                   trait
    pub fn register_vc_resolver(&mut self, vc_resolver: Box<dyn VcResolver>) {
        self.vc_resolvers.push(vc_resolver);
    }

    /// Register a message consumer for handling messages.
    /// 
    /// # Arguments
    /// 
    /// * `types` - types of messages to subsribe for
    /// * `consumer` - `MessageConsumer` that subscribes for given `types`
    pub fn register_message_consumer(&mut self, types: &Vec<String>, consumer: Box<dyn MessageConsumer>) {
        self.message_consumers.push(consumer);
        let mut subscriptions = Vec::new();
        for subscription in types.iter() {
            subscriptions.push(subscription.to_string());
        }
        self.message_subscriptions.push(subscriptions);
    }

    /// Send message to all subsribed consumers, only consumers, that subscribed to messages of type
    /// `message_type` (key in `message`).
    /// 
    /// # Arguments
    ///
    /// * `message` - message of format that can be parsed to `VadeMessage`
    pub async fn send_message<'a>(
        &mut self,
        message: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        debug!("got message: {:?}", message);
        let parsed: VadeMessage = serde_json::from_str(message).unwrap();
        let mut futures = Vec::new();
        for (i, consumer) in self.message_consumers.iter_mut().enumerate() {
            let subscriptions = &self.message_subscriptions[i];
            if subscriptions.iter().any(|i| i == &parsed.r#type) {
                futures.push(consumer.handle_message(&parsed.r#type, parsed.data.get()))
            }
        }
        match try_join_all(futures).await {
            Ok(responses) => Ok(responses),
            Err(e) => return Err(Box::new(SimpleError::new(format!("could not set handle message \"{}\"; {}", &parsed.r#type, e)))),
        }
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

    pub fn register_plugin(&mut self, plugin: Box<dyn VadePlugin>) {
        self.plugins.push(plugin);
    }

    pub async fn did_create(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.did_create(did_method, options, payload));
        }
        // TODO find a better solution than copy & paste >.>
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create did for method \"{}\"; {}", &did_method, e)))
        }
    }

    pub async fn did_resolve(
        &mut self,
        did: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.did_resolve(did));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not resolve did \"{}\"; {}", &did, e)))
        }
    }

    pub async fn did_update(
        &mut self,
        did: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.did_update(did, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not update did \"{}\"; {}", &did, e)))
        }
    }

    pub async fn vc_zkp_create_credential_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_create_credential_definition(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create credential definition for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_create_credential_offer(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_create_credential_offer(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create credential offer for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_create_credential_proposal(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_create_credential_proposal(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create credential proposal for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_create_credential_schema(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_create_credential_schema(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create credential schema for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_create_revocation_registry_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_create_revocation_registry_definition(
                method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not create revocation registry definition for method \"{}\"; {}",
                &method, e))
            ),
        }
    }

    pub async fn vc_zkp_update_revocation_registry(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_update_revocation_registry(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not update revocation registry for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_issue_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_issue_credential(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not issue credential for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_present_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_present_proof(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not present proof for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_request_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_request_credential(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not request credential for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_request_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_request_proof(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not request proof for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_revoke_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_revoke_credential(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not revoke credential for method \"{}\"; {}", &method, e)))
        }
    }

    pub async fn vc_zkp_verify_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(plugin.vc_zkp_verify_proof(method, options, payload));
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                Ok(filtered_results)
            },
            Err(e) => Err(Box::from(format!(
                "could not verify proof for method \"{}\"; {}", &method, e)))
        }
    }
}
