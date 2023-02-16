# OrderPeriodModelsPeriodOrderQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated** | Option<**String**> | Given a date, only orders updated after the provided date will be returned. | [optional]
**status_list** | Option<**String**> | Comma separated list of statuses to filter on. | [optional]
**market_id** | Option<**i32**> | Id of a market. | [optional]
**payment_name** | Option<**String**> | Name of a payment method | [optional]
**parcel_group_id** | Option<**i32**> | Id of a parcel group. | [optional]
**customer_id** | Option<**i32**> | The ID of a customer | [optional]
**email** | Option<**String**> | The email of a customer | [optional]
**include** | Option<**String**> | Comma separated list of child-collections to also include in the query result. | [optional]
**external_order_status** | Option<**i32**> | This status can be used by external systems to change the status of an order. Such as failed or done. | [optional]
**combine_product_container_rows** | Option<**bool**> | If true, will combine all order rows that are part of a container into a single container row. | [optional]
**packing_location_id** | Option<**i32**> | The packing place to get orders from. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


