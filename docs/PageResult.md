# PageResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the batch operation. If this property has a value for the first fetched page it has to be passed as a parameter for all subsequent requests- | [optional]
**page** | Option<**i32**> | The current page | [optional]
**row_count** | Option<**i32**> | Total number of rows | [optional]
**page_count** | Option<**i32**> | Total number of pages | [optional]
**page_size** | Option<**i32**> | Page size | [optional]
**has_more_rows** | Option<**bool**> | True if there is more content to fetch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


