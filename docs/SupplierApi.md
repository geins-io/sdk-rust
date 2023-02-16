# \SupplierApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_supplier**](SupplierApi.md#create_supplier) | **POST** /API/Supplier | Create a new supplier
[**get_supplier_by_id**](SupplierApi.md#get_supplier_by_id) | **GET** /API/Supplier/{id} | Get a specific supplier
[**query_suppliers**](SupplierApi.md#query_suppliers) | **POST** /API/Supplier/Query | Query suppliers
[**update_supplier**](SupplierApi.md#update_supplier) | **PUT** /API/Supplier/{id} | Updates a supplier



## create_supplier

> crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier create_supplier(supplier)
Create a new supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier** | [**SupplierPeriodModelsPeriodWritePeriodSupplier**](SupplierPeriodModelsPeriodWritePeriodSupplier.md) | The supplier to create. | [required] |

### Return type

[**crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier**](Envelope-Supplier.Models.Read.Supplier.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supplier_by_id

> crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier get_supplier_by_id(id)
Get a specific supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the supplier to get. | [required] |

### Return type

[**crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier**](Envelope-Supplier.Models.Read.Supplier.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_suppliers

> Vec<crate::models::SupplierPeriodModelsPeriodReadPeriodSupplier> query_suppliers(query)
Query suppliers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**SupplierPeriodModelsPeriodSupplierQuery**](SupplierPeriodModelsPeriodSupplierQuery.md) | The details of the query | [required] |

### Return type

[**Vec<crate::models::SupplierPeriodModelsPeriodReadPeriodSupplier>**](Supplier.Models.Read.Supplier.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supplier

> crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier update_supplier(id, supplier)
Updates a supplier

Leaving out a property will ensure no changes are made to that property.  Collection properties will delete and/or add as necessary to match the supplied data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the supplier to update. | [required] |
**supplier** | [**SupplierPeriodModelsPeriodWritePeriodSupplier**](SupplierPeriodModelsPeriodWritePeriodSupplier.md) | The supplier data to update. | [required] |

### Return type

[**crate::models::EnvelopeSupplierPeriodModelsPeriodReadPeriodSupplier**](Envelope-Supplier.Models.Read.Supplier.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

