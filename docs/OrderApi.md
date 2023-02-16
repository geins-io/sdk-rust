# \OrderApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_comment_to_order**](OrderApi.md#add_comment_to_order) | **POST** /API/Order/{id}/Comment | Adds a comment to the order
[**create_order**](OrderApi.md#create_order) | **POST** /API/Order | Post a new order
[**create_order_id**](OrderApi.md#create_order_id) | **POST** /API/Order/Id | Create a new order id
[**delete_order**](OrderApi.md#delete_order) | **DELETE** /API/Order/{id} | Deletes or deactivates an order
[**get_capture_by_id**](OrderApi.md#get_capture_by_id) | **GET** /API/Order/Capture/{captureId} | Get Capture by Id
[**get_order_by_id**](OrderApi.md#get_order_by_id) | **GET** /API/Order/{id}/{include} | Get an instance of a order
[**get_order_statuses**](OrderApi.md#get_order_statuses) | **GET** /API/Order/Statuses | Get a list of available order statuses
[**get_refund_by_id**](OrderApi.md#get_refund_by_id) | **GET** /API/Order/Refund/{refundId} | Get Refund by Id
[**partial_update_of_order**](OrderApi.md#partial_update_of_order) | **PATCH** /API/Order/{id} | Partial update of an order
[**query_orders**](OrderApi.md#query_orders) | **POST** /API/Order/Query | Query the order repository
[**set_capture_as_processed**](OrderApi.md#set_capture_as_processed) | **POST** /API/Order/Capture/SetAsProcessed | Set a capture as processed (= captured)
[**set_payment_as_payed**](OrderApi.md#set_payment_as_payed) | **POST** /API/Order/PaymentDetail/{paymentDetailId}/SetAsPayed | Set Payment Detail as payed
[**set_refund_as_processed**](OrderApi.md#set_refund_as_processed) | **POST** /API/Order/Refund/SetAsProcessed | Set a refund as processed (= settled)
[**update_order_status**](OrderApi.md#update_order_status) | **POST** /API/Order/{id}/Status/{status}/{transactionId}/{secondaryTransactionId} | Update order status
[**update_transaction_data**](OrderApi.md#update_transaction_data) | **POST** /API/Order/{id}/TransactionData | Updates transaction data on an order
[**validate_order**](OrderApi.md#validate_order) | **POST** /API/Order/ValidateCreation | Validates order data for order creation.



## add_comment_to_order

> serde_json::Value add_comment_to_order(id, order_comment)
Adds a comment to the order

This add to (not replace) any previous comments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Order ID. | [required] |
**order_comment** | [**ApiPeriodOrderPeriodOrderComment**](ApiPeriodOrderPeriodOrderComment.md) | The comment | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order

> crate::models::EnvelopeInt create_order(order)
Post a new order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | [**OrderPeriodModelsPeriodOrder**](OrderPeriodModelsPeriodOrder.md) | The order object. | [required] |

### Return type

[**crate::models::EnvelopeInt**](Envelope-Int.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_id

> crate::models::EnvelopeInt create_order_id()
Create a new order id

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EnvelopeInt**](Envelope-Int.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_order

> serde_json::Value delete_order(id, operation)
Deletes or deactivates an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the order to delete. | [required] |
**operation** | **i32** | The method of deletion desired. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_capture_by_id

> crate::models::OrderPeriodCapture get_capture_by_id(capture_id)
Get Capture by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**capture_id** | **uuid::Uuid** | Capture ID. | [required] |

### Return type

[**crate::models::OrderPeriodCapture**](Order.Capture.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_by_id

> crate::models::OrderPeriodModelsPeriodOrder get_order_by_id(id, include, combine_product_container_rows)
Get an instance of a order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the order to get. | [required] |
**include** | **String** | A comma separated string of what related collections to include with this result set. The  possible values are: rows, paymentdetails, shippingdetails and refunds. | [required] |
**combine_product_container_rows** | Option<**bool**> | If true, will combine all order rows that are part of a container into a single container row. |  |

### Return type

[**crate::models::OrderPeriodModelsPeriodOrder**](Order.Models.Order.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_statuses

> Vec<crate::models::OrderPeriodModelsPeriodOrderStatus> get_order_statuses()
Get a list of available order statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::OrderPeriodModelsPeriodOrderStatus>**](Order.Models.OrderStatus.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_refund_by_id

> crate::models::OrderPeriodRefund get_refund_by_id(refund_id)
Get Refund by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refund_id** | **uuid::Uuid** | Refund ID. | [required] |

### Return type

[**crate::models::OrderPeriodRefund**](Order.Refund.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partial_update_of_order

> serde_json::Value partial_update_of_order(id, order)
Partial update of an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The orderId of the order to update | [required] |
**order** | [**OrderPeriodModelsPeriodOrderUpdate**](OrderPeriodModelsPeriodOrderUpdate.md) | The order object containing the properties that should be updated. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_orders

> Vec<crate::models::OrderPeriodModelsPeriodOrder> query_orders(query)
Query the order repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**OrderPeriodModelsPeriodOrderQuery**](OrderPeriodModelsPeriodOrderQuery.md) | The details of the query. | [required] |

### Return type

[**Vec<crate::models::OrderPeriodModelsPeriodOrder>**](Order.Models.Order.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_capture_as_processed

> serde_json::Value set_capture_as_processed(processed_capture)
Set a capture as processed (= captured)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processed_capture** | [**OrderPeriodProcessedCapture**](OrderPeriodProcessedCapture.md) | Contains CaptureId and optional metadata about capture. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_payment_as_payed

> serde_json::Value set_payment_as_payed(payment_detail_id)
Set Payment Detail as payed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_detail_id** | **i32** | Payment Detail ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_refund_as_processed

> serde_json::Value set_refund_as_processed(processed_refund)
Set a refund as processed (= settled)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**processed_refund** | [**OrderPeriodProcessedRefund**](OrderPeriodProcessedRefund.md) | Contains RefundId and optional metadata about refund. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_order_status

> serde_json::Value update_order_status(id, status, transaction_id, secondary_transaction_id)
Update order status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Order ID. | [required] |
**status** | **i32** | Order status. These statuses can be set: cancelled, on-hold, inactive, out-of-stock,  backorder, completed, pending. | [required] |
**transaction_id** | **String** | A transaction id can be set here if status is set to pending. | [required] |
**secondary_transaction_id** | **String** | A secondary transaction id, if any, can be set here if status is set to pending. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction_data

> serde_json::Value update_transaction_data(id, transaction_data)
Updates transaction data on an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Order ID. | [required] |
**transaction_data** | [**ApiPeriodOrderPeriodTransactionData**](ApiPeriodOrderPeriodTransactionData.md) | The transaction data | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_order

> serde_json::Value validate_order(request)
Validates order data for order creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**OrderPeriodValidateOrderCreationRequest**](OrderPeriodValidateOrderCreationRequest.md) | The order data to validate. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

