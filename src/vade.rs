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

use crate::{VadePlugin, VadePluginResultValue};
use futures::future::try_join_all;

/// A [`Vade`] instance is your single point of contact for interacting with DIDs and VCs.
///
/// The current set of functions can be grouped into 3 clusters:
///
/// - management functions:
///     - [`register_plugin`]
/// - did interaction:
///     - [`did_create`]
///     - [`did_resolve`]
///     - [`did_update`]
/// - zero knowledge proof vc interaction:
///     - [`vc_zkp_create_credential_schema`]
///     - [`vc_zkp_create_credential_definition`]
///     - [`vc_zkp_create_credential_proposal`]
///     - [`vc_zkp_create_credential_offer`]
///     - [`vc_zkp_request_credential`]
///     - [`vc_zkp_create_revocation_registry_definition`]
///     - [`vc_zkp_update_revocation_registry`]
///     - [`vc_zkp_issue_credential`]
///     - [`vc_zkp_revoke_credential`]
///     - [`vc_zkp_request_proof`]
///     - [`vc_zkp_present_proof`]
///     - [`vc_zkp_verify_proof`]
/// 
/// Except for the management functions all functions will be delegated to plugins. Plugins handling follows the following rules:
///
/// - a [`Vade`] instance delegates **all** calls of plugin related functions to **all** registered plugins
/// - those [`VadePlugin`] instances then may or may not process the request
/// - requests may be ignored due to not being implemented or due to ignoring them due to plugin internal logic (e.g. if a did method is not supported by the plugin, requests for this method are usually ignored)
/// - ignored plugin requests do not end up in the result `Vec`, so a [`Vade`] may have registered multiple plugins, but if only on plugin caters to a certain did method, calls related to this method will only yield a single result
///
/// [`did_create`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_create
/// [`did_resolve`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_resolve
/// [`did_update`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.did_update
/// [`register_plugin`]: https://docs.rs/vade/*/vade/struct.Vade.html#method.register_plugin
/// [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
/// [`VadePlugin`]: https://docs.rs/vade/*/vade/struct.VadePlugin.html
/// [`vc_zkp_create_credential_definition`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_create_credential_definition
/// [`vc_zkp_create_credential_offer`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_create_credential_offer
/// [`vc_zkp_create_credential_proposal`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_create_credential_proposal
/// [`vc_zkp_create_credential_schema`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_create_credential_schema
/// [`vc_zkp_create_revocation_registry_definition`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_create_revocation_registry_definition
/// [`vc_zkp_issue_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_issue_credential
/// [`vc_zkp_present_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_present_proof
/// [`vc_zkp_request_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_request_credential
/// [`vc_zkp_request_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_request_proof
/// [`vc_zkp_revoke_credential`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_revoke_credential
/// [`vc_zkp_update_revocation_registry`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_update_revocation_registry
/// [`vc_zkp_verify_proof`]: https://docs.rs/vade/*/vade/struct.Vade.html#vc_zkp_verify_proof
pub struct Vade {
    /// registered plugins
    pub plugins: Vec<Box<dyn VadePlugin>>,
}

impl Vade {
    /// Creates new Vade instance, vectors are initialized as empty.
    pub fn new() -> Self {
        match env_logger::try_init() {
            Ok(_) | Err(_) => (),
        };
        Vade {
            plugins: Vec::new(),
        }
    }

    /// Creates a new DID. May also persist a DID document for it, depending on plugin implementation.
    ///
    /// # Arguments
    ///
    /// * `did_method` - did method to cater to, usually also used by plugins to decide if a plugins will process the request
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    /// 
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_create("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created new did: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn did_create(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("did_create");
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
                self.log_fun_leave("did_create", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create did for method \"{}\"; {}",
                &did_method, e
            ))),
        }
    }

    /// Fetch data about a DID from. This usually returns a DID document
    ///
    /// # Arguments
    ///
    /// * `did` - did to fetch data for
    ///
    /// # Example
    /// 
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_resolve("did:example:123").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("got did: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn did_resolve(
        &mut self,
        did: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("did_resolve");
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
                self.log_fun_leave("did_resolve", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not resolve did \"{}\"; {}",
                &did, e
            ))),
        }
    }

    /// Updates data related to new DID. May also persist a DID document for it, depending on plugin implementation.
    ///
    /// # Arguments
    ///
    /// * `did` - did to update data for
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_update("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("did successfully updated: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn did_update(
        &mut self,
        did: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("did_update");
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
                self.log_fun_leave("did_update", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not update did \"{}\"; {}",
                &did, e
            ))),
        }
    }

    /// Registers a new plugin. See [`VadePlugin`](https://docs.rs/vade/*/vade/struct.VadePlugin.html) for details about how they work.
    ///
    /// # Arguments
    ///
    /// * `plugin` - plugin to register
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// // use some_crate::ExamplePlugin;
    /// # use vade::VadePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn did_did_update() {
    ///     let mut vade = Vade::new();
    ///     let mut example_plugin = ExamplePlugin::new();
    ///     vade.register_plugin(Box::from(example_plugin));
    ///     let results = vade.did_create("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("did successfully updated: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub fn register_plugin(&mut self, plugin: Box<dyn VadePlugin>) {
        debug!("registering new vade plugin");
        self.plugins.push(plugin);
    }

    /// Creats a new zero-knowledge proof credential definition.
    ///
    /// # Arguments
    ///
    /// * `method` - method to create a credential definition for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_definition("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a credential definition: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_create_credential_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_create_credential_definition");
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
                self.log_fun_leave(
                    "vc_zkp_create_credential_definition",
                    filtered_results.len(),
                );
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create credential definition for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Creats a new zero-knowledge proof credential offer.
    ///
    /// # Arguments
    ///
    /// * `method` - method to create a credential offer for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_offer("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a credential offer: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_create_credential_offer(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_create_credential_offer");
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
                self.log_fun_leave("vc_zkp_create_credential_offer", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create credential offer for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Creats a new zero-knowledge proof credential proposal.
    ///
    /// # Arguments
    ///
    /// * `method` - method to create a credential proposal for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_proposal("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a credential proposal: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_create_credential_proposal(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_create_credential_proposal");
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
                self.log_fun_leave("vc_zkp_create_credential_proposal", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create credential proposal for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Creats a new zero-knowledge proof credential schema.
    ///
    /// # Arguments
    ///
    /// * `method` - method to create a credential schema for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_schema("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a credential schema: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_create_credential_schema(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_create_credential_schema");
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
                self.log_fun_leave("vc_zkp_create_credential_schema", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create credential schema for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Creats a new definition for a zero-knowledge proof revocation registry.
    ///
    /// # Arguments
    ///
    /// * `method` - method to create a revocation registry definition for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_revocation_registry_definition("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a revocation registry definition: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_create_revocation_registry_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_create_revocation_registry_definition");
        let mut futures = Vec::new();
        for plugin in self.plugins.iter_mut() {
            futures.push(
                plugin.vc_zkp_create_revocation_registry_definition(method, options, payload),
            );
        }
        match try_join_all(futures).await {
            Ok(responses) => {
                let mut filtered_results = Vec::new();
                for response in responses {
                    if let VadePluginResultValue::Success(value) = response {
                        filtered_results.push(value);
                    }
                }
                self.log_fun_leave(
                    "vc_zkp_create_revocation_registry_definition",
                    filtered_results.len(),
                );
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not create revocation registry definition for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Updates a revocation registry for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to update a revocation registry for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_update_revocation_registry("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("updated revocation registry: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_update_revocation_registry(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_update_revocation_registry");
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
                self.log_fun_leave("vc_zkp_update_revocation_registry", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not update revocation registry for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Issues a credential for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to issue a credential for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_issue_credential("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("issued credential: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_issue_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_issue_credential");
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
                self.log_fun_leave("vc_zkp_issue_credential", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not issue credential for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Presents a proof for a zero-knowledge proof credential.
    ///
    /// # Arguments
    ///
    /// * `method` - method to presents a proof for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_present_proof("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created a proof presentation: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_present_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_present_proof");
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
                self.log_fun_leave("vc_zkp_present_proof", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not present proof for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Requests a credential for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to request a credential for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_request_credential("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created credential request: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_request_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_request_credential");
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
                self.log_fun_leave("vc_zkp_request_credential", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not request credential for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Requests a proof for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to request a proof for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_request_proof("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("created proof request: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_request_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_request_proof");
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
                self.log_fun_leave("vc_zkp_request_proof", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not request proof for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Revokes a credential for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to revoke a credential for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_revoke_credential("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("revoked credential: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_revoke_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_revoke_credential");
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
                self.log_fun_leave("vc_zkp_revoke_credential", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not revoke credential for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Verifies a proof for a zero-knowledge proof.
    ///
    /// # Arguments
    ///
    /// * `method` - method to verify a proof for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_verify_proof("did:example", "", "").await.unwrap();
    ///     if !results.is_empty() {
    ///         println!("verified proof: {}", results[0].as_ref().unwrap());
    ///     }
    /// }
    /// ```
    pub async fn vc_zkp_verify_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
        self.log_fun_enter("vc_zkp_verify_proof");
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
                self.log_fun_leave("vc_zkp_verify_proof", filtered_results.len());
                Ok(filtered_results)
            }
            Err(e) => Err(Box::from(format!(
                "could not verify proof for method \"{}\"; {}",
                &method, e
            ))),
        }
    }

    /// Writes a debug message when entering a plugin function.
    ///
    /// # Arguments
    ///
    /// * `name` - name of called function
    fn log_fun_enter(&mut self, name: &str) {
        debug!(
            r#"delegating function "{}" to {} plugins"#,
            &name,
            self.plugins.len()
        );
    }


    /// Writes a debug message when leaving a plugin function.
    ///
    /// # Arguments
    ///
    /// * `name` - name of called function
    /// * `response_count` - number of `VadePluginResultValue::Success(T)` responses
    fn log_fun_leave(&mut self, name: &str, response_count: usize) {
        debug!(
            r#"function "{}" of {} plugins yielded {} results"#,
            &name,
            self.plugins.len(),
            &response_count,
        );
    }
}

impl Default for Vade {
    /// Default `Vade` instance
    fn default() -> Self {
        Vade::new()
    }
}
