/*
 * Geins Management API
 *
 *  Geins Management API is an RESTful api to power your applications who manages your geins services. Geins provides an easy-to-use and scalable solution for managing all aspects of an online store, from product listings and customer information to order processing and payment transactions.   :::tip With this API, you can build custom applications and integrate with third-party systems, dashboards and other bussiness logic apps. :::    ## Getting started Once you have created an account, you can start using the Management API by creating an `API key`. You can create as many API keys as you need. Each `API key` is connected to a specific account so you can keep track of operations and manage keys. You can find your `API key` in the `geins merchant center`.   ### Fast track Use one of our [SDKs](https://docs.geins.io/docs/sdk/introduction) to get started quickly. The SDKs are available for the most popular programming languages and frameworks.  Or, if you prefer to just take it for a test run:  [![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553?action=collection%2Ffork&collection-url=entityId%3D25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553%26entityType%3Dcollection%26workspaceId%3Da2a179ce-158e-46b0-8d06-e9640f45112c)  ### Authentication Two authentication methods are supported:   - `API Key`   - `Basic Auth` 
 *
 * The version of the OpenAPI document: v1.7
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WidgetRestrictionSetupPeriodWidgetRestrictionConfiguration {
    #[serde(rename = "ForcedResponsiveMode", skip_serializing_if = "Option::is_none")]
    pub forced_responsive_mode: Option<ForcedResponsiveMode>,
    #[serde(rename = "AllowedSizes", skip_serializing_if = "Option::is_none")]
    pub allowed_sizes: Option<Vec<AllowedSizes>>,
}

impl WidgetRestrictionSetupPeriodWidgetRestrictionConfiguration {
    pub fn new() -> WidgetRestrictionSetupPeriodWidgetRestrictionConfiguration {
        WidgetRestrictionSetupPeriodWidgetRestrictionConfiguration {
            forced_responsive_mode: None,
            allowed_sizes: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ForcedResponsiveMode {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for ForcedResponsiveMode {
    fn default() -> ForcedResponsiveMode {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowedSizes {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "6")]
    Variant6,
}

impl Default for AllowedSizes {
    fn default() -> AllowedSizes {
        Self::Variant1
    }
}

