//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use log::info;
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::ApiError;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

use openapi_client::models::{self};
use openapi_client::server::MakeService;
use openapi_client::{Api, GetRatesResponse};

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(service);

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
