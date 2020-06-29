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

//! This library is a framework for working with VCs and DIDs of different platforms and types in a constant manner. Even if the actual implementation and logic behind them may change, vade offers a consistent interface to work with them.
//!
//! The name "Vade" is an acronym for "VC and DID engine" and focuses on working with VCs and DIDs. It has been designed with the idea of offering a consistent interface to work with while supporting to move the actual work into plugins.
//!
//! This library is currently under development. Behavior, as well as provided exports, will most probably change over time.
//!
//! Documentation about [`Vade`]s functions and their meaning can be found [`here`](https://docs.rs/vade/*/vade/struct.Vade.html)
//!
//! ## Example Usage
//!
//! ```
//! use vade::Vade;
//! // use some_crate:ExamplePlugin;
//! # use vade::VadePlugin;
//! # struct ExamplePlugin { }
//! # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
//! # impl VadePlugin for ExamplePlugin {}
//!
//! async fn example_vade_usage() {
//!     let ep: ExamplePlugin = ExamplePlugin::new();
//!     let mut vade = Vade::new();
//!     vade.register_plugin(Box::from(ep));
//!
//!     match vade.did_create("did:example", "", "").await {
//!         Ok(results) => {
//!             let result = results[0].as_ref().unwrap().to_string();
//!             println!("created did: {}", result);
//!         },
//!         Err(e) => panic!(format!("could not create did; {}", e)),
//!     };
//! }
//! ```
//!
//! As you can see, an instance of `ExamplePlugin` is created and handed over to a [`Vade`] instance with [`register_plugin`]. To be a valid argument for this, `ExamplePlugin` needs to implement [`VadePlugin`].
//!
//! [`Vade`] delegates the call *all* functions with the same name as the functions of [`VadePlugin`] to *all* registered plugins, so the result of such calls is a `Vec` of optional `String` values (`Vec<Option<String>>`).
//! 
//! ## Basic Plugin Flow
//!
//! Calls of plugin related functions follow the rule set described here:
//!
//! - a [`Vade`] instance delegates **all** calls of plugin related functions to **all** registered plugins
//! - those [`VadePlugin`] instances then may or may not process the request
//! - requests may be ignored due to not being implemented or due to ignoring them due to plugin internal logic (e.g. if a did method is not supported by the plugin, requests for this method are usually ignored)
//! - ignored plugin requests do not end up in the result `Vec`, so a [`Vade`] may have registered multiple plugins, but if only on plugin caters to a certain did method, calls related to this method will only yield a single result
//!
//! ![vade_plugin_flow](https://user-images.githubusercontent.com/1394421/85983296-8f3dd700-b9e7-11ea-92ee-47e8c441e576.png)
//!
//! ## Writing own Plugins
//!
//! Writing own plugin is rather simple, an example and details how to write them can be found in the [`VadePlugin`] documentation.
//!
//! ## Wasm Support
//!
//! Vade supports Wasm! ^^
//!
//! For an example how to use [`Vade`] in Wasm and a how to guide, have a look at our [vade-wasm-example] project.
//!
//! [`did_create`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html#method.did_create
//! [`register_plugin`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_plugin
//! [`vade-evan`]: https://docs.rs/vade-evan
//! [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
//! [`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html

extern crate env_logger;
#[macro_use]
extern crate log;

mod vade;
mod vade_plugin;

pub use self::vade::Vade;
pub use self::vade_plugin::{VadePlugin, VadePluginResultValue};
