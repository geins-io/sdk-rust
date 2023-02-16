# ProductPeriodModelsPeriodReadPeriodProductItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | Option<**i32**> | The product item id. | [optional]
**article_number** | Option<**String**> | The article number for the product item. | [optional]
**product_id** | Option<**i32**> | The id of the product that the item belongs to. | [optional]
**name** | Option<**String**> | The name of the product item. | [optional]
**shelf** | Option<**String**> | The shelf name where the product item can be found. | [optional]
**weight** | Option<**i32**> | The weight of the item in grams (g). | [optional]
**length** | Option<**i32**> | The length of the item in millimeters (mm). | [optional]
**width** | Option<**i32**> | The width of the item in millimeters (mm). | [optional]
**height** | Option<**i32**> | The height of the item in millimeters (mm). | [optional]
**gtin** | Option<**String**> | The GTIN number for the item. | [optional]
**date_created** | Option<**String**> | The date the item was created. | [optional]
**date_updated** | Option<**String**> | The date the item was last updated. | [optional]
**active** | Option<**bool**> | The current state of the item. | [optional]
**external_id** | Option<**String**> | External Id of the product item. | [optional]
**stock** | Option<[**crate::models::ProductPeriodModelsPeriodReadPeriodProductItemStock**](Product.Models.Read.ProductItemStock.md)> |  | [optional]
**shipping_fees** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodShippingFee>**](Product.Models.Read.ShippingFee.md)> | The lowest shipping fees for each market and country for the product item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


