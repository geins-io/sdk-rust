# \PageAreaApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_a_page_area**](PageAreaApi.md#create_or_update_a_page_area) | **POST** /API/PageArea | Create or update a page area
[**create_or_update_page_area_family**](PageAreaApi.md#create_or_update_page_area_family) | **POST** /API/PageAreaFamily | Create or update a page area family
[**get_page_area**](PageAreaApi.md#get_page_area) | **GET** /API/PageArea/{name} | Get a specific page area
[**get_page_area_family**](PageAreaApi.md#get_page_area_family) | **GET** /API/PageAreaFamily/{familyId} | Get a specific page area family
[**list_page_area_families**](PageAreaApi.md#list_page_area_families) | **GET** /API/PageAreaFamily/List | Gets a list of all page area families, including nested data



## create_or_update_a_page_area

> crate::models::EnvelopePageAreaPeriodModelsPeriodReadPeriodPageArea create_or_update_a_page_area(area)
Create or update a page area

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**area** | [**PageAreaPeriodModelsPeriodWritePeriodPageArea**](PageAreaPeriodModelsPeriodWritePeriodPageArea.md) | The area model to create or update. | [required] |

### Return type

[**crate::models::EnvelopePageAreaPeriodModelsPeriodReadPeriodPageArea**](Envelope-PageArea.Models.Read.PageArea.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_page_area_family

> crate::models::EnvelopePageAreaPeriodModelsPeriodReadPeriodPageAreaFamily create_or_update_page_area_family(family)
Create or update a page area family

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family** | [**PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily**](PageAreaPeriodModelsPeriodWritePeriodPageAreaFamily.md) | The family model to create or update. | [required] |

### Return type

[**crate::models::EnvelopePageAreaPeriodModelsPeriodReadPeriodPageAreaFamily**](Envelope-PageArea.Models.Read.PageAreaFamily.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_page_area

> crate::models::PageAreaPeriodModelsPeriodReadPeriodPageArea get_page_area(name)
Get a specific page area

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the page area to get. | [required] |

### Return type

[**crate::models::PageAreaPeriodModelsPeriodReadPeriodPageArea**](PageArea.Models.Read.PageArea.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_page_area_family

> crate::models::PageAreaPeriodModelsPeriodReadPeriodPageAreaFamily get_page_area_family(family_id)
Get a specific page area family

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family_id** | **i32** | The id of the page area family to get. | [required] |

### Return type

[**crate::models::PageAreaPeriodModelsPeriodReadPeriodPageAreaFamily**](PageArea.Models.Read.PageAreaFamily.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_page_area_families

> Vec<crate::models::PageAreaPeriodModelsPeriodReadPeriodPageAreaFamily> list_page_area_families()
Gets a list of all page area families, including nested data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PageAreaPeriodModelsPeriodReadPeriodPageAreaFamily>**](PageArea.Models.Read.PageAreaFamily.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

