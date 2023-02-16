# OrderPeriodCapture

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capture_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**order_payment_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**order_id** | Option<**i32**> |  | [optional]
**external_order_id** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**reference** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**processed_on** | Option<**String**> |  | [optional]
**captured_item_total** | Option<**f64**> |  | [optional]
**captured_shipping_fee** | Option<**f64**> |  | [optional]
**captured_payment_fee** | Option<**f64**> |  | [optional]
**captured_discount** | Option<**f64**> |  | [optional]
**captured_balance** | Option<**f64**> |  | [optional]
**vat_rate** | Option<**f64**> |  | [optional]
**tracking_number** | Option<**String**> |  | [optional]
**shipping_name** | Option<**String**> |  | [optional]
**tracking_uri** | Option<**String**> |  | [optional]
**shipping_method** | Option<**String**> |  | [optional]
**payment_name** | Option<**String**> |  | [optional]
**locale** | Option<**String**> |  | [optional]
**rows** | Option<[**Vec<crate::models::OrderPeriodCaptureRow>**](Order.CaptureRow.md)> |  | [optional]
**order_transaction_id** | Option<**String**> |  | [optional]
**secondary_order_transaction_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


