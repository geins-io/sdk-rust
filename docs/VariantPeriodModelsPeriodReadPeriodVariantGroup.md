# VariantPeriodModelsPeriodReadPeriodVariantGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**i32**> | The id of variant goup. | [optional]
**name** | Option<**String**> | The optional internal name of the variant group. | [optional]
**collapse_in_lists** | Option<**bool**> | Determine visibility of non-main products of this group in lists. | [optional]
**main_product_id** | Option<**i32**> | The main product of this group. If the group is collapsed in lists, this will be the only visible product. | [optional]
**product_ids** | Option<**Vec<i32>**> | The product ids of the variants that belong to this group. | [optional]
**products** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProduct>**](Product.Models.Read.Product.md)> | Products belonging to the Variant group. Only included when parameter \"include\" is supplied in the query string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


