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

use async_trait::async_trait;

/// Wrapper enum for a plugins return value
pub enum VadePluginResultValue<T> {
    /// Plugin does not implement this function, this is returned by default as the
    /// [`VadePlugin`](https://docs.rs/vade/*/vade/trait.VadePlugin.html)
    /// trait offers a default implementation for every function (which returns `NotImplemented`).
    NotImplemented,
    /// Plugin implements function but is not "interested" in fulfilling function call.
    /// This mostly signs that the responding plugin does not resolve/handle given method,
    /// e.g. a plugin may resolve dids with prefix `did:example123` and not dids with
    /// prefix `did:example456`.
    Ignored,
    /// Plugin handled request and returned a value of type `T`.
    Success(T),
}

impl<T> VadePluginResultValue<T> {
    /// Unwraps inner value like:
    /// - `Success(T)` unwraps successfully to `T`
    /// - NotImplemented unwraps to an error
    /// - Ignored unwraps to an error
    /// ```
    pub fn unwrap(self) -> T {
        match self {
            VadePluginResultValue::Success(val) => val,
            VadePluginResultValue::NotImplemented => {
                panic!("called `VadePluginResultValue::unwrap()` on a `NotImplemented` value")
            }
            VadePluginResultValue::Ignored => {
                panic!("called `VadePluginResultValue::unwrap()` on a `Ignored` value")
            }
        }
    }
}

#[async_trait(?Send)]
#[allow(unused_variables)] // to keep proper names for documentation and derived implementations
pub trait VadePlugin {
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_create("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created new did: {}", &value);
    ///     }
    /// }
    /// ```
    async fn did_create(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_resolve("did:example:123").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("got did: {}", &value);
    ///     }
    /// }
    /// ```
    async fn did_resolve(
        &mut self,
        _did: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_update("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("updated did: {}", &value);
    ///     }
    /// }
    /// ```
    async fn did_update(
        &mut self,
        did: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a new zero-knowledge proof credential definition. A credential definition holds cryptographic key mateiral
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_definition("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("successfully created a credential definition: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_create_credential_definition(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_offer("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential offer: {}", &value);
    ///     }
    /// }
    async fn vc_zkp_create_credential_offer(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_proposal("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential proposal: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_create_credential_proposal(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_schema("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential schema: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_create_credential_schema(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_revocation_registry_definition("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a revocation registry definition: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_create_revocation_registry_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_update_revocation_registry("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("updated revocation registry: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_update_revocation_registry(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_issue_credential("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("issued credential: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_issue_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_present_proof("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a proof presentation: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_present_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_request_credential("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created credential request: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_request_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_request_proof("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created proof request: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_request_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_revoke_credential("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("revoked credential: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_revoke_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Verifies a one or multiple proofs sent in a proof presentation.
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_verify_proof("did:example", "", "").await.unwrap();
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("verified proof: {}", &value);
    ///     }
    /// }
    /// ```
    async fn vc_zkp_verify_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }
}
