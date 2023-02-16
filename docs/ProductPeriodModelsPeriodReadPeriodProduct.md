# ProductPeriodModelsPeriodReadPeriodProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | Option<**i32**> | The unique identifier for the product. | [optional]
**article_number** | Option<**String**> | The article number of the product. | [optional]
**names** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | The localized names of the product. | [optional]
**date_created** | Option<**String**> | The date the product was created. | [optional]
**date_updated** | Option<**String**> | The date the product was last updated. | [optional]
**active** | Option<**bool**> | The current state of the product. | [optional]
**purchase_price** | Option<**f64**> | The purchase price in the currency defined in {Product.Models.Read.Product.PurchasePriceCurrency}. | [optional]
**purchase_price_currency** | Option<**String**> | The 3-letter ISO 4217 currency code for the amount given in {Product.Models.Read.Product.PurchasePrice}. | [optional]
**short_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized short texts for the product. | [optional]
**long_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized long texts for the product. | [optional]
**tech_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized tech texts for the product. | [optional]
**items** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductItem>**](Product.Models.Read.ProductItem.md)> | The items belonging to the product. | [optional]
**prices** | Option<[**Vec<crate::models::PriceListPeriodModelsPeriodReadPeriodPriceListPrice>**](PriceList.Models.Read.PriceListPrice.md)> | The current prices of the product. | [optional]
**categories** | Option<[**Vec<crate::models::CategoryPeriodModelsPeriodReadPeriodCategory>**](Category.Models.Read.Category.md)> | The categories the product belongs to. | [optional]
**images** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodImage>**](Product.Models.Read.Image.md)> | The images for the product | [optional]
**brand_id** | Option<**i32**> | The brand id of the product. | [optional]
**brand_name** | Option<**String**> | The brand name of the product. | [optional]
**supplier_id** | Option<**i32**> | The supplier id of the product. | [optional]
**supplier_name** | Option<**String**> | The supplier name of the product. | [optional]
**parameter_values** | Option<[**Vec<crate::models::ProductParameterPeriodModelsPeriodReadPeriodProductParameterValue>**](ProductParameter.Models.Read.ProductParameterValue.md)> | The parameter values associated with the product. | [optional]
**variants** | Option<[**Vec<crate::models::VariantPeriodModelsPeriodReadPeriodVariant>**](Variant.Models.Read.Variant.md)> | The variants for this product. | [optional]
**markets** | Option<[**Vec<crate::models::MarketPeriodModelsPeriodMarket>**](Market.Models.Market.md)> | The markets for this product | [optional]
**vat** | Option<**f64**> | The vat percent for this product. Eg) 0.25 meaning 25% VAT. | [optional]
**primary_image** | Option<**String**> | The filename of this products primary image. | [optional]
**freight_class_id** | Option<**i32**> | ID of freight class | [optional]
**intrastat_code** | Option<**String**> | Intrastat code of the product | [optional]
**country_of_origin** | Option<**String**> | Country of orgin of product | [optional]
**variant_group_id** | Option<**i32**> | ID of Variant Group to which the product is associated | [optional]
**vat_id** | Option<**i32**> | ID of Vat | [optional]
**external_id** | Option<**String**> | External Id of the product. | [optional]
**activation_date** | Option<**String**> | Activation date for the product. | [optional]
**feeds** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodFeedMembership>**](Product.Models.Read.FeedMembership.md)> | The feeds the product is a member of | [optional]
**urls** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodProductUrl>**](Product.Models.Read.ProductUrl.md)> | All canonical urls for the product | [optional]
**main_category_id** | Option<**i32**> | The main category id for the product. | [optional]
**related_products** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodReadPeriodRelatedProduct>**](Product.Models.Read.RelatedProduct.md)> | The related products for the product. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


