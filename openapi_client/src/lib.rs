#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::task::{Context, Poll};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/api/v1";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetRatesResponse {
    /// 通貨ペアのレート
    Status200(models::GetRatesResponse),
    /// レート情報なし
    Status404(models::ErrorResponse),
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// 通貨ペアのレートを取得
    async fn get_rates(
        &self,
        pair: models::Pair,
        period: Option<models::Period>,
        count: Option<i32>,
        base_datetime: Option<chrono::DateTime<chrono::Utc>>,
        context: &C,
    ) -> Result<GetRatesResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// 通貨ペアのレートを取得
    async fn get_rates(
        &self,
        pair: models::Pair,
        period: Option<models::Period>,
        count: Option<i32>,
        base_datetime: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<GetRatesResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// 通貨ペアのレートを取得
    async fn get_rates(
        &self,
        pair: models::Pair,
        period: Option<models::Period>,
        count: Option<i32>,
        base_datetime: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<GetRatesResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .get_rates(pair, period, count, base_datetime, &context)
            .await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
