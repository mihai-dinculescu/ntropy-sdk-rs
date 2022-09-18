/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// Model : Model that has been trained.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(
        rename = "account_holder_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_type: Option<String>,
}

impl Model {
    /// Model that has been trained.
    pub fn new(name: String) -> Model {
        Model {
            name,
            created_at: None,
            account_holder_type: None,
        }
    }
}
