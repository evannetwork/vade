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

extern crate vade;

use async_trait::async_trait;
use vade::{Vade, VadePlugin, VadePluginResultValue};

const EXAMPLE_DID_DOCUMENT_STR: &str = r###"{
    "@context": "https://www.w3.org/ns/did/v1",
    "id": "did:example:123456789abcdefghi"
}"###;

pub struct TestPlugin {}

impl TestPlugin {
    pub fn new() -> TestPlugin {
        TestPlugin {}
    }
}

impl Default for TestPlugin {
    fn default() -> Self {
        TestPlugin::new()
    }
}

#[async_trait(?Send)]
impl VadePlugin for TestPlugin {
    // test plugin did_create handles this request
    async fn did_create(
        &mut self,
        _did_method: &str,
        _options: &str,
        _payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::Success(Some(
            EXAMPLE_DID_DOCUMENT_STR.to_string(),
        )))
    }

    // test plugin did_resolve just ignores this request
    async fn did_resolve(
        &mut self,
        _did: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Ok(VadePluginResultValue::Ignored)
    }

    // test plugin did_update returns an error
    async fn did_update(
        &mut self,
        _did: &str,
        _options: &str,
        _payload: &str,
    ) -> Result<VadePluginResultValue<Option<String>>, Box<dyn std::error::Error>> {
        Err(Box::from("yikes"))
    }
}

#[tokio::test]
async fn vade_plugin_plugin_can_call_functions_implemented_in_plugin() {
    let mut tp: TestPlugin = TestPlugin::new();
    match tp.did_create("", "", "").await {
        Ok(response) => match response {
            VadePluginResultValue::Success(result) => {
                assert_eq!(result.unwrap(), EXAMPLE_DID_DOCUMENT_STR.to_string())
            }
            _ => panic!("unexpected result"),
        },
        Err(e) => panic!(format!("{}", e)),
    }
}

#[tokio::test]
async fn vade_plugin_plugin_can_call_fallback_for_not_implemented() {
    let mut tp: TestPlugin = TestPlugin::new();
    match tp.vc_zkp_verify_proof("", "", "").await {
        Ok(response) => {
            assert!(match response {
                VadePluginResultValue::NotImplemented => true,
                _ => false,
            });
        }
        Err(e) => panic!(format!("{}", e)),
    }
}

#[tokio::test]
async fn vade_plugin_vade_can_call_functions_implemented_in_plugin() {
    let tp: TestPlugin = TestPlugin::new();
    let mut vade = Vade::new();
    vade.register_plugin(Box::from(tp));
    match vade.did_create("", "", "").await {
        Ok(results) => assert_eq!(
            results[0].as_ref().unwrap().to_string(),
            EXAMPLE_DID_DOCUMENT_STR.to_string()
        ),
        Err(e) => panic!(format!("{}", e)),
    };
}
