# \UserApi

All URIs are relative to *https://mgmtapi.geins.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_profile**](UserApi.md#create_user_profile) | **POST** /API/User | Create user profile
[**delete_user_profile**](UserApi.md#delete_user_profile) | **DELETE** /API/User/email | Delete user profile
[**get_user_profile**](UserApi.md#get_user_profile) | **POST** /API/User/Query | Get a specific user profile
[**update_user_profile**](UserApi.md#update_user_profile) | **PUT** /API/User | Update user profile



## create_user_profile

> crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile create_user_profile(user_profile)
Create user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_profile** | [**UserPeriodModelsPeriodWritePeriodUserProfile**](UserPeriodModelsPeriodWritePeriodUserProfile.md) | The user profile | [required] |

### Return type

[**crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile**](Envelope-User.Models.Read.UserProfile.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_profile

> serde_json::Value delete_user_profile(email)
Delete user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_profile

> crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile get_user_profile(query)
Get a specific user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**UserPeriodModelsPeriodUserProfileQuery**](UserPeriodModelsPeriodUserProfileQuery.md) | The details of the query. | [required] |

### Return type

[**crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile**](Envelope-User.Models.Read.UserProfile.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_profile

> crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile update_user_profile(user_profile)
Update user profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_profile** | [**UserPeriodModelsPeriodWritePeriodUserProfile**](UserPeriodModelsPeriodWritePeriodUserProfile.md) | The user profile | [required] |

### Return type

[**crate::models::EnvelopeUserPeriodModelsPeriodReadPeriodUserProfile**](Envelope-User.Models.Read.UserProfile.md)

### Authorization

[apiKey](../README.md#apiKey), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
- **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

