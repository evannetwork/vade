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

//! [`Vade`] is a framework for working with VCs and DIDs from different providers and on different platforms in a constant manner.
//! Even if the actual implementation and logic behind them may change, [`Vade`] offers a consistent interface to work with them.
//! It has been developed with [wasm support] in mind to allow it not only to run on servers but also on different clients
//! with limited resources like IoT devices.
//!
//! The name "Vade" is an acronym for "VC and DID engine" and focuses on working with VCs and DIDs. It has been designed with the idea of offering a consistent interface to work with while supporting to move the actual work into plugins.
//!
//! This library is currently under development. Behavior, as well as provided exports, may change over time.
//!
//! Documentation about [`Vade`]s functions and their meaning can be found [`here`](https://docs.rs/vade/*/vade/struct.Vade.html).
//!
//! ## Plugins
//!
//! [`Vade`] is relying on plugins to run interact provider specific logic. The current set of Plugins can be seen below:
//!
//! ### DID plugins
//!
//! | Method   | Status | Link |
//! | -------- | ------ | ----- |
//! | did:evan | 0.0.1  | [vade-evan](https://crates.io/crates/vade-evan) |
//! | did:example | 0.0.1  | [vade-example-plugin](https://github.com/evannetwork/vade-example-plugin) |
//! | (universal resolver method list) | in development  | - |
//!
//! More coming soon. To write your own plugins, have a look at [writing own plugins].
//!
//! ### VC plugins
//!
//! | Method   | Status | Link |
//! | -------- | ------ | ----- |
//! | did:evan | 0.0.1  | [vade-evan](https://crates.io/crates/vade-evan) |
//!
//! More coming soon. To write your own plugins, have a look at [writing own plugins].
//!
//! ## Example Usage
//!
//! ```rust
//! use vade::Vade;
//! // use some_crate:ExamplePlugin;
//! # use vade::VadePlugin;
//! # struct ExamplePlugin { }
//! # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
//! # impl VadePlugin for ExamplePlugin {}
//!
//! async fn example_vade_usage() -> Result<(), Box<dyn std::error::Error>> {
//!     let ep: ExamplePlugin = ExamplePlugin::new();
//!     let mut vade = Vade::new();
//!     vade.register_plugin(Box::from(ep));
//!
//!     match vade.did_create("did:example", "", "").await {
//!         Ok(results) => {
//!             let result = results[0].as_ref().ok_or("result not found")?.to_string();
//!             println!("created did: {}", result);
//!         },
//!         Err(e) => panic!(format!("could not create did; {}", e)),
//!     };
//!     Ok(())
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
//! ## Vade Features
//!
//! The current set of features can be grouped into 3 clusters:
//!
//! - management functions
//! - DID interaction
//! - zero knowledge proof VC interaction
//!
//! ### Management Functions
//!
//! **[`register_plugin`]**
//!
//! Registers a new plugin. See [`VadePlugin`](https://docs.rs/vade/*/vade/struct.VadePlugin.html) for details about how they work.
//!
//! ### DID Interaction
//!
//! **[`did_create`]**
//!
//! Creates a new DID. May also persist a DID document for it, depending on plugin implementation.
//!
//! -----
//!
//! **[`did_resolve`]**
//!
//! Fetch data about a DID. This usually returns a DID document.
//!
//! -----
//!
//! **[`did_update`]**
//!
//! Updates data related to a DID. May also persist a DID document for it, depending on plugin implementation.
//!
//! ### Zero Knowledge Proof VC Interaction
//!
//! **[`vc_zkp_create_credential_schema`]**
//!
//! Creates a new zero-knowledge proof credential schema. The schema specifies properties a credential
//! includes, both optional and mandatory.
//!
//! -----
//!
//! **[`vc_zkp_create_credential_definition`]**
//!
//! Creates a new zero-knowledge proof credential definition. A credential definition holds cryptographic key material
//! and is needed by an issuer to issue a credential, thus needs to be created before issuance. A credential definition
//! is always bound to one credential schema.
//!
//! -----
//!
//! **[`vc_zkp_create_credential_proposal`]**
//!
//! Creates a new zero-knowledge proof credential proposal. This message is the first in the
//! credential issuance flow.
//!
//! -----
//!
//! **[`vc_zkp_create_credential_offer`]**
//!
//! Creates a new zero-knowledge proof credential offer. This message is the response to a credential proposal.
//!
//! -----
//!
//! **[`vc_zkp_request_credential`]**
//!
//! Requests a credential. This message is the response to a credential offering.
//!
//! -----
//!
//! **[`vc_zkp_create_revocation_registry_definition`]**
//!
//! Creates a new revocation registry definition. The definition consists of a public and a private part.
//! The public part holds the cryptographic material needed to create non-revocation proofs. The private part
//! needs to reside with the registry owner and is used to revoke credentials.
//!
//! -----
//!
//! **[`vc_zkp_update_revocation_registry`]**
//!
//! Updates a revocation registry for a zero-knowledge proof. This step is necessary after revocation one or
//! more credentials.
//!
//! -----
//!
//! **[`vc_zkp_issue_credential`]**
//!
//! Issues a new credential. This requires an issued schema, credential definition, an active revocation
//! registry and a credential request message.
//!
//! -----
//!
//! **[`vc_zkp_revoke_credential`]**
//!
//! Revokes a credential. After revocation the published revocation registry needs to be updated with information
//! returned by this function.
//!
//! -----
//!
//! **[`vc_zkp_request_proof`]**
//!
//! Requests a zero-knowledge proof for one or more credentials issued under one or more specific schemas.
//!
//! -----
//!
//! **[`vc_zkp_present_proof`]**
//!
//! Presents a proof for a zero-knowledge proof credential. A proof presentation is the response to a
//! proof request.
//!
//! -----
//!
//! **[`vc_zkp_verify_proof`]**
//!
//! Verifies a one or multiple proofs sent in a proof presentation.
//!
//! ### Custom Functions
//!
//! **[`run_custom_function`]**
//!
//! Calls a custom function. Plugins may subscribe to such custom calls, that are not part of the default set of [`Vade`]s default feature set, which allows to add custom plugin logic while using `Vade. Examples for this may be connection handling and key generation.
//!
//! -----
//!
//! Except for the management functions all functions will be delegated to plugins. Plugins handling follows the following rules:
//!
//! - a [`Vade`] instance delegates **all** calls of plugin related functions to **all** registered plugins
//! - those [`VadePlugin`] instances then may or may not process the request
//! - requests may be ignored due to not being implemented or due to ignoring them due to plugin internal logic (e.g. if a did method is not supported by the plugin, requests for this method are usually ignored)
//! - ignored plugin requests do not end up in the result `Vec`, so a [`Vade`] may have registered multiple plugins, but if only on plugin caters to a certain did method, calls related to this method will only yield a single result
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
//! [`did_create`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_create
//! [`did_resolve`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_resolve
//! [`did_update`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_update
//! [`register_plugin`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_plugin
//! [`run_custom_function`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.run_custom_function
//! [`vade-evan`]: https://docs.rs/vade-evan
//! [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
//! [`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html
//! [`vc_zkp_create_credential_definition`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_create_credential_definition
//! [`vc_zkp_create_credential_offer`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_create_credential_offer
//! [`vc_zkp_create_credential_proposal`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_create_credential_proposal
//! [`vc_zkp_create_credential_schema`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_create_credential_schema
//! [`vc_zkp_create_revocation_registry_definition`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_create_revocation_registry_definition
//! [`vc_zkp_issue_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_issue_credential
//! [`vc_zkp_present_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_present_proof
//! [`vc_zkp_request_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_request_credential
//! [`vc_zkp_request_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_request_proof
//! [`vc_zkp_revoke_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_revoke_credential
//! [`vc_zkp_update_revocation_registry`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_update_revocation_registry
//! [`vc_zkp_verify_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.vc_zkp_verify_proof
//! [vade-wasm-example]: https://github.com/evannetwork/vade-wasm-example
//! <!-- for lib.rs -->
//! [wasm support]: https://docs.rs/vade/*/vade/#wasm-support
//! [writing own plugins]: https://docs.rs/vade/*/vade/#writing-own-plugins
//! <!-- -->
//! <!-- for Readme
//! [wasm support]: #wasm-support
//! [writing own plugins]: #writing-own-plugins
//! -->
extern crate env_logger;
#[macro_use]
extern crate log;

mod vade;
mod vade_plugin;

pub use self::vade::Vade;
pub use self::vade_plugin::{VadePlugin, VadePluginResultValue};
