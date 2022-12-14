/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// BatchResult : Batch transaction enrichment status.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchResult {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results:
        Option<Vec<crate::models::NtropyServerApiV22EnrichTransactionsSync200ResponseInner>>,
}

impl BatchResult {
    /// Batch transaction enrichment status.
    pub fn new(status: Status, updated_at: String, id: uuid::Uuid) -> BatchResult {
        BatchResult {
            status,
            updated_at,
            progress: None,
            id,
            results: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::Started
    }
}
