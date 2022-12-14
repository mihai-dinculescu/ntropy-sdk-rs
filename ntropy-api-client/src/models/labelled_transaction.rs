/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// LabelledTransaction : A transaction for an account holder

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LabelledTransaction {
    /// The type of an account holder, may be one of 'consumer', 'business', 'freelance' or 'unknown'.
    #[serde(rename = "account_holder_type")]
    pub account_holder_type: AccountHolderType,
    /// The amount of the transaction
    #[serde(rename = "amount")]
    pub amount: f32,
    /// The direction of the transaction (incoming or outgoing)
    #[serde(rename = "entry_type")]
    pub entry_type: EntryType,
    /// The currency of the transaction in ISO 4217 format
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The description string of the transaction
    #[serde(rename = "description")]
    pub description: String,
    /// The country where the transaction was made in ISO 3166-2 format
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The Merchant Category Code of the merchant according to ISO 18245
    #[serde(rename = "mcc", skip_serializing_if = "Option::is_none")]
    pub mcc: Option<f32>,
    /// The ground truth label for this transaction.
    #[serde(rename = "label")]
    pub label: String,
}

impl LabelledTransaction {
    /// A transaction for an account holder
    pub fn new(
        account_holder_type: AccountHolderType,
        amount: f32,
        entry_type: EntryType,
        iso_currency_code: String,
        description: String,
        label: String,
    ) -> LabelledTransaction {
        LabelledTransaction {
            account_holder_type,
            amount,
            entry_type,
            iso_currency_code,
            description,
            country: None,
            mcc: None,
            label,
        }
    }
}

/// The type of an account holder, may be one of 'consumer', 'business', 'freelance' or 'unknown'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountHolderType {
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "freelance")]
    Freelance,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for AccountHolderType {
    fn default() -> AccountHolderType {
        Self::Consumer
    }
}
/// The direction of the transaction (incoming or outgoing)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntryType {
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "outgoing")]
    Outgoing,
}

impl Default for EntryType {
    fn default() -> EntryType {
        Self::Incoming
    }
}
