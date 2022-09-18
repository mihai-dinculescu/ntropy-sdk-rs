use std::sync::Arc;

use reqwest::StatusCode;
use thiserror::Error;

use ntropy_api_client::apis::enrichment_api::{
    NtropyServerPeriodApiPeriodV22PeriodEnrichTransactionsAsyncError,
    NtropyServerPeriodApiPeriodV22PeriodGetAsyncEnrichmentResultError,
};
use ntropy_api_client::apis::Error as ApiClientError;
use ntropy_api_client::models::TransactionError;

/// Ntropy SDK Error.
#[derive(Error, Debug, Clone)]
pub enum NtropyError {
    /// Client IO Error.
    #[error("Client IO Error")]
    ClientIOError(#[from] Arc<std::io::Error>),
    /// Client Request Error.
    #[error("Client Request Error")]
    ClientRequestError(#[from] Arc<reqwest::Error>),
    /// Client Serialization Error.
    #[error("Client Serialization Error")]
    ClientSerializationError(#[from] Arc<serde_json::Error>),
    /// Ntropy Response Error.
    #[error("Response Error")]
    ResponseError { status: StatusCode, content: String },
    /// Ntropy Transaction Error.
    #[error("Transaction Error {}", .0.error)]
    TransactionError(TransactionError),
}

impl From<ApiClientError<NtropyServerPeriodApiPeriodV22PeriodEnrichTransactionsAsyncError>>
    for NtropyError
{
    fn from(
        e: ApiClientError<NtropyServerPeriodApiPeriodV22PeriodEnrichTransactionsAsyncError>,
    ) -> Self {
        match e {
            ApiClientError::ResponseError(e) => NtropyError::ResponseError {
                status: e.status,
                content: e.content,
            },
            ApiClientError::Io(e) => NtropyError::ClientIOError(Arc::new(e)),
            ApiClientError::Reqwest(e) => NtropyError::ClientRequestError(Arc::new(e)),
            ApiClientError::Serde(e) => NtropyError::ClientSerializationError(Arc::new(e)),
        }
    }
}

impl From<ApiClientError<NtropyServerPeriodApiPeriodV22PeriodGetAsyncEnrichmentResultError>>
    for NtropyError
{
    fn from(
        e: ApiClientError<NtropyServerPeriodApiPeriodV22PeriodGetAsyncEnrichmentResultError>,
    ) -> Self {
        match e {
            ApiClientError::ResponseError(e) => NtropyError::ResponseError {
                status: e.status,
                content: e.content,
            },
            ApiClientError::Io(e) => NtropyError::ClientIOError(Arc::new(e)),
            ApiClientError::Reqwest(e) => NtropyError::ClientRequestError(Arc::new(e)),
            ApiClientError::Serde(e) => NtropyError::ClientSerializationError(Arc::new(e)),
        }
    }
}
