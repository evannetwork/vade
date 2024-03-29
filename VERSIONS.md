# vade

## Next Version

### Features

### Fixes

### Deprecations

## Version 0.1.1

### Features

- add `vc_zkp_propose_proof` to `Vade` and `VadePlugin` to allow creating proof proposal via plugins

## Version 0.1.0

### Features

- add functions for handling DIDComm messages to `Vade` and `VadePlugin`:
  - `didcomm_receive`
  - `didcomm_send`

## Version 0.0.8

### Features

- add `run_custom_function` to support custom logic

### Fixes

- remove unwrap calls

## Version 0.0.7

### Features

- add `VadePlugin` trait
- add `VadePluginResultValue` enum
- update `Vade` to be able to work with `VadePlugins`

### Deprecations

- remove `DidResolver`, `Logger`, `VcResolver`, `MessageHandler`
  - traits
  - `Vade` functions that use them
  - tests related to them
- remove `RustStorageCache` example implementation and tests

## Version 0.0.6

### Features

- add support for wasm compilation
- update documentation

## Version 0.0.5

### Fixes

- update documentation (grammar, wording)
- update links in docu

## Version 0.0.4

### Fixes

- update links in `Readme.md` and src files
- add license file
- add badges

## Version 0.0.3

### Fixes

- fix links in `Readme.md`

## Version 0.0.2

### Fixes

- fix unnecessary visibility qualifier
- add documentation link to `Cargo.toml`

## Version 0.0.1

- initial version
