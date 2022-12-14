/*
 * Ntropy Transaction API
 *
 * The world's most powerful, multi-geo, multi-lingual transaction categorization API.
 *
 * The version of the OpenAPI document:
 * Contact: api@ntropy.com
 * Generated by: https://openapi-generator.tech
 */

/// AccountHolderType : The type of an account holder, may be one of 'consumer', 'business', 'freelance' or 'unknown'

/// The type of an account holder, may be one of 'consumer', 'business', 'freelance' or 'unknown'
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

impl ToString for AccountHolderType {
    fn to_string(&self) -> String {
        match self {
            Self::Consumer => String::from("consumer"),
            Self::Business => String::from("business"),
            Self::Freelance => String::from("freelance"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

impl Default for AccountHolderType {
    fn default() -> AccountHolderType {
        Self::Consumer
    }
}
