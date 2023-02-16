# \MarketApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_market_by_id**](MarketApi.md#get_market_by_id) | **GET** /API/Market/{marketId} | Get a specific market
[**list_markets**](MarketApi.md#list_markets) | **GET** /API/Market/List | Gets a list of all markets



## get_market_by_id

> crate::models::EnvelopeMarketPeriodModelsPeriodMarket get_market_by_id(market_id, market_id_type)
Get a specific market

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_id** | **String** | The id of the market to get. | [required] |
**market_id_type** | Option<**i32**> | The type of market id supplied. See {Market.Models.MarketIdType} for valid options. |  |

### Return type

[**crate::models::EnvelopeMarketPeriodModelsPeriodMarket**](Envelope-Market.Models.Market.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_markets

> crate::models::MarketPeriodModelsPeriodMarket list_markets()
Gets a list of all markets

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MarketPeriodModelsPeriodMarket**](Market.Models.Market.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

