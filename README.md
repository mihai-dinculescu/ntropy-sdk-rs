# Ntropy SDK

This repository hosts the SDK for the Ntropy API. To use the Ntropy API you require an API key which can be requested at [ntropy.com](https://ntropy.com).

The Ntropy API provides transaction enrichment and categorization, account ledger, metrics and custom model training. The full documentation is available at the [developer portal](https://developers.ntropy.com).

## Running the examples

```bash
NTROPY_API_KEY=<api-key> cargo run --example enrich_transactions
```

## Contributing

### Tests

```bash
NTROPY_API_KEY=<api-key> cargo test
```

### Docs

```bash
cargo doc --package ntropy-sdk --open
```

### Re-generating the Open API client

```bash
./generate-api-client.sh
```

#### manual fixes

##### `Cargo.toml`

- change edition to 2021

##### `src/lib.rs`

- Add `#![allow(clippy::all)]` to the top of the file

##### `src/apis/misc_api.rs`

- add `use crate::models::AccountHolderType;` to the top of the file
- change `account_holder_type = account_holder_type` to `account_holder_type = account_holder_type.to_string()`

##### `src/models/ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner.rs`

- change `error: String` to `error: Option<String>`
- in the constructor, change `error,` to `error: Some(error),`

##### `src/models/transaction_error.rs`

- change `pub error_details: Option<Box<crate::models::TransactionErrorErrorDetails>>,` to `pub error_details: Option<serde_json::Value>,`
