# ProductPeriodModelsPeriodWritePeriodProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**article_number** | Option<**String**> | The article number of the product. | [optional]
**names** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | The localized names of the product. | [optional]
**active** | Option<**bool**> | The current state of the product. | [optional]
**purchase_price** | Option<**f64**> | The purchase price in the currency defined in {Product.Models.Write.Product.PurchasePriceCurrency}. | [optional]
**purchase_price_currency** | Option<**String**> | The 3-letter ISO 4217 currency code for the amount given in {Product.Models.Write.Product.PurchasePrice}. | [optional]
**short_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized short texts for the product. | [optional]
**long_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized long texts for the product. | [optional]
**tech_texts** | Option<[**Vec<crate::models::SharedPeriodModelsPeriodLocalizableContent>**](Shared.Models.LocalizableContent.md)> | Localized tech texts for the product. | [optional]
**brand_id** | Option<**i32**> | The brand of the product. | [optional]
**supplier_id** | Option<**i32**> | The supplier id of the product. | [optional]
**items** | Option<[**Vec<crate::models::ProductPeriodModelsPeriodWritePeriodProductItem>**](Product.Models.Write.ProductItem.md)> | The items belonging to the product. | [optional]
**category_ids** | Option<**Vec<i32>**> | The category ids the product belongs to. | [optional]
**parameter_values** | Option<[**Vec<crate::models::ProductParameterPeriodModelsPeriodWritePeriodProductParameterValue>**](ProductParameter.Models.Write.ProductParameterValue.md)> | The parameter values associated with the product. | [optional]
**variants** | Option<[**Vec<crate::models::VariantPeriodModelsPeriodWritePeriodVariant>**](Variant.Models.Write.Variant.md)> | The variants for this product. | [optional]
**markets** | Option<[**Vec<crate::models::MarketPeriodModelsPeriodMarket>**](Market.Models.Market.md)> | The markets for this product | [optional]
**freight_class_id** | Option<**i32**> | ID of freight class | [optional]
**intrastat_code** | Option<**String**> | Intrastat code of the product | [optional]
**country_of_origin** | Option<**String**> | Country of orgin of product | [optional]
**variant_group_id** | Option<**i32**> | ID of Variant Group to whom the product should be associated | [optional]
**vat** | Option<**i32**> | ID or rate of VAT (On create and if no VAT is provided then default VAT will be used) | [optional]
**vat_type** | Option<**String**> | Defines how VAT parameter should be interpreted  Actual = VAT parameter is interpreted as VAT rate  VatId = VAT parameter is interpreted as VAT Id | [optional]
**external_id** | Option<**String**> | External Id of the product. | [optional]
**activation_date** | Option<**String**> | Activation date for the product. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


