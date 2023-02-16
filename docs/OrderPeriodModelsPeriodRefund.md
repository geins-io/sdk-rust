# OrderPeriodModelsPeriodRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for this refund | [optional]
**order_row_id** | Option<**i32**> | Reference to the order row that has been refunded | [optional]
**payment_detail_id** | Option<**i32**> | Reference to the payment detail that has been refunded | [optional]
**return_id** | Option<**i32**> | Id number of the return. Can be used to group refunds. | [optional]
**article_number** | Option<**String**> | Article number. If the refund is not bound to an order row this field contains an optional refund article number. | [optional]
**created_at** | Option<**String**> | Datetime when the refund was created | [optional]
**total** | Option<**f64**> | Total amount refunded | [optional]
**reason_code** | Option<**i32**> | Reason code for the refund | [optional]
**reason** | Option<**String**> | Reason for refund | [optional]
**to_balance** | Option<**bool**> | Shows if the refund was deposited to the customers balance | [optional]
**vat** | Option<**f64**> | Vat percent in decimals for the refunded amount | [optional]
**item_id** | Option<**i32**> | Item ID (SKU). | [optional]
**refund_type** | Option<**String**> | Refund Type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


