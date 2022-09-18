//! Ntropy SDK
//!
//! The Ntropy API provides transaction enrichment and categorization, account ledger, metrics and custom model training. The full documentation is available at the developer portal.
//!
//! # Examples
//! ```rust
//! use ntropy_sdk::{AccountHolderType, Client, EntryType, Transaction};
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("NTROPY_API_KEY").expect("failed to get NTROPY_API_KEY");
//!     let client = Client::new(&api_key);
//!
//!     let transactions = vec![Transaction {
//!         description: "AMAZON WEB SERVICES AWS.AMAZON.CO WA Ref5543286P25S Crd15".to_string(),
//!         entry_type: EntryType::Outgoing,
//!         amount: 12042.37,
//!         iso_currency_code: "USD".to_string(),
//!         date: Some("2021-11-01".to_string()),
//!         transaction_id: "4yp49x3tbj9mD8DB4fM8DDY6Yxbx8YP14g565Xketw3tFmn".to_string(),
//!         country: Some("US".to_string()),
//!         account_holder_id: Some("id-1".to_string()),
//!         account_holder_type: Some(AccountHolderType::Business),
//!         ..Default::default()
//!     }];
//!
//!     let mut result = client
//!         .enrich_transactions(transactions, None, Some(false), None, None)
//!         .await;
//!
//!     while let Some(data) = result.recv().await {
//!         match data {
//!             Ok(data) => println!("transaction: {:#?}", data),
//!             Err(e) => println!("error: {:#?}", e),
//!         }
//!     }
//! }
//! ```
pub mod client;
pub mod errors;
mod helpers;

pub use client::{Client, DEFAULT_CHANNEL_BUFFER_SIZE, DEFAULT_POLL_RATE};
pub use errors::NtropyError;
pub use ntropy_api_client::models::{
    transaction::AccountHolderType, transaction::EntryType, EnrichedTransaction, Transaction,
    TransactionError,
};
