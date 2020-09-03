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
    /// Plugin does not implement this function. This is returned by default as the
    /// [`VadePlugin`](https://docs.rs/vade/*/vade/trait.VadePlugin.html)
    /// trait offers a default implementation for every function (which returns `NotImplemented`).
    /// So if a function is not explicitly implemented by a plugin itself, a call to this function
    /// will return `NotImplemented`.
    NotImplemented,
    /// Plugin implements function but is not "interested" in fulfilling function call.
    /// This mostly signs that the responding plugin does not resolve/handle given method,
    /// e.g. a plugin may resolve dids with prefix `did:example123` and not dids with
    /// prefix `did:example456`.
    Ignored,
    /// Plugin handled request and returned a value of type `T`. Note that `Success` values can be
    /// unwrapped. So if you know, that a plugin implements a function and handles requests of your
    /// method, you can call
    /// [`unwrap`](https://docs.rs/vade/*/vade/enum.VadePluginResultValue.html#method.unwrap) on it
    /// to fetch the underlying value of type `T`.
    Success(T),
}

impl<T> VadePluginResultValue<T> {
    /// Unwraps inner value like:
    /// - `Success(T)` unwraps successfully to `T`
    /// - `NotImplemented` and `Ignored` unwrap to errors
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

/// ## About
///
/// The plugins are the bread and butter of the underlying [`Vade`] logic. [`Vade`] is your single
/// point of contact in your application and all your calls are executed against it. [`Vade`] itself
/// manages the plugins, delegates calls to them and filters the results. The actual logic
/// concerning specific DID methods resides in the plugins and they are responsible for implementing
/// argument handling, resolving DIDs, etc.
///
/// ## Call delegation
///
/// All functions of the [`VadePlugin`] trait have a counterpart with the same name but a slightly
/// different signature in [`Vade`] that will delegate calls to the plugins' functions with the same
/// name. While the plugin returns a `VadePluginResultValue<T>`result, [`Vade`] will return
/// a `Vec<T>` result. [`Vade`]'s result is the list of all results from all plugins that did
/// implement the called function and did not ignore the request.
///
/// For example [`did_create`](https://docs.rs/vade/*/vade/struct.Vade.html#method.did_create)
/// / [`did_create`](https://docs.rs/vade/*/vade/trait.VadePlugin.html#method.did_create):
///
/// [`Vade`]'s function:
///
/// ```ignored
/// pub async fn did_create(
///     &mut self,
///     did_method: &str,
///     options: &str,
///     payload: &str,
///     ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
///     // ...
/// }
/// ```
///
/// Will call all [`VadePlugin`]s' functions:
///
/// ```ignored
/// pub async fn did_create(
///     &mut self,
///     did_method: &str,
///     options: &str,
///     payload: &str,
/// ) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
///     // ...
/// }
/// ```
///
/// ## Result Values of Plugins
///
/// Plugins return results with the type [`VadePluginResultValue`], which has 3 Variants:
///
/// - [`NotImplemented`], for functions not implemented in a plugin
/// - [`Ignored`], for functions implemented in a plugin but ignore the request (e.g. due to an unknown method)
/// - [`Success`], for successful requests' results
///
/// ## Example
///
/// A simple plugin could look like this:
///
/// ```rust
/// use async_trait::async_trait;
/// use vade::{VadePlugin, VadePluginResultValue};
///
/// struct ExamplePlugin { }
///
/// impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
///
/// #[async_trait(?Send)]
/// impl VadePlugin for ExamplePlugin {
///     async fn did_create(
///         &mut self,
///         _did_method: &str,
///         _options: &str,
///         _payload: &str,
///     ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
///         Ok(VadePluginResultValue::Success(Some(
///             r#"{ "id": "did:example123:456" }"#.to_string(),
///         )))
///     }
/// }
/// ```
///
/// There is no need to implement all [`VadePlugin`] functions, unimplemented functions will be
/// ignored. Also make sure to return [`Ignored`], your function is not responsible for a given
/// did or method.
///
/// [`Ignored`]: https://docs.rs/vade/*/vade/enum.VadePluginResultValue.html#variant.Ignored
/// [`NotImplemented`]: https://docs.rs/vade/*/vade/enum.VadePluginResultValue.html#variant.NotImplemented
/// [`Success`]: https://docs.rs/vade/*/vade/enum.VadePluginResultValue.html#variant.Success
/// [`Vade`]: https://docs.rs/vade/*/vade/struct.Vade.html
/// [`VadePlugin`]: https://docs.rs/vade/*/vade/trait.VadePlugin.html
/// [`VadePluginResultValue`]: https://docs.rs/vade/*/vade/enum.VadePluginResultValue.html

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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_create("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created new did: {}", &value);
    ///     }
    ///     Ok(())
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

    /// Fetch data about a DID. This usually returns a DID document.
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_resolve("did:example:123").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("got did: {}", &value);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    async fn did_resolve(
        &mut self,
        _did: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.did_update("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("updated did: {}", &value);
    ///     }
    ///     Ok(())
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.run_custom_function("did:example", "test connection", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("connection status: {}", &value);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    async fn run_custom_function(
        &mut self,
        method: &str,
        function: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
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
    /// use vade::{VadePlugin, VadePluginResultValue};
    /// // use some_crate:ExamplePlugin;
    /// # struct ExamplePlugin { }
    /// # impl ExamplePlugin { pub fn new() -> Self { ExamplePlugin {} } }
    /// # impl VadePlugin for ExamplePlugin {}
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_definition("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("successfully created a credential definition: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_offer("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential offer: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_proposal("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential proposal: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_credential_schema("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a credential schema: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_create_revocation_registry_definition("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a revocation registry definition: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_update_revocation_registry("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("updated revocation registry: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_issue_credential("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("issued credential: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_present_proof("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created a proof presentation: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_request_credential("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created credential request: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_request_proof("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("created proof request: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_revoke_credential("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("revoked credential: {}", &value);
    ///     }
    ///     Ok(())
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
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ep: ExamplePlugin = ExamplePlugin::new();
    ///     let result = ep.vc_zkp_verify_proof("did:example", "", "").await?;
    ///     if let VadePluginResultValue::Success(Some(value)) = result {
    ///         println!("verified proof: {}", &value);
    ///     }
    ///     Ok(())
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
