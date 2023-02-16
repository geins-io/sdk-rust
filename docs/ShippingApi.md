# \ShippingApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_parcel_group**](ShippingApi.md#create_parcel_group) | **POST** /API/Shipping/ParcelGroup | Create a new parcel group
[**query_shipping_options**](ShippingApi.md#query_shipping_options) | **POST** /API/Shipping/Query | Query shipping options



## create_parcel_group

> crate::models::EnvelopeInt create_parcel_group(parcel_group_options)
Create a new parcel group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parcel_group_options** | [**ShippingPeriodModelsPeriodParcelGroupOptions**](ShippingPeriodModelsPeriodParcelGroupOptions.md) | Options for the new parcel group. | [required] |

### Return type

[**crate::models::EnvelopeInt**](Envelope-Int.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_shipping_options

> Vec<crate::models::ShippingPeriodModelsPeriodShippingOption> query_shipping_options(shipping_query)
Query shipping options

No response envelope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipping_query** | [**ShippingPeriodModelsPeriodShippingQuery**](ShippingPeriodModelsPeriodShippingQuery.md) | The query to filter shipping options by. | [required] |

### Return type

[**Vec<crate::models::ShippingPeriodModelsPeriodShippingOption>**](Shipping.Models.ShippingOption.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

