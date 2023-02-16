# OrderPeriodRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refund_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**refund_instance_id** | Option<**i32**> |  | [optional]
**order_id** | Option<**i32**> |  | [optional]
**external_order_id** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**reference** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**processed_on** | Option<**String**> |  | [optional]
**refunded_item_total** | Option<**f64**> |  | [optional]
**refunded_shipping_fee** | Option<**f64**> |  | [optional]
**refunded_payment_fee** | Option<**f64**> |  | [optional]
**refunded_discount** | Option<**f64**> |  | [optional]
**refunded_balance** | Option<**f64**> |  | [optional]
**vat_rate** | Option<**f64**> |  | [optional]
**payment_name** | Option<**String**> |  | [optional]
**locale** | Option<**String**> |  | [optional]
**rows** | Option<[**Vec<crate::models::OrderPeriodRefundRow>**](Order.RefundRow.md)> |  | [optional]
**order_transaction_id** | Option<**String**> |  | [optional]
**secondary_order_transaction_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


