# OrderPeriodModelsPeriodPaymentDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier for this payment detail. Exception: For some payment options this field can be 0. These orders only have one payment detail. | [optional]
**payment_id** | Option<**i32**> | Payment method id | [optional]
**name** | Option<**String**> | The name of the payment method | [optional]
**display_name** | Option<**String**> | The display name of the payment method | [optional]
**transaction_id** | Option<**String**> | The transaction id (external reference). | [optional]
**secondary_transaction_id** | Option<**String**> | The secondary transaction id, if any (external reference). | [optional]
**reservation_number** | Option<**String**> | The reservation number. This field is not available for all payment methods. | [optional]
**reservation_date** | Option<**String**> | Reservation date | [optional]
**payment_date** | Option<**String**> | Payment date | [optional]
**total** | Option<**f64**> | Total | [optional]
**payed** | Option<**bool**> | Shows if the order is paid using this payment method | [optional]
**payment_fee** | Option<**f64**> | The payment fee | [optional]
**shipping_fee** | Option<**f64**> | The shipping fee | [optional]
**payment_option** | Option<**String**> | The name of the payment option, if any.  This doesn't have to be the same as the payment name. Eg \"Direct bank payment\", \"Card\", \"Invoice\" etc. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


