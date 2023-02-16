/*
 * Geins Management API
 *
 *  Geins Management API is an RESTful api to power your applications who manages your geins services. Geins provides an easy-to-use and scalable solution for managing all aspects of an online store, from product listings and customer information to order processing and payment transactions.   :::tip With this API, you can build custom applications and integrate with third-party systems, dashboards and other bussiness logic apps. :::    ## Getting started Once you have created an account, you can start using the Management API by creating an `API key`. You can create as many API keys as you need. Each `API key` is connected to a specific account so you can keep track of operations and manage keys. You can find your `API key` in the `geins merchant center`.   ### Fast track Use one of our [SDKs](https://docs.geins.io/docs/sdk/introduction) to get started quickly. The SDKs are available for the most popular programming languages and frameworks.  Or, if you prefer to just take it for a test run:  [![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553?action=collection%2Ffork&collection-url=entityId%3D25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553%26entityType%3Dcollection%26workspaceId%3Da2a179ce-158e-46b0-8d06-e9640f45112c)  ### Authentication Two authentication methods are supported:   - `API Key`   - `Basic Auth` 
 *
 * The version of the OpenAPI document: v1.7
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue : A parameter value for a product.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue {
    /// The unique identifier of this parameter value.
    #[serde(rename = "ParameterValueId", skip_serializing_if = "Option::is_none")]
    pub parameter_value_id: Option<i32>,
    /// The product id of the parameter.
    #[serde(rename = "ProductId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    /// The unique identifier of the parameter that this value belongs to.
    #[serde(rename = "ParameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<i32>,
    /// The non-localized name of the parameter.
    #[serde(rename = "ParameterName", skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// The unique identifier of the group that this parameter belongs to.
    #[serde(rename = "GroupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    /// The name of the group that this parameter belongs to.
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The type of parameter.
    #[serde(rename = "ParameterType", skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<ParameterType>,
    /// The identifying value of the parameter.
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The non-localized description of the parameter.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The localized descriptions of the parameter.
    #[serde(rename = "LocalizedDescriptions", skip_serializing_if = "Option::is_none")]
    pub localized_descriptions: Option<Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>>,
    /// The internal identifier of the parameter.
    #[serde(rename = "InternalIdentifier", skip_serializing_if = "Option::is_none")]
    pub internal_identifier: Option<String>,
}

impl ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue {
    /// A parameter value for a product.
    pub fn new() -> ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue {
        ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue {
            parameter_value_id: None,
            product_id: None,
            parameter_id: None,
            parameter_name: None,
            group_id: None,
            group_name: None,
            parameter_type: None,
            value: None,
            description: None,
            localized_descriptions: None,
            internal_identifier: None,
        }
    }
}

/// The type of parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ParameterType {
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
    #[serde(rename = "7")]
    Variant7,
}

impl Default for ParameterType {
    fn default() -> ParameterType {
        Self::Variant1
    }
}
