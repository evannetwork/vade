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

//! Traits for interoperability with [`Vade`] instances.
//! 
//! [`Vade`]: crate::Vade

use async_trait::async_trait;
use std::any::Any;

/// Wrapper enum for a plugins return value
pub enum VadePluginResultValue<T> {
    /// Plugin does not implement this function
    NotImplemented,
    /// Plugin implements function but is not "interested" in fullfilling function call
    Ignored,
    /// Plugin handled request and returned a value of type T
    Success(T),
}

/// Implementing struct supports fetching did documents by their id.
#[async_trait(?Send)]
pub trait DidResolver {
    /// Checks given DID document.
    /// A DID document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this DID
    /// - that it considers this DID as invalid
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to check document for
    /// * `value` - value to check
    async fn check_did(
        &self,
        did_name: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Gets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to fetch
    async fn get_did_document(
        &self,
        key: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// Sets document for given did name.
    ///
    /// # Arguments
    ///
    /// * `did_name` - did_name to set value for
    /// * `value` - value to set
    async fn set_did_document(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// Implementing struct supports logging, for now only `log` is supported.
pub trait Logger {
    /// Cast to `Any` for downcasting,
    /// see https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object.
    fn as_any(
        &self,
    ) -> &dyn Any;

    /// Logs given message with given level.
    /// 
    /// # Arguments
    ///
    /// * `message` - message to log
    /// * `level` - optional arguments for logging level, levels may differ based on environment
    fn log(
        &self,
        message: &str,
        level: Option<&str>,
    );
}

#[async_trait(?Send)]
/// Implementing sruct support generic message handling, has to be registered with
/// Vade::register_message_consumer and subscribed to specific message types.
pub trait MessageConsumer {
    /// Reacts to a given message and optionally returns a reply.
    /// 
    /// # Arguments
    /// 
    /// * `message_type` - type of message this consumer had subscribed for
    /// * `message_data` - arbitrary data for plugin, e.g. a JSON
    async fn handle_message(
        &mut self,
        message_type: &str,
        message_data: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;
}

/// Implementing struct supports fetching vc documents by their id.
#[async_trait(?Send)]
pub trait VcResolver {
    /// Checks given VC document.
    /// A VC document is considered as valid if returning ().
    /// Resolver may throw to indicate
    /// - that it is not responsible for this VC
    /// - that it considers this VC as invalid
    ///
    /// # Arguments
    ///
    /// * `vc_id` - vc_id to check document for
    /// * `value` - value to check
    async fn check_vc(
        &self,
        vc_id: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Gets document for given vc name.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to fetch
    async fn get_vc_document(
        &self,
        vd_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;

    /// Sets document for given vc name.
    ///
    /// # Arguments
    ///
    /// * `vc_name` - vc_name to set value for
    /// * `value` - value to set
    async fn set_vc_document(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait(?Send)]
#[allow(unused_variables)]
pub trait VadePlugin {
    async fn did_create(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    async fn did_resolve(&mut self, _did: &str) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    async fn did_update(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }
    /// Creates a new credential definition and stores it on-chain.
    async fn vc_zkp_create_credential_definition(
        &mut self,
        did_method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a `CredentialOffer` message.
    async fn vc_zkp_create_credential_offer(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a `CredentialProposal` message.
    async fn vc_zkp_create_credential_proposal(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a new credential schema and stores it on-chain.
    async fn vc_zkp_create_credential_schema(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a new revocation registry definition and stores it on-chain.
    async fn vc_zkp_create_revocation_registry_definition(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    async fn vc_zkp_update_revocation_registry(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Issues a new credential.
    async fn vc_zkp_issue_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Creates a `CredentialProof` message.
    async fn vc_zkp_present_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }
    
    /// Creates a `CredentialRequest` message.
    async fn vc_zkp_request_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }
    
    /// Creates a `ProofRequest` message
    async fn vc_zkp_request_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Revokes a credential and updates the revocation registry definition.
    async fn vc_zkp_revoke_credential(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }

    /// Verifies a given proof presentation in accordance to specified proof request
    async fn vc_zkp_verify_proof(
        &mut self,
        method: &str,
        options: &str,
        payload: &str,
    ) -> Result<VadePluginResultValue<String>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::NotImplemented)
    }
}
