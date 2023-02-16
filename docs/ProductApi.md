# \ProductApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_availability_monitor**](ProductApi.md#add_availability_monitor) | **POST** /API/Product/MonitorAvailability | Add a product availability monitor
[**add_category_to_product**](ProductApi.md#add_category_to_product) | **PUT** /API/Product/{productId}/Category | Adds a category relation to a product
[**add_image_to_product**](ProductApi.md#add_image_to_product) | **PUT** /API/Product/{productId}/Image/{imageName} | Adds an image relation to a product
[**add_related_products_to_product**](ProductApi.md#add_related_products_to_product) | **PUT** /API/Product/{productId}/Related | Add related products to a product
[**batch_update_product_items**](ProductApi.md#batch_update_product_items) | **PUT** /API/Product/Items | Updates product items in batch
[**batch_update_stock_values**](ProductApi.md#batch_update_stock_values) | **PUT** /API/Product/Stock | Update stock values for multiple product items
[**create_product**](ProductApi.md#create_product) | **POST** /API/Product | Create a new product
[**create_product_items**](ProductApi.md#create_product_items) | **POST** /API/Product/{productId}/Item | Create a new product item
[**get_product_by_id**](ProductApi.md#get_product_by_id) | **GET** /API/Product/{productId} | Get a specific product
[**get_product_item_by_id**](ProductApi.md#get_product_item_by_id) | **GET** /API/Product/Item/{itemId} | Get a specific product item
[**link_related_products_by_relation_id**](ProductApi.md#link_related_products_by_relation_id) | **PUT** /API/Product/{productId}/Related/{relationTypeId} | Add related products to a product using a fixed relation type
[**list_all_product_items_paged**](ProductApi.md#list_all_product_items_paged) | **GET** /API/Product/Items/{page} | Get all product items with pagination
[**list_feeds**](ProductApi.md#list_feeds) | **GET** /API/Product/Feeds | Gets a list of all feeds
[**list_product_items**](ProductApi.md#list_product_items) | **GET** /API/Product/Items | Get all product items
[**list_product_relation_types**](ProductApi.md#list_product_relation_types) | **GET** /API/Product/RelationTypes | Gets a list of product relation types
[**query_products**](ProductApi.md#query_products) | **POST** /API/Product/Query | Query products
[**query_products_paged**](ProductApi.md#query_products_paged) | **POST** /API/Product/Query/{page} | Query products with pagination
[**query_stock**](ProductApi.md#query_stock) | **POST** /API/Product/Stock/Query | Query stock
[**update_product**](ProductApi.md#update_product) | **PUT** /API/Product/{productId} | Updates a product
[**update_product_item**](ProductApi.md#update_product_item) | **PUT** /API/Product/Item/{itemId} | Updates a product item



## add_availability_monitor

> crate::models::Envelope add_availability_monitor(model)
Add a product availability monitor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | [**ProductPeriodModelsPeriodMonitorSku**](ProductPeriodModelsPeriodMonitorSku.md) |  | [required] |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_category_to_product

> crate::models::Envelope add_category_to_product(product_id, product_category, product_id_type)
Adds a category relation to a product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to update. | [required] |
**product_category** | [**ProductPeriodModelsPeriodProductCategory**](ProductPeriodModelsPeriodProductCategory.md) | The category to relate the product to. | [required] |
**product_id_type** | Option<**i32**> | The type of product id supplied in {productId}. |  |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_image_to_product

> crate::models::Envelope add_image_to_product(product_id, image_name, is_primary_image, product_id_type)
Adds an image relation to a product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to update. | [required] |
**image_name** | **String** | The name of the image to set to product. | [required] |
**is_primary_image** | Option<**bool**> | Specifies whether {imageName} should be set as the primary image of the product. |  |
**product_id_type** | Option<**i32**> | The type of product id supplied in {productId}. |  |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_related_products_to_product

> crate::models::ProductPeriodModelsPeriodRelatedProductEnvelope add_related_products_to_product(product_id, related_products, product_id_type)
Add related products to a product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the main product to which the relations will be created | [required] |
**related_products** | [**Vec<crate::models::ProductPeriodModelsPeriodWritePeriodRelatedProduct>**](Product.Models.Write.RelatedProduct.md) | The list of related products to link with the main product. | [required] |
**product_id_type** | Option<**i32**> | The type of product id for {productId} and supplied in {relatedProducts}. |  |

### Return type

[**crate::models::ProductPeriodModelsPeriodRelatedProductEnvelope**](Product.Models.RelatedProductEnvelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_update_product_items

> crate::models::Envelope batch_update_product_items(product_items, product_item_id_type)
Updates product items in batch

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_items** | [**Vec<crate::models::ProductPeriodModelsPeriodWritePeriodProductItem>**](Product.Models.Write.ProductItem.md) | The product items to update | [required] |
**product_item_id_type** | Option<**i32**> | The type of product item id |  |

### Return type

[**crate::models::Envelope**](Envelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_update_stock_values

> crate::models::ProductPeriodModelsPeriodStockEnvelope batch_update_stock_values(product_item_stocks, product_item_id_type)
Update stock values for multiple product items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_item_stocks** | [**Vec<crate::models::ProductPeriodModelsPeriodWritePeriodProductItemStock>**](Product.Models.Write.ProductItemStock.md) | List of product item ids with new stock values. | [required] |
**product_item_id_type** | Option<**i32**> | The type of product item id supplied in {productItemStocks}. |  |

### Return type

[**crate::models::ProductPeriodModelsPeriodStockEnvelope**](Product.Models.StockEnvelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product

> crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct create_product(product, include)
Create a new product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | [**ProductPeriodModelsPeriodWritePeriodProduct**](ProductPeriodModelsPeriodWritePeriodProduct.md) | The product to create. | [required] |
**include** | Option<**String**> | Comma separated list of child-collections to also include with the created product. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct**](Envelope-Product.Models.Read.Product.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_product_items

> crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProductItem create_product_items(product_id, product_item, product_id_type)
Create a new product item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to create an item on. | [required] |
**product_item** | [**ProductPeriodModelsPeriodWritePeriodProductItem**](ProductPeriodModelsPeriodWritePeriodProductItem.md) | The product item to create. | [required] |
**product_id_type** | Option<**i32**> | The type of product id supplied in {productId}. |  |

### Return type

[**crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProductItem**](Envelope-Product.Models.Read.ProductItem.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_by_id

> crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct get_product_by_id(product_id, product_id_type, include)
Get a specific product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to get. | [required] |
**product_id_type** | Option<**i32**> | The type of product id supplied in {productId}. |  |
**include** | Option<**String**> | Comma separated list of child-collections to also include with the requested product. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct**](Envelope-Product.Models.Read.Product.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_item_by_id

> crate::models::ProductPeriodProductItemEnvelope get_product_item_by_id(item_id, product_item_id_type)
Get a specific product item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The id of the product item to get. | [required] |
**product_item_id_type** | Option<**i32**> | The type of product item id supplied in {itemId}. |  |

### Return type

[**crate::models::ProductPeriodProductItemEnvelope**](Product.ProductItemEnvelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_related_products_by_relation_id

> crate::models::ProductPeriodModelsPeriodRelatedProductEnvelope link_related_products_by_relation_id(product_id, relation_type_id, related_products, product_id_type)
Add related products to a product using a fixed relation type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the main product to which the relations will be created | [required] |
**relation_type_id** | **i32** | The relation type id that will apply to all related products in {relatedProducts} | [required] |
**related_products** | [**Vec<crate::models::ProductPeriodModelsPeriodWritePeriodRelatedProduct>**](Product.Models.Write.RelatedProduct.md) | The list of related products to link with the main product. | [required] |
**product_id_type** | Option<**i32**> | The type of product id for {productId} and supplied in {relatedProducts}. |  |

### Return type

[**crate::models::ProductPeriodModelsPeriodRelatedProductEnvelope**](Product.Models.RelatedProductEnvelope.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_product_items_paged

> crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProductItem list_all_product_items_paged(page)
Get all product items with pagination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** | The page to fetch | [required] |

### Return type

[**crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProductItem**](Envelope-List-Product.Models.Read.ProductItem.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_feeds

> crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodFeed list_feeds()
Gets a list of all feeds

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodFeed**](Envelope-List-Product.Models.Read.Feed.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_items

> Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductItem> list_product_items()
Get all product items

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductItem>**](Product.Models.Read.ProductItem.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_relation_types

> crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodRelationType list_product_relation_types()
Gets a list of product relation types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodRelationType**](Envelope-List-Product.Models.Read.RelationType.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_products

> crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProduct query_products(query, include)
Query products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**ProductPeriodModelsPeriodProductQuery**](ProductPeriodModelsPeriodProductQuery.md) | The details of the query | [required] |
**include** | Option<**String**> | Comma separated list of child-collections to also include with the queried products. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProduct**](Envelope-List-Product.Models.Read.Product.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_products_paged

> crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProduct query_products_paged(page, query, include)
Query products with pagination

The {Product.Models.ProductQuery.BatchId} property is mandatory when fetching a page other than the first page.  If no BatchId is provided for the first page, a new batch is created and the BatchId can be found in the {Envelope.PageResult} field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** | The page to fetch. To start a new batched query it is mandatory to send in page=1 | [required] |
**query** | [**ProductPeriodModelsPeriodProductQuery**](ProductPeriodModelsPeriodProductQuery.md) | The details of the query | [required] |
**include** | Option<**String**> | Comma separated list of child-collections to also include with the queried products. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeListProductPeriodModelsPeriodReadPeriodProduct**](Envelope-List-Product.Models.Read.Product.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_stock

> Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductItemStock> query_stock(product_item_ids)
Query stock

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_item_ids** | [**Vec<i32>**](i32.md) | The details of the query. | [required] |

### Return type

[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductItemStock>**](Product.Models.Read.ProductItemStock.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product

> crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct update_product(product_id, product, product_id_type, include)
Updates a product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | The id of the product to update. | [required] |
**product** | [**ProductPeriodModelsPeriodWritePeriodProduct**](ProductPeriodModelsPeriodWritePeriodProduct.md) | The product data to update. | [required] |
**product_id_type** | Option<**i32**> | The type of product id supplied in {productId}. |  |
**include** | Option<**String**> | Comma separated list of child-collections to also include with the updated product. See  {Product.Models.ProductIncludes} names for valid options. |  |

### Return type

[**crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProduct**](Envelope-Product.Models.Read.Product.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_item

> crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProductItem update_product_item(item_id, product_item, product_item_id_type)
Updates a product item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The id of the product item to update. | [required] |
**product_item** | [**ProductPeriodModelsPeriodWritePeriodProductItem**](ProductPeriodModelsPeriodWritePeriodProductItem.md) | The product item data to update. | [required] |
**product_item_id_type** | Option<**i32**> | The type of product item id supplied in {itemId}. |  |

### Return type

[**crate::models::EnvelopeProductPeriodModelsPeriodReadPeriodProductItem**](Envelope-Product.Models.Read.ProductItem.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

