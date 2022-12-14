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
pub struct MetricStatisticsValue {
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f32>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
}

impl MetricStatisticsValue {
    pub fn new() -> MetricStatisticsValue {
        MetricStatisticsValue {
            sum: None,
            count: None,
        }
    }
}
