/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// EnrichedTransaction : An enriched transaction

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnrichedTransaction {
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
    #[serde(rename = "chart_of_accounts")]
    pub chart_of_accounts: Vec<String>,
    #[serde(rename = "recurrence", skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
    #[serde(rename = "recurrence_group", skip_serializing_if = "Option::is_none")]
    pub recurrence_group: Option<Box<crate::models::RecurrenceGroup>>,
    #[serde(
        rename = "recurrence_group_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub recurrence_group_id: Option<String>,
    #[serde(rename = "mcc", skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Vec<i32>>,
    #[serde(rename = "person", skip_serializing_if = "Option::is_none")]
    pub person: Option<String>,
    #[serde(rename = "transaction_type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
}

impl EnrichedTransaction {
    /// An enriched transaction
    pub fn new(transaction_id: String, chart_of_accounts: Vec<String>) -> EnrichedTransaction {
        EnrichedTransaction {
            transaction_id,
            logo: None,
            website: None,
            location: None,
            merchant: None,
            merchant_id: None,
            labels: None,
            chart_of_accounts,
            recurrence: None,
            recurrence_group: None,
            recurrence_group_id: None,
            mcc: None,
            person: None,
            transaction_type: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Recurrence {
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "subscription")]
    Subscription,
    #[serde(rename = "one off")]
    OneOff,
    #[serde(rename = "repeating")]
    Repeating,
}

impl Default for Recurrence {
    fn default() -> Recurrence {
        Self::Recurring
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        Self::Business
    }
}
