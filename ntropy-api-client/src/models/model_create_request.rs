/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// ModelCreateRequest : Object for creating a custom model

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ModelCreateRequest {
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::LabelledTransaction>,
}

impl ModelCreateRequest {
    /// Object for creating a custom model
    pub fn new(transactions: Vec<crate::models::LabelledTransaction>) -> ModelCreateRequest {
        ModelCreateRequest { transactions }
    }
}
