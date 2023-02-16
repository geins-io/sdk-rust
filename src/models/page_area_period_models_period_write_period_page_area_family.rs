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
pub struct PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This page area family has access to the following properties that can be used for filtering, when rendering itself.  <para>  The following properties are available:  SiteId,  LanguageId,  ProductId,  CategoryId,  BrandId,  InfoPageId,  DiscountCampaignNumber,  GenderId,  Sale,  UserTypeId  ActiveFrom,  ActiveTo  </para>
    #[serde(rename = "FilterableProperties", skip_serializing_if = "Option::is_none")]
    pub filterable_properties: Option<Vec<String>>,
    #[serde(rename = "Areas", skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<crate::models::PageAreaPeriodModelsPeriodWritePeriodPageArea>>,
}

impl PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily {
    pub fn new() -> PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily {
        PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily {
            id: None,
            name: None,
            filterable_properties: None,
            areas: None,
        }
    }
}


