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

use crate::{VadePlugin, VadePluginResultValue, VadeResult};

/// A [`Vade`] instance is your single point of contact for interacting with DIDs and VCs.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_create("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created new did: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn did_create(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "did_create";
        self.log_fun_enter(&task_name, &did_method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.did_create(did_method, options, payload)?);
        }
        self.filter_results(task_name, did_method, results)
    }

    /// Fetch data about a DID. This usually returns a DID document.
    ///
    /// # Arguments
    ///
    /// * `did` - did to fetch data for
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_resolve("did:example:123")?;
    ///     if !results.is_empty() {
    ///         println!("got did: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn did_resolve(&mut self, did: &str) -> VadeResult<Vec<Option<String>>> {
        let task_name = "did_resolve";
        self.log_fun_enter(&task_name, &did);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.did_resolve(did)?);
        }
        self.filter_results(task_name, did, results)
    }

    /// Updates data related to a DID. May also persist a DID document for it, depending on plugin implementation.
    ///
    /// # Arguments
    ///
    /// * `did` - DID to update data for
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.did_update("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("did successfully updated: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn did_update(
        &mut self,
        did: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "did_update";
        self.log_fun_enter(&task_name, &did);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.did_update(did, options, payload)?);
        }
        self.filter_results(task_name, did, results)
    }

    /// Processes a DIDComm message as received, this may prepare a matching response for it
    /// if the DIDComm message can be interpreted and answered by a plugin's implementation.
    ///
    /// This response **may** be sent, depending on the configuration and implementation of
    /// underlying plugins, but it is usually also returned as response to this request.
    ///
    /// # Arguments
    ///
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (usually a raw DIDComm message)
    ///
    /// # Example
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.didcomm_receive("", "")?;
    ///     if !results.is_empty() {
    ///         println!("received DIDComm message: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn didcomm_receive(
        &mut self,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "didcomm_receive";
        self.log_fun_enter(&task_name, &task_name);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.didcomm_receive(options, payload)?);
        }
        self.filter_results(task_name, task_name, results)
    }

    /// Processes a DIDComm message and prepares it for sending.
    ///
    /// It **may** be sent, depending on the configuration and implementation of underlying plugins.
    ///
    /// # Arguments
    ///
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (usually a raw DIDComm message)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.didcomm_send("", "")?;
    ///     if !results.is_empty() {
    ///         println!("prepared DIDComm message: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn didcomm_send(
        &mut self,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "didcomm_send";
        self.log_fun_enter(&task_name, &task_name);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.didcomm_send(options, payload)?);
        }
        self.filter_results(task_name, task_name, results)
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     let mut example_plugin = ExamplePlugin::new();
    ///     vade.register_plugin(Box::from(example_plugin));
    ///     let results = vade.did_create("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("did successfully updated: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn register_plugin(&mut self, plugin: Box<dyn VadePlugin>) {
        debug!("registering new vade plugin");
        self.plugins.push(plugin);
    }

    /// Runs a custom function, this allows to use `Vade`s API for custom calls, that do not belong
    /// to `Vade`s core functionality but may be required for a projects use cases.
    ///
    /// # Arguments
    ///
    /// * `method` - method to call a function for (e.g. "did:example")
    /// * `function` - function to call (e.g. "test connection")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.run_custom_function("did:example", "test connection", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("connection status is: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn run_custom_function(
        &mut self,
        method: &str,
        function: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "run_custom_function";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.run_custom_function(method, function, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Creates a new zero-knowledge proof credential definition. A credential definition holds cryptographic key material
    /// and is needed by an issuer to issue a credential, thus needs to be created before issuance. A credential definition
    /// is always bound to one credential schema.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_definition("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a credential definition: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_create_credential_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_create_credential_definition";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_create_credential_definition(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Creates a new zero-knowledge proof credential offer. This message is the response to a credential proposal.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_offer("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a credential offer: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_create_credential_offer(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_create_credential_offer";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_create_credential_offer(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Creates a new zero-knowledge proof credential proposal. This message is the first in the
    /// credential issuance flow.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_proposal("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a credential proposal: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_create_credential_proposal(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_create_credential_proposal";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_create_credential_proposal(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Creates a new zero-knowledge proof credential schema. The schema specifies properties a credential
    /// includes, both optional and mandatory.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_credential_schema("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a credential schema: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_create_credential_schema(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_create_credential_schema";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_create_credential_schema(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Creates a new revocation registry definition. The definition consists of a public and a private part.
    /// The public part holds the cryptographic material needed to create non-revocation proofs. The private part
    /// needs to reside with the registry owner and is used to revoke credentials.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_create_revocation_registry_definition("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a revocation registry definition: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_create_revocation_registry_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_create_revocation_registry_definition";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(
                plugin.vc_zkp_create_revocation_registry_definition(method, options, payload)?,
            );
        }
        self.filter_results(task_name, method, results)
    }

    /// Updates a revocation registry for a zero-knowledge proof. This step is necessary after revocation one or
    /// more credentials.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_update_revocation_registry("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("updated revocation registry: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_update_revocation_registry(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_update_revocation_registry";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_update_revocation_registry(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Issues a new credential. This requires an issued schema, credential definition, an active revocation
    /// registry and a credential request message.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_issue_credential("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("issued credential: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_issue_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_issue_credential";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_issue_credential(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Finishes a credential, e.g. by incorporating the prover's master secret into the credential signature after issuance.
    ///
    /// # Arguments
    ///
    /// * `method` - method to update a finish credential for (e.g. "did:example")
    /// * `options` - JSON string with additional information supporting the request (e.g. authentication data)
    /// * `payload` - JSON string with information for the request (e.g. actual data to write)
    ///
    /// # Example
    ///
    /// ```
    /// use vade::Vade;
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_finish_credential("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("issued credential: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_finish_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_finish_credential";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_finish_credential(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Presents a proof for a zero-knowledge proof credential. A proof presentation is the response to a
    /// proof request.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_present_proof("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created a proof presentation: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_present_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_present_proof";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_present_proof(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Requests a credential. This message is the response to a credential offering.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_request_credential("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created credential request: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_request_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_request_credential";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_request_credential(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Requests a zero-knowledge proof for one or more credentials issued under one or more specific schemas.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_request_proof("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("created proof request: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_request_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_request_proof";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_request_proof(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Revokes a credential. After revocation the published revocation registry needs to be updated with information
    /// returned by this function.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_revoke_credential("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("revoked credential: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_revoke_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_revoke_credential";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_revoke_credential(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    /// Verifies one or multiple proofs sent in a proof presentation.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut vade = Vade::new();
    ///     // // register example plugin e.g. with
    ///     // vade.register_plugin(example_plugin);
    ///     let results = vade.vc_zkp_verify_proof("did:example", "", "")?;
    ///     if !results.is_empty() {
    ///         println!("verified proof: {}", results[0].as_ref().ok_or("result not found")?);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn vc_zkp_verify_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> VadeResult<Vec<Option<String>>> {
        let task_name = "vc_zkp_verify_proof";
        self.log_fun_enter(&task_name, &method);
        let mut results = Vec::new();
        for plugin in self.plugins.iter_mut() {
            results.push(plugin.vc_zkp_verify_proof(method, options, payload)?);
        }
        self.filter_results(task_name, method, results)
    }

    fn filter_results<T>(
        &self,
        task_name: &str,
        did_or_method: &str,
        results: Vec<VadePluginResultValue<T>>,
    ) -> VadeResult<Vec<T>> {
        let mut filtered_results = Vec::new();
        for result in results {
            if let VadePluginResultValue::Success(value) = result {
                filtered_results.push(value);
            }
        }
        self.log_fun_leave(&task_name, filtered_results.len(), &did_or_method);

        Ok(filtered_results)
    }

    /// Writes a debug message when entering a plugin function.
    ///
    /// # Arguments
    ///
    /// * `name` - name of called function
    fn log_fun_enter(&mut self, name: &str, method_or_id: &str) {
        debug!(
            r#"delegating function "{}" to {} plugins with method/id "{}""#,
            &name,
            self.plugins.len(),
            method_or_id,
        );
    }

    /// Writes a debug message when leaving a plugin function.
    ///
    /// # Arguments
    ///
    /// * `name` - name of called function
    /// * `response_count` - number of `VadePluginResultValue::Success(T)` responses
    fn log_fun_leave(&self, name: &str, response_count: usize, method_or_id: &str) {
        debug!(
            r#"function "{}" of {} plugins yielded {} results for method/id "{}""#,
            &name,
            self.plugins.len(),
            &response_count,
            method_or_id,
        );
    }
}

impl Default for Vade {
    /// Default `Vade` instance
    fn default() -> Self {
        Vade::new()
    }
}
