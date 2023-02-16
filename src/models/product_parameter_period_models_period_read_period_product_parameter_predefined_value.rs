/*
 * Geins Management API
 *
 *  Geins Management API is an RESTful api to power your applications who manages your geins services. Geins provides an easy-to-use and scalable solution for managing all aspects of an online store, from product listings and customer information to order processing and payment transactions.   :::tip With this API, you can build custom applications and integrate with third-party systems, dashboards and other bussiness logic apps. :::    ## Getting started Once you have created an account, you can start using the Management API by creating an `API key`. You can create as many API keys as you need. Each `API key` is connected to a specific account so you can keep track of operations and manage keys. You can find your `API key` in the `geins merchant center`.   ### Fast track Use one of our [SDKs](https://docs.geins.io/docs/sdk/introduction) to get started quickly. The SDKs are available for the most popular programming languages and frameworks.  Or, if you prefer to just take it for a test run:  [![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553?action=collection%2Ffork&collection-url=entityId%3D25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553%26entityType%3Dcollection%26workspaceId%3Da2a179ce-158e-46b0-8d06-e9640f45112c)  ### Authentication Two authentication methods are supported:   - `API Key`   - `Basic Auth` 
 *
 * The version of the OpenAPI document: v1.7
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue : A predefined value for a product parameter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue {
    /// The unique identifier for the parameter.
    #[serde(rename = "ParameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<i32>,
    /// The predefined value id of the parameter.
    #[serde(rename = "PredefinedValueId", skip_serializing_if = "Option::is_none")]
    pub predefined_value_id: Option<i32>,
    /// The non-localized predefined value name of the parameter.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The localized predefined value names of the parameter.
    #[serde(rename = "LocalizedNames", skip_serializing_if = "Option::is_none")]
    pub localized_names: Option<Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>>,
}

impl ProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue {
    /// A predefined value for a product parameter.
    pub fn new() -> ProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue {
        ProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue {
            parameter_id: None,
            predefined_value_id: None,
            name: None,
            localized_names: None,
        }
    }
}


