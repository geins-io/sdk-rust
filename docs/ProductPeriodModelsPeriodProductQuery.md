# ProductPeriodModelsPeriodProductQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_after** | Option<**String**> | Limits query to products updated after the specified date. | [optional]
**product_ids** | Option<**Vec<i32>**> | Limits query to only include the supplied product ids. | [optional]
**article_numbers** | Option<**Vec<String>**> | Limits query to only include products with supplied article numbers. | [optional]
**only_sellable** | Option<**bool**> | Limits query to only include products that are available for purchase | [optional]
**feed_id** | Option<**i32**> | Limits query to only include products contained in the specified feed. | [optional]
**batch_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Used to fetch products where the result set is split into batches | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


