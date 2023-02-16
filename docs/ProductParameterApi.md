# \ProductParameterApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_replace_product_parameter_values**](ProductParameterApi.md#batch_replace_product_parameter_values) | **POST** /API/ProductParameter/Values | Replace multiple product parameter values
[**batch_update_product_parameter_values**](ProductParameterApi.md#batch_update_product_parameter_values) | **PUT** /API/ProductParameter/Values | Update multiple product parameter values
[**create_or_update_product_parameter_value**](ProductParameterApi.md#create_or_update_product_parameter_value) | **POST** /API/ProductParameter/Value | Create or update a new product parameter value
[**create_product_parameter**](ProductParameterApi.md#create_product_parameter) | **POST** /API/ProductParameter | Create a new product parameter
[**create_product_parameter_group**](ProductParameterApi.md#create_product_parameter_group) | **POST** /API/ProductParameter/Group | Create a new product parameter group
[**create_product_parameter_predefined_value**](ProductParameterApi.md#create_product_parameter_predefined_value) | **POST** /API/ProductParameter/PredefinedValue | Create a new predefined value for a product parameter
[**get_product_parameter_by_id**](ProductParameterApi.md#get_product_parameter_by_id) | **GET** /API/ProductParameter/{id} | Get a specific product parameter
[**get_product_parameter_group_by_id**](ProductParameterApi.md#get_product_parameter_group_by_id) | **GET** /API/ProductParameter/Group/{id} | Get a specific product parameter group
[**get_product_parameter_predefined_value**](ProductParameterApi.md#get_product_parameter_predefined_value) | **GET** /API/ProductParameter/PredefinedValue/{id} | Get a specific predefined value for a product parameter
[**get_product_parameter_value**](ProductParameterApi.md#get_product_parameter_value) | **GET** /API/ProductParameter/Value/{id} | Get a specific product parameter value
[**update_product_parameter**](ProductParameterApi.md#update_product_parameter) | **PUT** /API/ProductParameter/{id} | Updates a product parameter
[**update_product_parameter_group**](ProductParameterApi.md#update_product_parameter_group) | **PUT** /API/ProductParameter/Group/{id} | Update a product parameter group



## batch_replace_product_parameter_values

> crate::models::Envelope batch_replace_product_parameter_values(product_parameter_batch)
Replace multiple product parameter values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter_batch** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterValueBatch**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterValueBatch.md) | The product parameter values to replace. | [required] |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_update_product_parameter_values

> crate::models::Envelope batch_update_product_parameter_values(product_parameter_batch)
Update multiple product parameter values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter_batch** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterValueBatch**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterValueBatch.md) | The product parameter values update. | [required] |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_product_parameter_value

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue create_or_update_product_parameter_value(product_parameter_value)
Create or update a new product parameter value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter_value** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterValue**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterValue.md) | The product parameter value to create or update. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue**](Envelope-ProductParameter.Models.Read.ProductParameterValue.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product_parameter

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter create_product_parameter(product_parameter)
Create a new product parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameter**](ProductParameterPeriodModelsPeriodWritePeriodProductParameter.md) | The product parameter to create. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter**](Envelope-ProductParameter.Models.Read.ProductParameter.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product_parameter_group

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup create_product_parameter_group(product_parameter_group)
Create a new product parameter group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter_group** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterGroup**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterGroup.md) | The product parameter group to create. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup**](Envelope-ProductParameter.Models.Read.ProductParameterGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product_parameter_predefined_value

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue create_product_parameter_predefined_value(product_parameter_predefined_value)
Create a new predefined value for a product parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_parameter_predefined_value** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterPredefinedValue**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterPredefinedValue.md) | The predefined value to create. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterPredefinedValue**](Envelope-ProductParameter.Models.Read.ProductParameterPredefinedValue.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_parameter_by_id

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter get_product_parameter_by_id(id)
Get a specific product parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the product parameter to get. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter**](Envelope-ProductParameter.Models.Read.ProductParameter.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_parameter_group_by_id

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup get_product_parameter_group_by_id(id)
Get a specific product parameter group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the product parameter group to get. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup**](Envelope-ProductParameter.Models.Read.ProductParameterGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_parameter_predefined_value

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue get_product_parameter_predefined_value(id)
Get a specific predefined value for a product parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the predefined value to get. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue**](Envelope-ProductParameter.Models.Read.ProductParameterValue.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_parameter_value

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue get_product_parameter_value(id, predefined_value_id)
Get a specific product parameter value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the product parameter value to get. | [required] |
**predefined_value_id** | Option<**String**> | The predefined value id of the product parameter value to get. Only applicable for {ProductParameter.Models.ProductParameterType.Multi}. |  |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterValue**](Envelope-ProductParameter.Models.Read.ProductParameterValue.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_parameter

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter update_product_parameter(id, product_parameter)
Updates a product parameter

Leaving out a property will ensure no changes are made to that property. Collection properties will delete and/or add as necessary to match the supplied data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the product parameter to update. | [required] |
**product_parameter** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameter**](ProductParameterPeriodModelsPeriodWritePeriodProductParameter.md) | The product parameter data to update. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameter**](Envelope-ProductParameter.Models.Read.ProductParameter.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_parameter_group

> crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup update_product_parameter_group(id, product_parameter_group)
Update a product parameter group

Leaving out a property will ensure no changes are made to that property. Collection properties will delete and/or add as necessary to match the supplied data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The id of the product parameter group to update. | [required] |
**product_parameter_group** | [**ProductParameterPeriodModelsPeriodWritePeriodProductParameterGroup**](ProductParameterPeriodModelsPeriodWritePeriodProductParameterGroup.md) | The product parameter group data to update. | [required] |

### Return type

[**crate::models::EnvelopeProductParameterPeriodModelsPeriodReadPeriodProductParameterGroup**](Envelope-ProductParameter.Models.Read.ProductParameterGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

