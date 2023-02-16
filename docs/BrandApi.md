# \BrandApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_brand**](BrandApi.md#create_brand) | **POST** /API/Brand | Create a new brand
[**get_brand_by_id**](BrandApi.md#get_brand_by_id) | **GET** /API/Brand/{id} | Get a specific brand
[**query_brands**](BrandApi.md#query_brands) | **POST** /API/Brand/Query | Query brands
[**update_brand**](BrandApi.md#update_brand) | **PUT** /API/Brand/{id} | Updates a brand



## create_brand

> crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand create_brand(brand)
Create a new brand

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brand** | [**BrandPeriodModelsPeriodWritePeriodBrand**](BrandPeriodModelsPeriodWritePeriodBrand.md) | The brand to create. | [required] |

### Return type

[**crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand**](Envelope-Brand.Models.Read.Brand.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_brand_by_id

> crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand get_brand_by_id(id)
Get a specific brand

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the brand to get. | [required] |

### Return type

[**crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand**](Envelope-Brand.Models.Read.Brand.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_brands

> Vec<crate::models::BrandPeriodModelsPeriodReadPeriodBrand> query_brands(query)
Query brands

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**BrandPeriodModelsPeriodBrandQuery**](BrandPeriodModelsPeriodBrandQuery.md) | The details of the query. | [required] |

### Return type

[**Vec<crate::models::BrandPeriodModelsPeriodReadPeriodBrand>**](Brand.Models.Read.Brand.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_brand

> crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand update_brand(id, brand)
Updates a brand

Leaving out a property will ensure no changes are made to that property.  Collection properties will delete and/or add as necessary to match the supplied data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the brand to update. | [required] |
**brand** | [**BrandPeriodModelsPeriodWritePeriodBrand**](BrandPeriodModelsPeriodWritePeriodBrand.md) | The brand data to update. | [required] |

### Return type

[**crate::models::EnvelopeBrandPeriodModelsPeriodReadPeriodBrand**](Envelope-Brand.Models.Read.Brand.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

