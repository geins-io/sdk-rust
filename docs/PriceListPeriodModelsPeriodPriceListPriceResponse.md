# PriceListPeriodModelsPeriodPriceListPriceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<**String**> | Information about the outcome of the request. | [optional]
**invalid** | Option<[**Vec<crate::models::PriceListPeriodModelsPeriodWritePeriodPriceListPrice>**](PriceList.Models.Write.PriceListPrice.md)> | Supplied PriceList prices that failed validation. | [optional]
**not_found** | Option<[**Vec<crate::models::PriceListPeriodModelsPeriodWritePeriodPriceListPrice>**](PriceList.Models.Write.PriceListPrice.md)> | Supplied PriceList prices that were technically valid, but couldn't be found. | [optional]
**update_count** | Option<**i32**> | Number of price updates resulting from the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


