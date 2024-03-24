use async_trait::async_trait;
use log::info;
use std::marker::PhantomData;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};

use openapi_client::models::{self};
use openapi_client::server::MakeService;
use openapi_client::{Api, GetRatesResponse};

pub async fn create(addr: &str) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let service = openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    hyper::server::Server::bind(&addr)
        .serve(service)
        .await
        .unwrap()
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// 通貨ペアのレートを取得
    async fn get_rates(
        &self,
        pair: models::Pair,
        period: Option<models::Period>,
        count: Option<i32>,
        base_datetime: Option<chrono::DateTime<chrono::Utc>>,
        context: &C,
    ) -> Result<GetRatesResponse, ApiError> {
        info!(
            "get_rates({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}",
            pair,
            period,
            count,
            base_datetime,
            context.get().0.clone()
        );
        let response = models::GetRatesResponse {
            period,
            rates: Some(vec![{
                models::Rate {
                    closing: Some(1.0),
                    opening: Some(1.0),
                    high: Some(1.0),
                    low: Some(1.0),
                    volume: Some(1.0),
                    begin_date: Some(chrono::Utc::now()),
                    end_date: Some(chrono::Utc::now()),
                }
            }]),
        };
        Ok(GetRatesResponse::Status200(response))
        //Err(ApiError("Generic failure".into()))
    }
}
