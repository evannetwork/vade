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
use serde_json::Value;
use vade::traits::MessageConsumer;
use vade::Vade;


pub struct TestMessageConsumer {
    message_count: u64,
}

impl TestMessageConsumer {
    pub fn new() -> TestMessageConsumer {
        TestMessageConsumer {
            message_count: 0,
        }
    }
}

#[async_trait(?Send)]
impl MessageConsumer for TestMessageConsumer {
    async fn handle_message(
        &mut self,
        _message_type: &str,
        message_data: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.message_count = self.message_count + 1;
        Ok(Option::from(format!(r###"{{ "type": "response", "data": {{ "count": {}, "lastMessage": {} }} }}"###, self.message_count, &message_data).to_string()))
    }
}

#[tokio::test]
async fn library_message_consumer_can_be_registered() {
    let mut vade = Vade::new();
    let tmc = TestMessageConsumer::new();
    vade.register_message_consumer(
        &vec!["message1", "message2"].iter().map(|&x| String::from(x)).collect(),
        Box::from(tmc),
    );
}

#[tokio::test]
async fn library_message_consumer_can_receive_messages() {
    let mut vade = Vade::new();
    let tmc = TestMessageConsumer::new();
    vade.register_message_consumer(
        &vec!["message1", "message2"].iter().map(|&x| String::from(x)).collect(),
        Box::from(tmc),
    );

    // regular increase
    let responses = vade.send_message(r###"{ "type": "message1", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 1);

    // regular increase
    let responses = vade.send_message(r###"{ "type": "message1", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 2);

    // regular increase
    let responses = vade.send_message(r###"{ "type": "message1", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 3);
}

#[tokio::test]
async fn library_message_consumer_can_ignore_messages() {
    let mut vade = Vade::new();
    let tmc = TestMessageConsumer::new();
    vade.register_message_consumer(
        &vec!["message1", "message2"].iter().map(|&x| String::from(x)).collect(),
        Box::from(tmc),
    );

    // regular increase
    let responses = vade.send_message(r###"{ "type": "message1", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 1);

    // regular increase
    let responses = vade.send_message(r###"{ "type": "message2", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 2);

    // no response as type does not match
    let responses = vade.send_message(r###"{ "type": "message3", "data": {} }"###).await.unwrap();
    assert_eq!(responses.len(), 0);

    // counting resumes from last accepted message
    let responses = vade.send_message(r###"{ "type": "message2", "data": {} }"###).await.unwrap();
    let parsed: Value = serde_json::from_str(responses[0].as_ref().unwrap()).unwrap();
    assert_eq!(parsed["data"]["count"].as_u64().unwrap(), 3);
}
