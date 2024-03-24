# rate_api

All URIs are relative to *http://127.0.0.1:8080/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**getRates**](rate_api.md#getRates) | **GET** /rates/{pair} | 通貨ペアのレートを取得


# **getRates**
> models::GetRatesResponse getRates(pair, optional)
通貨ペアのレートを取得

基準日時における最新のレート情報を取得します。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **pair** | [****](.md)| 通貨ペア | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pair** | [****](.md)| 通貨ペア | 
 **period** | [****](.md)| 期間 | 
 **count** | **i32**| 取得件数 | [default to 100]
 **base_datetime** | **chrono::DateTime::<chrono::Utc>**| 基準日時。省略時は現在日時となる。 | 

### Return type

[**models::GetRatesResponse**](GetRatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

