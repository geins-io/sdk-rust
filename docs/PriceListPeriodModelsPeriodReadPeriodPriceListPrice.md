# PriceListPeriodModelsPeriodReadPeriodPriceListPrice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**i32**> | The id of the product that this price applies to. | [optional]
**price_list_id** | Option<**i32**> | The id of the price list that this price is associated with. | [optional]
**price_list_name** | Option<**String**> | The name of the price list that this price is associated with. | [optional]
**price_inc_vat** | Option<**f64**> | The price, inc VAT, in the currency of the associated price list. | [optional]
**price_ex_vat** | Option<**f64**> | The price, ex VAT, in the currency of the associated price list. | [optional]
**vat_rate** | Option<**f64**> | The Vat Rate | [optional]
**country** | Option<**String**> | The 2-letter ISO country code for this price. | [optional]
**currency** | Option<**String**> | The 3-letter ISO 4217 currency code for this price. | [optional]
**staggered_count** | Option<**i32**> | Staggered count for this price. Defaults to 1. | [optional]
**valid_from** | Option<**String**> | The date the price is valid from. | [optional]
**valid_to** | Option<**String**> | The date the price is valid to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


