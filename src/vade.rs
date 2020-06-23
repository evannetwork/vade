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

/// Vade library, that holds plugins and delegates calls to them.
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

    pub fn register_plugin(&mut self, plugin: Box<dyn VadePlugin>) {
        debug!("registering new vade plugin");
        self.plugins.push(plugin);
    }

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

    fn log_fun_enter(&mut self, name: &str) {
        debug!(
            r#"delegating function "{}" to {} plugins"#,
            &name,
            self.plugins.len()
        );
    }

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
