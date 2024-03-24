use futures::{future, future::BoxFuture, future::FutureExt, stream, stream::TryStreamExt, Stream};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, HeaderMap, Request, Response, StatusCode};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
use url::form_urlencoded;

use crate::header;
#[allow(unused_imports)]
use crate::models;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api, GetRatesResponse};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet =
            regex::RegexSet::new(vec![r"^/api/v1/rates/(?P<pair>[^/?#]*)$"])
                .expect("Unable to create global regex set");
    }
    pub(crate) static ID_RATES_PAIR: usize = 0;
    lazy_static! {
        pub static ref REGEX_RATES_PAIR: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/api/v1/rates/(?P<pair>[^/?#]*)$")
                .expect("Unable to create regex for RATES_PAIR");
    }
}

pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(self.api_impl.clone()))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Unable to create Method Not Allowed response"))
}

pub struct Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C> Clone for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C>
where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Send + Sync + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match method {
                // GetRates - GET /rates/{pair}
                hyper::Method::GET if path.matched(paths::ID_RATES_PAIR) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params = paths::REGEX_RATES_PAIR.captures(path).unwrap_or_else(|| {
                        panic!(
                            "Path {} matched RE RATES_PAIR in set but failed match against \"{}\"",
                            path,
                            paths::REGEX_RATES_PAIR.as_str()
                        )
                    });

                    let param_pair = match percent_encoding::percent_decode(path_params["pair"].as_bytes()).decode_utf8() {
                    Ok(param_pair) => match param_pair.parse::<models::Pair>() {
                        Ok(param_pair) => param_pair,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter pair: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["pair"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_period = query_params
                        .iter()
                        .filter(|e| e.0 == "period")
                        .map(|e| e.1.clone())
                        .next();
                    let param_period = match param_period {
                        Some(param_period) => {
                            let param_period =
                                <models::Period as std::str::FromStr>::from_str(&param_period);
                            match param_period {
                            Ok(param_period) => Some(param_period),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter period - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter period")),
                        }
                        }
                        None => None,
                    };
                    let param_count = query_params
                        .iter()
                        .filter(|e| e.0 == "count")
                        .map(|e| e.1.clone())
                        .next();
                    let param_count = match param_count {
                        Some(param_count) => {
                            let param_count = <i32 as std::str::FromStr>::from_str(&param_count);
                            match param_count {
                            Ok(param_count) => Some(param_count),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter count - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter count")),
                        }
                        }
                        None => None,
                    };
                    let param_base_datetime = query_params
                        .iter()
                        .filter(|e| e.0 == "base_datetime")
                        .map(|e| e.1.clone())
                        .next();
                    let param_base_datetime = match param_base_datetime {
                        Some(param_base_datetime) => {
                            let param_base_datetime =
                                <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(
                                    &param_base_datetime,
                                );
                            match param_base_datetime {
                            Ok(param_base_datetime) => Some(param_base_datetime),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter base_datetime - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter base_datetime")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_rates(
                            param_pair,
                            param_period,
                            param_count,
                            param_base_datetime,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetRatesResponse::Status200(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RATES_STATUS200"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetRatesResponse::Status404(body) => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RATES_STATUS404"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                        },
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                _ if path.matched(paths::ID_RATES_PAIR) => method_not_allowed(),
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")),
            }
        }
        Box::pin(run(self.api_impl.clone(), req))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // GetRates - GET /rates/{pair}
            hyper::Method::GET if path.matched(paths::ID_RATES_PAIR) => Some("GetRates"),
            _ => None,
        }
    }
}
