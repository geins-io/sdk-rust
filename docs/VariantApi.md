# \VariantApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_product_to_variant_group**](VariantApi.md#add_product_to_variant_group) | **PUT** /API/VariantGroup/{groupId}/{productId} | Adds a product to an existing group
[**add_product_to_variant_group_by_product_id**](VariantApi.md#add_product_to_variant_group_by_product_id) | **PUT** /API/Variant/{productId1}/{productId2} | Adds a product to an existing group
[**create_variant_group**](VariantApi.md#create_variant_group) | **POST** /API/VariantGroup | Create a new variant group
[**create_variant_group_with_product**](VariantApi.md#create_variant_group_with_product) | **POST** /API/Variant/{productId}/VariantGroup | Create a new group for the provided product id
[**delete_variant_group**](VariantApi.md#delete_variant_group) | **DELETE** /API/VariantGroup/{groupId} | Delete an entire variant group
[**delete_variant_group_by_product_id**](VariantApi.md#delete_variant_group_by_product_id) | **DELETE** /API/Variant/{productId}/VariantGroup | Delete an entire variant group
[**get_variant_group**](VariantApi.md#get_variant_group) | **GET** /API/VariantGroup/{groupId} | Get a specific variant group
[**get_variant_group_by_product_id**](VariantApi.md#get_variant_group_by_product_id) | **GET** /API/Variant/{productId}/VariantGroup | Get the variant group for the provided id
[**get_variant_labels**](VariantApi.md#get_variant_labels) | **GET** /API/Variant/Labels | Get all valid variant labels
[**remove_product_from_variant_group**](VariantApi.md#remove_product_from_variant_group) | **DELETE** /API/Variant/{productId} | Remove a product from its variant group
[**update_variant**](VariantApi.md#update_variant) | **PUT** /API/Variant/{productId} | Adds the variant details for the product with the provided ID
[**update_variant_group**](VariantApi.md#update_variant_group) | **PUT** /API/VariantGroup/{groupId} | Updates the settings of a group



## add_product_to_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup add_product_to_variant_group(group_id, product_id, variant, product_id_type, include)
Adds a product to an existing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | The ID of the group to which a product should be added | [required] |
**product_id** | **String** | The ID of the product to be added to the target group. | [required] |
**variant** | [**Vec<crate::models::VariantPeriodModelsPeriodWritePeriodVariant>**](Variant.Models.Write.Variant.md) | The variant details. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_product_to_variant_group_by_product_id

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup add_product_to_variant_group_by_product_id(product_id1, product_id2, product_id_type, include)
Adds a product to an existing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id1** | **String** | The ID of a product belonging to the target group. | [required] |
**product_id2** | **String** | The ID of the product to be added to the target group. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup create_variant_group(variant_group, include)
Create a new variant group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant_group** | [**VariantPeriodModelsPeriodWritePeriodVariantGroup**](VariantPeriodModelsPeriodWritePeriodVariantGroup.md) | The settings for the new group. | [required] |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_variant_group_with_product

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup create_variant_group_with_product(product_id, variant_group, product_id_type, include)
Create a new group for the provided product id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product, for which to create a group. | [required] |
**variant_group** | [**VariantPeriodModelsPeriodWritePeriodVariantGroup**](VariantPeriodModelsPeriodWritePeriodVariantGroup.md) | The settings for the new group. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup delete_variant_group(group_id)
Delete an entire variant group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | The ID of the group to delete | [required] |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_variant_group_by_product_id

> crate::models::Envelope delete_variant_group_by_product_id(product_id, product_id_type)
Delete an entire variant group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of a product that belongs to the group to remove. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup get_variant_group(group_id, include)
Get a specific variant group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | The ID of the group to get | [required] |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variant_group_by_product_id

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup get_variant_group_by_product_id(product_id, product_id_type, include)
Get the variant group for the provided id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product, for which to get the group. | [required] |
**product_id_type** | Option<**i32**> | The type of id. |  |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_variant_labels

> crate::models::EnvelopeString get_variant_labels()
Get all valid variant labels

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EnvelopeString**](Envelope-String.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_product_from_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup remove_product_from_variant_group(product_id, product_id_type, include)
Remove a product from its variant group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to remove from group. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_variant

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariant update_variant(product_id, variant, product_id_type)
Adds the variant details for the product with the provided ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The ID of the product for which to update the variant details. | [required] |
**variant** | [**Vec<crate::models::VariantPeriodModelsPeriodWritePeriodVariant>**](Variant.Models.Write.Variant.md) | The variant details. | [required] |
**product_id_type** | Option<**i32**> | The type of product id provided. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariant**](Envelope-Variant.Models.Read.Variant-.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_variant_group

> crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup update_variant_group(group_id, variant_group, include)
Updates the settings of a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | The ID of the group to update | [required] |
**variant_group** | [**VariantPeriodModelsPeriodWritePeriodVariantGroup**](VariantPeriodModelsPeriodWritePeriodVariantGroup.md) | The new settings for the group | [required] |
**include** | Option<**String**> | Comma separated list of product child collections to also include with the variant group. Set to \"product\" to only include basic product data and to null (skip) to not include any product data. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeVariantPeriodModelsPeriodReadPeriodVariantGroup**](Envelope-Variant.Models.Read.VariantGroup.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

