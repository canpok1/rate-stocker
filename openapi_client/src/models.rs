#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorResponse {
    /// エラーメッセージ
    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

}


impl ErrorResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            error: None,
        }
    }
}

/// Converts the ErrorResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.error.as_ref().map(|error| {
                [
                    "error".to_string(),
                    error.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorResponse {
            error: intermediate_rep.error.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetRatesResponse {
    #[serde(rename = "period")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub period: Option<models::Period>,

    #[serde(rename = "rates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rates: Option<Vec<models::Rate>>,

}


impl GetRatesResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> GetRatesResponse {
        GetRatesResponse {
            period: None,
            rates: None,
        }
    }
}

/// Converts the GetRatesResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetRatesResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping period in query parameter serialization

            // Skipping rates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetRatesResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetRatesResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub period: Vec<models::Period>,
            pub rates: Vec<Vec<models::Rate>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetRatesResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "period" => intermediate_rep.period.push(<models::Period as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "rates" => return std::result::Result::Err("Parsing a container in this style is not supported in GetRatesResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetRatesResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetRatesResponse {
            period: intermediate_rep.period.into_iter().next(),
            rates: intermediate_rep.rates.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetRatesResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetRatesResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetRatesResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetRatesResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetRatesResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetRatesResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetRatesResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 通貨ペア
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Pair {
    #[serde(rename = "btc_jpy")]
    BtcJpy,
    #[serde(rename = "etc_jpy")]
    EtcJpy,
    #[serde(rename = "lsk_jpy")]
    LskJpy,
    #[serde(rename = "mona_jpy")]
    MonaJpy,
    #[serde(rename = "plt_jpy")]
    PltJpy,
    #[serde(rename = "fnct_jpy")]
    FnctJpy,
    #[serde(rename = "dai_jpy")]
    DaiJpy,
    #[serde(rename = "wbtc_jpy")]
    WbtcJpy,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pair::BtcJpy => write!(f, "btc_jpy"),
            Pair::EtcJpy => write!(f, "etc_jpy"),
            Pair::LskJpy => write!(f, "lsk_jpy"),
            Pair::MonaJpy => write!(f, "mona_jpy"),
            Pair::PltJpy => write!(f, "plt_jpy"),
            Pair::FnctJpy => write!(f, "fnct_jpy"),
            Pair::DaiJpy => write!(f, "dai_jpy"),
            Pair::WbtcJpy => write!(f, "wbtc_jpy"),
        }
    }
}

impl std::str::FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "btc_jpy" => std::result::Result::Ok(Pair::BtcJpy),
            "etc_jpy" => std::result::Result::Ok(Pair::EtcJpy),
            "lsk_jpy" => std::result::Result::Ok(Pair::LskJpy),
            "mona_jpy" => std::result::Result::Ok(Pair::MonaJpy),
            "plt_jpy" => std::result::Result::Ok(Pair::PltJpy),
            "fnct_jpy" => std::result::Result::Ok(Pair::FnctJpy),
            "dai_jpy" => std::result::Result::Ok(Pair::DaiJpy),
            "wbtc_jpy" => std::result::Result::Ok(Pair::WbtcJpy),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// 期間
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Period {
    #[serde(rename = "daily")]
    Daily,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Period::Daily => write!(f, "daily"),
        }
    }
}

impl std::str::FromStr for Period {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "daily" => std::result::Result::Ok(Period::Daily),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// レート
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Rate {
    /// 終値
    #[serde(rename = "closing")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closing: Option<f64>,

    /// 始値
    #[serde(rename = "opening")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opening: Option<f64>,

    /// 高値
    #[serde(rename = "high")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub high: Option<f64>,

    /// 安値
    #[serde(rename = "low")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub low: Option<f64>,

    /// 出来高
    #[serde(rename = "volume")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume: Option<f64>,

    /// 期間の開始日時
    #[serde(rename = "begin_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub begin_date: Option<chrono::DateTime::<chrono::Utc>>,

    /// 期間の終了日時
    #[serde(rename = "end_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_date: Option<chrono::DateTime::<chrono::Utc>>,

}


impl Rate {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Rate {
        Rate {
            closing: None,
            opening: None,
            high: None,
            low: None,
            volume: None,
            begin_date: None,
            end_date: None,
        }
    }
}

/// Converts the Rate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Rate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.closing.as_ref().map(|closing| {
                [
                    "closing".to_string(),
                    closing.to_string(),
                ].join(",")
            }),


            self.opening.as_ref().map(|opening| {
                [
                    "opening".to_string(),
                    opening.to_string(),
                ].join(",")
            }),


            self.high.as_ref().map(|high| {
                [
                    "high".to_string(),
                    high.to_string(),
                ].join(",")
            }),


            self.low.as_ref().map(|low| {
                [
                    "low".to_string(),
                    low.to_string(),
                ].join(",")
            }),


            self.volume.as_ref().map(|volume| {
                [
                    "volume".to_string(),
                    volume.to_string(),
                ].join(",")
            }),

            // Skipping begin_date in query parameter serialization

            // Skipping end_date in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Rate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Rate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub closing: Vec<f64>,
            pub opening: Vec<f64>,
            pub high: Vec<f64>,
            pub low: Vec<f64>,
            pub volume: Vec<f64>,
            pub begin_date: Vec<chrono::DateTime::<chrono::Utc>>,
            pub end_date: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Rate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "closing" => intermediate_rep.closing.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "opening" => intermediate_rep.opening.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "high" => intermediate_rep.high.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "low" => intermediate_rep.low.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volume" => intermediate_rep.volume.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "begin_date" => intermediate_rep.begin_date.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "end_date" => intermediate_rep.end_date.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Rate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Rate {
            closing: intermediate_rep.closing.into_iter().next(),
            opening: intermediate_rep.opening.into_iter().next(),
            high: intermediate_rep.high.into_iter().next(),
            low: intermediate_rep.low.into_iter().next(),
            volume: intermediate_rep.volume.into_iter().next(),
            begin_date: intermediate_rep.begin_date.into_iter().next(),
            end_date: intermediate_rep.end_date.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Rate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Rate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Rate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Rate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Rate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Rate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Rate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

