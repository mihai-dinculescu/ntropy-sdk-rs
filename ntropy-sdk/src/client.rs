use std::time::Duration;

use ntropy_api_client::apis::enrichment_api::{
    ntropy_server_period_api_period_v22_period_enrich_transactions_async,
    ntropy_server_period_api_period_v22_period_get_async_enrichment_result,
};
use ntropy_api_client::models::batch_result::Status;
use ntropy_api_client::models::{BatchResult, EnrichedTransaction, Transaction};

use ntropy_api_client::apis::configuration::{ApiKey, Configuration};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time;

use crate::errors::NtropyError;
use crate::helpers::map_enriched_transaction;

/// Default poll rate for checking the status of a batch.
pub const DEFAULT_POLL_RATE: Duration = Duration::from_millis(100);
/// Default channel buffer size for receiving enriched transactions.
pub const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 65536;

/// Ntropy SDK Client
pub struct Client {
    configuration: Configuration,
    poll_rate: Duration,
    channel_buffer_size: usize,
}

impl Client {
    /// Returns a new Ntropy SDK Client
    ///
    /// # Arguments
    /// * `api_key` - Ntropy API Key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ntropy_sdk::Client;
    ///
    /// let client = Client::new("ntropy-api-key");
    /// ```
    pub fn new(api_key: &str) -> Self {
        let configuration = Configuration {
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key.to_string(),
            }),
            ..Default::default()
        };

        Self {
            configuration,
            poll_rate: DEFAULT_POLL_RATE,
            channel_buffer_size: DEFAULT_CHANNEL_BUFFER_SIZE,
        }
    }

    /// Sets the poll rate interval for the client
    ///
    /// # Arguments
    /// * `poll_rate` - Poll for results interval. The default is [`crate::DEFAULT_POLL_RATE`].
    ///
    /// # Example
    /// ```rust
    /// use ntropy_sdk::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::new("ntropy-api-key")
    ///       .with_poll_rate(Duration::from_millis(500));
    /// ```
    pub fn with_poll_rate(self, poll_rate: Duration) -> Self {
        Self { poll_rate, ..self }
    }

    /// Sets the channel buffer size
    ///
    /// # Arguments
    /// * `channel_buffer_size` - The buffer size of the `Receiver` channel. The default is [`crate::DEFAULT_CHANNEL_BUFFER_SIZE`].
    ///
    /// # Example
    /// ```rust
    /// use ntropy_sdk::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::new("ntropy-api-key")
    ///       .with_channel_buffer_size(1024);
    /// ```
    pub fn with_channel_buffer_size(self, channel_buffer_size: usize) -> Self {
        Self {
            channel_buffer_size,
            ..self
        }
    }

    /// Enrich and add transactions to the ledgers of account holders asynchronously.
    ///
    /// Returns a [`tokio::sync::mpsc::Receiver`] channel that can be used to receive the results of the enrichment.
    ///
    /// # Arguments
    /// * `transactions` - A list of transactions to enrich
    /// * `labeling` - If enabled, this flag will assign category labels to the supplied transaction. By default, this is true.
    /// * `create_account_holders` - If enabled, this flag will create any non-existent account holder in the provided transaction list, as long as an account_holder_type is provided. By default, this is true.
    /// * `model_name` - If provided, the transaction enrichment will use the referenced custom model for labelling.
    /// * `timeout` - The timeout for the request. If not provided, the request will run indefinitely.
    pub async fn enrich_transactions(
        &self,
        transactions: Vec<Transaction>,
        labeling: Option<bool>,
        create_account_holders: Option<bool>,
        model_name: Option<&str>,
        timeout: Option<Duration>,
    ) -> Receiver<Result<EnrichedTransaction, NtropyError>> {
        let (tx, rx) =
            channel::<Result<EnrichedTransaction, NtropyError>>(self.channel_buffer_size);

        // send the batch to be processed
        let result: Result<BatchResult, NtropyError> =
            ntropy_server_period_api_period_v22_period_enrich_transactions_async(
                &self.configuration,
                transactions,
                labeling,
                create_account_holders,
                model_name,
            )
            .await
            .map_err(|e| e.into());

        // poll the batch result for however long the timeout permits
        let configuration = self.configuration.clone();
        let poll_rate = self.poll_rate;

        let fut = Self::poll_batch_result(configuration, poll_rate, result, tx.clone());

        // spawn a new async task so we can return the receiver immediately
        let handle = tokio::spawn(fut);

        // abort the pooling when the timeout is reached
        if let Some(timeout) = timeout {
            tokio::spawn(async move {
                tokio::time::sleep(timeout).await;
                handle.abort();
            });
        }

        rx
    }

    async fn poll_batch_result(
        configuration: Configuration,
        poll_rate: Duration,
        mut result: Result<BatchResult, NtropyError>,
        tx: Sender<Result<EnrichedTransaction, NtropyError>>,
    ) {
        let mut should_continue_pooling = true;

        while should_continue_pooling {
            match &result {
                Ok(ref batch) => {
                    should_continue_pooling = batch.status == Status::Started;

                    if let Some(results) = &batch.results {
                        if !results.is_empty() {
                            for transaction in results {
                                tx.send(
                                    map_enriched_transaction(transaction)
                                        .map_err(NtropyError::TransactionError),
                                )
                                .await
                                .expect("failed to send transaction, receiver was closed");
                            }
                        }
                    };

                    if should_continue_pooling {
                        // the batch is still processing, so we need to poll again
                        time::sleep(poll_rate).await;
                        result =
                            ntropy_server_period_api_period_v22_period_get_async_enrichment_result(
                                &configuration.clone(),
                                &batch.id.to_string(),
                            )
                            .await
                            .map_err(|e| e.into());
                    }
                }
                Err(e) => {
                    should_continue_pooling = false;
                    tx.send(Err(e.clone()))
                        .await
                        .expect("failed to send transaction, receiver was closed");
                }
            }
        }
    }
}
