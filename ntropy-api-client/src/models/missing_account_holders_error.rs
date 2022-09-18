/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MissingAccountHoldersError {
    /// Message of the error
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "missingIds", skip_serializing_if = "Option::is_none")]
    pub missing_ids: Option<Vec<String>>,
}

impl MissingAccountHoldersError {
    pub fn new() -> MissingAccountHoldersError {
        MissingAccountHoldersError {
            detail: None,
            missing_ids: None,
        }
    }
}