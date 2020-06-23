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

/// Wrapper enum for a plugins return value
pub enum VadePluginResultValue<T> {
    /// Plugin does not implement this function
    NotImplemented,
    /// Plugin implements function but is not "interested" in fullfilling function call.
    /// This mostly signs that the responding plugin does not resolve/handle given method,
    /// e.g. a plugin may resolve dids with prefix `did:example123` and not dids with
    /// prefix `did:example456`.
    Ignored,
    /// Plugin handled request and returned a value of type T
    Success(T),
}

#[async_trait(?Send)]
#[allow(unused_variables)]  // to keep proper names for documentation and derived implementations
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
        did: &str,
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
