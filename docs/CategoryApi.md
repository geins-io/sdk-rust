# \CategoryApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_category**](CategoryApi.md#create_category) | **POST** /API/Category | Create a new category
[**get_category_by_id**](CategoryApi.md#get_category_by_id) | **GET** /API/Category/{id} | Get a specific category
[**query_categories**](CategoryApi.md#query_categories) | **POST** /API/Category/Query | Query categories
[**update_category**](CategoryApi.md#update_category) | **PUT** /API/Category/{id} | Update a category



## create_category

> crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory create_category(category)
Create a new category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | [**CategoryPeriodModelsPeriodWritePeriodCategory**](CategoryPeriodModelsPeriodWritePeriodCategory.md) | The category to create. | [required] |

### Return type

[**crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory**](Envelope-Category.Models.Read.Category.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_category_by_id

> crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory get_category_by_id(id)
Get a specific category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the category to get. | [required] |

### Return type

[**crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory**](Envelope-Category.Models.Read.Category.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_categories

> Vec<crate::models::CategoryPeriodModelsPeriodReadPeriodCategory> query_categories(query)
Query categories

No response envelope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**CategoryPeriodModelsPeriodCategoryQuery**](CategoryPeriodModelsPeriodCategoryQuery.md) | The query to filter categories by. | [required] |

### Return type

[**Vec<crate::models::CategoryPeriodModelsPeriodReadPeriodCategory>**](Category.Models.Read.Category.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_category

> crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory update_category(id, category)
Update a category

Leaving out a property will ensure no changes are made to that property. Collection properties will delete and/or add as necessary to match the supplied data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the category to update. | [required] |
**category** | [**CategoryPeriodModelsPeriodWritePeriodCategory**](CategoryPeriodModelsPeriodWritePeriodCategory.md) | The category data to update. | [required] |

### Return type

[**crate::models::EnvelopeCategoryPeriodModelsPeriodReadPeriodCategory**](Envelope-Category.Models.Read.Category.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

