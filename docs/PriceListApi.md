# \PriceListApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_price_lists**](PriceListApi.md#list_price_lists) | **GET** /API/PriceList/List | Get all price list definitions
[**update_pricelist_prices**](PriceListApi.md#update_pricelist_prices) | **PUT** /API/PriceList/Price | Updates price list prices



## list_price_lists

> Vec<crate::models::PriceListPeriodModelsPeriodPriceList> list_price_lists()
Get all price list definitions

- Prices on campaign price lists (id: xxxxxx2) can not be updated. Any such entries will be ignored.  - ID for Ordinary, Sale and Campaign price lists starts from 1000000.   The ID is calculated by this formula, Market ID * 1000000 + Type of price list (Ordinary=0, Sale=1, Capaign=2)  So :  Ordinary price list for market with ID 1 has ID = 1000000  Sale price list for market with ID 1 has ID = 1000001  Campaign price list for market with ID 1 has ID = 1000002  Ordinary price list for market with ID 2 has ID = 2000000  And so on ...

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PriceListPeriodModelsPeriodPriceList>**](PriceList.Models.PriceList.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pricelist_prices

> crate::models::PriceListPeriodModelsPeriodPriceListPriceResponse update_pricelist_prices(price_list_prices, product_id_type, prices_inc_vat)
Updates price list prices

- Prices on campaign price lists (id: xxxxxx2) can not be updated. Any such entries will be ignored.  - ID for Ordinary, Sale and Campaign price lists starts from 1000000.   The ID is calculated by this formula, Market ID * 1000000 + Type of price list (Ordinary=0, Sale=1, Capaign=2)  So :  Ordinary price list for market with ID 1 has ID = 1000000  Sale price list for market with ID 1 has ID = 1000001  Campaign price list for market with ID 1 has ID = 1000002  Ordinary price list for market with ID 2 has ID = 2000000  And so on ...

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price_list_prices** | [**Vec<crate::models::PriceListPeriodModelsPeriodWritePeriodPriceListPrice>**](PriceList.Models.Write.PriceListPrice.md) | List of new price list prices. | [required] |
**product_id_type** | Option<**i32**> | The type of product id supplied in {priceListPrices} |  |
**prices_inc_vat** | Option<**bool**> | Set to true if prices in {priceListPrices} includes VAT. Leave blank or set to false if they exclude VAT. |  |

### Return type

[**crate::models::PriceListPeriodModelsPeriodPriceListPriceResponse**](PriceList.Models.PriceListPriceResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

