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

//! This library is intended to be used as a core library for developing own self-sovereign identity based applications.
//!
//! So why the name? "Vade" is an acronym for "VC and DID engine". It focuses on working with VCs and DIDs but does not hold much logic concerning their structure. Confused? Guessed as much, so what this library actually does is:
//!

extern crate env_logger;
#[macro_use]
extern crate log;

mod vade;
mod vade_plugin;

pub use self::vade::Vade;
pub use self::vade_plugin::{VadePlugin, VadePluginResultValue};
