/*
 * Geins Management API
 *
 *  Geins Management API is an RESTful api to power your applications who manages your geins services. Geins provides an easy-to-use and scalable solution for managing all aspects of an online store, from product listings and customer information to order processing and payment transactions.   :::tip With this API, you can build custom applications and integrate with third-party systems, dashboards and other bussiness logic apps. :::    ## Getting started Once you have created an account, you can start using the Management API by creating an `API key`. You can create as many API keys as you need. Each `API key` is connected to a specific account so you can keep track of operations and manage keys. You can find your `API key` in the `geins merchant center`.   ### Fast track Use one of our [SDKs](https://docs.geins.io/docs/sdk/introduction) to get started quickly. The SDKs are available for the most popular programming languages and frameworks.  Or, if you prefer to just take it for a test run:  [![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553?action=collection%2Ffork&collection-url=entityId%3D25895885-aaf6598f-1a7c-4949-85d7-ba846c42d553%26entityType%3Dcollection%26workspaceId%3Da2a179ce-158e-46b0-8d06-e9640f45112c)  ### Authentication Two authentication methods are supported:   - `API Key`   - `Basic Auth` 
 *
 * The version of the OpenAPI document: v1.7
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OrderPeriodModelsPeriodOrder : Data carrier for an order.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderPeriodModelsPeriodOrder {
    /// The unique identifier for this order.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The unique external identifier for this order.
    #[serde(rename = "ExternalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The social security number, or organisational number of the customer.
    #[serde(rename = "PersonalId", skip_serializing_if = "Option::is_none")]
    pub personal_id: Option<String>,
    /// The unique identifier of the customer placing this order.
    #[serde(rename = "CustomerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i32>,
    /// Customer type. Usually 1 for private customers and 2 for companies. This field is customer specific
    #[serde(rename = "CustomerTypeId", skip_serializing_if = "Option::is_none")]
    pub customer_type_id: Option<i32>,
    /// Datetime when the order was created.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Datetime when the order was last updated.
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The DateTime when the order was completed (delivered, payed).
    #[serde(rename = "CompletedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// The order status. Possbile values: cancelled, on-hold, inactive, refunded, partial,  pending-payment, out-of-stock, backorder, completed, pending.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ISO Currency code.
    #[serde(rename = "Currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The Currency Rate to SEK.
    #[serde(rename = "CurrencyRate", skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    /// The unique identifier for the market this order originates from.
    #[serde(rename = "MarketId", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<i32>,
    /// The market name. Usually this is the equal to the site name.
    #[serde(rename = "MarketName", skip_serializing_if = "Option::is_none")]
    pub market_name: Option<String>,
    /// Two-letter Language code.
    #[serde(rename = "Language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Order total.
    #[serde(rename = "OrderTotal", skip_serializing_if = "Option::is_none")]
    pub order_total: Option<f64>,
    /// Expected total sum to be paid after discount and balance.   <para>The value is usually taken directly from the payment provider and represents the actual reserved amount.</para><para>If this differs from OrderTotal, actions should be taken to ensure they match. This usually happens due to rounding.</para>
    #[serde(rename = "ExpectedSum", skip_serializing_if = "Option::is_none")]
    pub expected_sum: Option<f64>,
    /// Order VAT total.
    #[serde(rename = "VATTotal", skip_serializing_if = "Option::is_none")]
    pub vat_total: Option<f64>,
    /// Order value inc vat after discount but before balance
    #[serde(rename = "OrderValueIncVat", skip_serializing_if = "Option::is_none")]
    pub order_value_inc_vat: Option<f64>,
    /// Order value ex vat after discount but before balance
    #[serde(rename = "OrderValueExVat", skip_serializing_if = "Option::is_none")]
    pub order_value_ex_vat: Option<f64>,
    /// Item value inc vat excluding fees and discount
    #[serde(rename = "ItemValueIncVat", skip_serializing_if = "Option::is_none")]
    pub item_value_inc_vat: Option<f64>,
    /// Item value ex vat excluding fees and discount
    #[serde(rename = "ItemValueExVat", skip_serializing_if = "Option::is_none")]
    pub item_value_ex_vat: Option<f64>,
    /// Total discount inc vat.
    #[serde(rename = "Discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    /// Total discount ex vat.
    #[serde(rename = "DiscountExVat", skip_serializing_if = "Option::is_none")]
    pub discount_ex_vat: Option<f64>,
    /// The amount which was withdrawn from the customers balance inc vat.
    #[serde(rename = "FromBalance", skip_serializing_if = "Option::is_none")]
    pub from_balance: Option<f64>,
    /// Shipping fee inc vat.
    #[serde(rename = "ShippingFee", skip_serializing_if = "Option::is_none")]
    pub shipping_fee: Option<f64>,
    /// Shipping fee ex vat.
    #[serde(rename = "ShippingFeeExVat", skip_serializing_if = "Option::is_none")]
    pub shipping_fee_ex_vat: Option<f64>,
    /// Payment fee inc vat.
    #[serde(rename = "PaymentFee", skip_serializing_if = "Option::is_none")]
    pub payment_fee: Option<f64>,
    /// Payment fee ex vat.
    #[serde(rename = "PaymentFeeExVat", skip_serializing_if = "Option::is_none")]
    pub payment_fee_ex_vat: Option<f64>,
    /// Order message. Can contain instructions from customer or added details about the order.
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Internal order messages. Can contain internal details about the order.
    #[serde(rename = "OrderMessages", skip_serializing_if = "Option::is_none")]
    pub order_messages: Option<Vec<String>>,
    /// List of payment details <seealso cref=\"T:Order.Models.PaymentDetail\" />.
    #[serde(rename = "PaymentDetails", skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<Vec<crate::models::OrderPeriodModelsPeriodPaymentDetail>>,
    /// List of shipping details <seealso cref=\"T:Order.Models.ShippingDetail\" />.
    #[serde(rename = "ShippingDetails", skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<Vec<crate::models::OrderPeriodModelsPeriodShippingDetail>>,
    #[serde(rename = "ShippingAddress", skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<crate::models::OrderPeriodModelsPeriodAddress>>,
    #[serde(rename = "BillingAddress", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<crate::models::OrderPeriodModelsPeriodAddress>>,
    /// List of order rows
    #[serde(rename = "Rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<crate::models::OrderPeriodModelsPeriodOrderRow>>,
    /// List of order refunds <seealso cref=\"T:Order.Models.Refund\" />.
    #[serde(rename = "Refunds", skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<crate::models::OrderPeriodModelsPeriodRefund>>,
    /// Customer IP-number.
    #[serde(rename = "Ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Customer User Agent.
    #[serde(rename = "UserAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// Chosen service location.
    #[serde(rename = "ServiceLocation", skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    /// Campaign code applied to the order.
    #[serde(rename = "CampaignCode", skip_serializing_if = "Option::is_none")]
    pub campaign_code: Option<String>,
    /// The internal id of the applied campaign code.
    #[serde(rename = "CampaignCodeId", skip_serializing_if = "Option::is_none")]
    pub campaign_code_id: Option<i32>,
    /// General percent discount applied to the order.
    #[serde(rename = "Percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    /// The desired delivery date of the order.
    #[serde(rename = "DesiredDeliveryDate", skip_serializing_if = "Option::is_none")]
    pub desired_delivery_date: Option<String>,
    /// The gender of the customer. True = male, False = female, null = unknown.
    #[serde(rename = "Gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<bool>,
    /// The unique identifier for the cart from which this order originates.
    #[serde(rename = "CartId", skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<i32>,
    /// The session id for the from which this order originates.
    #[serde(rename = "SessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "ExternalOrderStatus", skip_serializing_if = "Option::is_none")]
    pub external_order_status: Option<ExternalOrderStatus>,
    /// The ids for the campaigns applied to this order (not rows)
    #[serde(rename = "CampaignIds", skip_serializing_if = "Option::is_none")]
    pub campaign_ids: Option<Vec<String>>,
    /// The names for the campaigns applied to this order (not rows)
    #[serde(rename = "CampaignNames", skip_serializing_if = "Option::is_none")]
    pub campaign_names: Option<Vec<String>>,
    /// The order meta data to store additional information about the order. Eg. customer specific shipping data to include for nShift checkout (former UDC) shipments
    #[serde(rename = "MetaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<::std::collections::HashMap<String, String>>,
    /// The public id of this order.
    #[serde(rename = "PublicId", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<uuid::Uuid>,
}

impl OrderPeriodModelsPeriodOrder {
    /// Data carrier for an order.
    pub fn new() -> OrderPeriodModelsPeriodOrder {
        OrderPeriodModelsPeriodOrder {
            id: None,
            external_id: None,
            personal_id: None,
            customer_id: None,
            customer_type_id: None,
            created_at: None,
            updated_at: None,
            completed_at: None,
            status: None,
            currency: None,
            currency_rate: None,
            market_id: None,
            market_name: None,
            language: None,
            order_total: None,
            expected_sum: None,
            vat_total: None,
            order_value_inc_vat: None,
            order_value_ex_vat: None,
            item_value_inc_vat: None,
            item_value_ex_vat: None,
            discount: None,
            discount_ex_vat: None,
            from_balance: None,
            shipping_fee: None,
            shipping_fee_ex_vat: None,
            payment_fee: None,
            payment_fee_ex_vat: None,
            message: None,
            order_messages: None,
            payment_details: None,
            shipping_details: None,
            shipping_address: None,
            billing_address: None,
            rows: None,
            refunds: None,
            ip: None,
            user_agent: None,
            service_location: None,
            campaign_code: None,
            campaign_code_id: None,
            percent: None,
            desired_delivery_date: None,
            gender: None,
            cart_id: None,
            session_id: None,
            external_order_status: None,
            campaign_ids: None,
            campaign_names: None,
            meta_data: None,
            public_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExternalOrderStatus {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "20")]
    Variant20,
    #[serde(rename = "30")]
    Variant30,
    #[serde(rename = "40")]
    Variant40,
}

impl Default for ExternalOrderStatus {
    fn default() -> ExternalOrderStatus {
        Self::Variant0
    }
}

