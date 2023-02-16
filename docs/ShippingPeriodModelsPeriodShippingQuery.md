# ShippingPeriodModelsPeriodShippingQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site_id** | Option<**i32**> | The site ID the delivery options belong to. Required. | [optional]
**country_id** | Option<**i32**> | The country ID where the order should be shipped to. | [optional]
**shipping_id** | Option<**i32**> | Carismar Shipping Option ID | [optional]
**delivery_option_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unifaun Delivery Option ID | [optional]
**order** | Option<[**crate::models::OrderPeriodCheckoutOrder**](Order.CheckoutOrder.md)> |  | [optional]
**minimum_free_shipping_limit** | Option<**f64**> | The cart sum limit for free shipping - to be used for conditions in the delivery checkout portal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


