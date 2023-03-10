/*
 * Geins Management API
 *
 *  Geins Management API is an RESTful api to power your applications who manages your geins services. Geins provides an easy-to-use and scalable solution for managing all aspects of an online store, from product listings and customer information to order processing and payment transactions.   :::tip With this API, you can build custom applications and integrate with third-party systems, dashboards and other bussiness logic apps. :::    ## Getting started Once you have created an account, you can start using the Management API by creating an `API key`. You can create as many API keys as you need. Each `API key` is connected to a specific account so you can keep track of operations and manage keys. You can find your `API key` in the `geins merchant center`.   ### Fast track Use one of our [SDKs](https://docs.geins.io/docs/sdk/introduction) to get started quickly. The SDKs are available for the most popular programming languages and frameworks.  Or, if you prefer to just take it for a test run:  [![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553?action=collection%2Ffork&collection-url=entityId%3D25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553%26entityType%3Dcollection%26workspaceId%3Da2a179ce-158e-46b0-8d06-e9640f45112c)  ### Authentication Two authentication methods are supported:   - `API Key`   - `Basic Auth` 
 *
 * The version of the OpenAPI document: v1.7
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderPeriodModelsPeriodOrderQuery : Data carrying class for order queries



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderPeriodModelsPeriodOrderQuery {
    /// Given a date, only orders updated after the provided date will be returned.
    #[serde(rename = "Updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// Comma separated list of statuses to filter on.
    #[serde(rename = "StatusList", skip_serializing_if = "Option::is_none")]
    pub status_list: Option<String>,
    /// Id of a market.
    #[serde(rename = "MarketId", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<i32>,
    /// Name of a payment method
    #[serde(rename = "PaymentName", skip_serializing_if = "Option::is_none")]
    pub payment_name: Option<String>,
    /// Id of a parcel group.
    #[serde(rename = "ParcelGroupId", skip_serializing_if = "Option::is_none")]
    pub parcel_group_id: Option<i32>,
    /// The ID of a customer
    #[serde(rename = "CustomerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i32>,
    /// The email of a customer
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Comma separated list of child-collections to also include in the query result.
    #[serde(rename = "Include", skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    /// This status can be used by external systems to change the status of an order. Such as failed or done.
    #[serde(rename = "ExternalOrderStatus", skip_serializing_if = "Option::is_none")]
    pub external_order_status: Option<i32>,
    /// If true, will combine all order rows that are part of a container into a single container row.
    #[serde(rename = "CombineProductContainerRows", skip_serializing_if = "Option::is_none")]
    pub combine_product_container_rows: Option<bool>,
    /// The packing place to get orders from.
    #[serde(rename = "PackingLocationId", skip_serializing_if = "Option::is_none")]
    pub packing_location_id: Option<i32>,
}

impl OrderPeriodModelsPeriodOrderQuery {
    /// Data carrying class for order queries
    pub fn new() -> OrderPeriodModelsPeriodOrderQuery {
        OrderPeriodModelsPeriodOrderQuery {
            updated: None,
            status_list: None,
            market_id: None,
            payment_name: None,
            parcel_group_id: None,
            customer_id: None,
            email: None,
            include: None,
            external_order_status: None,
            combine_product_container_rows: None,
            packing_location_id: None,
        }
    }
}


