use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use chrono::NaiveDate;
use lib::datetime::{to_datetime_jst, today_jst};
use lib::{establish_connection, select_daily_rates, select_rate_type};
use log::info;
use std::marker::PhantomData;
use std::time::Duration;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::{ApiError, EmptyContext, Has, XSpanIdString};

use openapi_client::models::{self, ErrorResponse};
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
        let pair = pair.to_string();
        let period = if let Some(p) = period {
            p
        } else {
            models::Period::Daily
        };
        let base_date: NaiveDate = if let Some(d) = base_datetime {
            d.date_naive()
        } else {
            today_jst()
        };

        let conn = &mut establish_connection();

        let rate_type = select_rate_type(conn, &pair);
        if rate_type.is_none() {
            let response = ErrorResponse {
                error: format!("pair is unsupported, pair:[{}]", pair).to_owned(),
            };
            return Ok(GetRatesResponse::Status404(response));
        }

        let rate_type_id = rate_type.unwrap().id;
        let limit: i64 = if let Some(limit) = count {
            limit as i64
        } else {
            100
        };
        let rates = select_daily_rates(conn, &rate_type_id, &base_date, limit);
        if rates.is_empty() {
            let response = ErrorResponse {
                error: format!("rate is empty, pair:[{}]", pair).to_owned(),
            };
            return Ok(GetRatesResponse::Status404(response));
        }

        let rates = rates
            .iter()
            .map(|v| {
                let begin_date_jst = to_datetime_jst(v.rate_date, 0);
                let end_date_jst = begin_date_jst + Duration::from_millis(24 * 60 * 60 * 1000 - 1);
                models::Rate {
                    closing: v.closing_rate.to_f64().unwrap(),
                    opening: v.opening_rate.to_f64().unwrap(),
                    high: v.high_rate.to_f64().unwrap(),
                    low: v.low_rate.to_f64().unwrap(),
                    volume: v.volume.to_f64().unwrap(),
                    begin_date: begin_date_jst.to_utc(),
                    end_date: end_date_jst.to_utc(),
                }
            })
            .collect();

        let response = models::GetRatesResponse { period, rates };
        Ok(GetRatesResponse::Status200(response))
    }
}
