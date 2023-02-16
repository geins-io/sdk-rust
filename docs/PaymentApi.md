# \PaymentApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_payment_options**](PaymentApi.md#query_payment_options) | **POST** /API/Payment/Query | Query payment options



## query_payment_options

> Vec<crate::models::PaymentPeriodModelsPeriodPaymentOption> query_payment_options(query)
Query payment options

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**PaymentPeriodModelsPeriodPaymentOptionQuery**](PaymentPeriodModelsPeriodPaymentOptionQuery.md) | The payment query | [required] |

### Return type

[**Vec<crate::models::PaymentPeriodModelsPeriodPaymentOption>**](Payment.Models.PaymentOption.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

