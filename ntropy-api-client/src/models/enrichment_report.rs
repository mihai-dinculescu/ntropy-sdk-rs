/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// EnrichmentReport : An object describing expected enriched values for a transaction

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnrichmentReport {
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "merchant", skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,
    #[serde(rename = "merchant_id", skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "person", skip_serializing_if = "Option::is_none")]
    pub person: Option<String>,
    #[serde(rename = "chart_of_accounts", skip_serializing_if = "Option::is_none")]
    pub chart_of_accounts: Option<String>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(rename = "recurrence_group", skip_serializing_if = "Option::is_none")]
    pub recurrence_group: Option<Box<crate::models::RecurrenceGroup>>,
    #[serde(
        rename = "recurrence_group_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub recurrence_group_id: Option<String>,
    #[serde(rename = "mcc", skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Vec<i32>>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl EnrichmentReport {
    /// An object describing expected enriched values for a transaction
    pub fn new(transaction_id: String) -> EnrichmentReport {
        EnrichmentReport {
            transaction_id,
            logo: None,
            website: None,
            location: None,
            merchant: None,
            merchant_id: None,
            labels: None,
            person: None,
            chart_of_accounts: None,
            recurrence: None,
            recurrence_group: None,
            recurrence_group_id: None,
            mcc: None,
            notes: None,
        }
    }
}
