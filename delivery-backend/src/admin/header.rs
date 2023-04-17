use axum::{
    headers::{Error as HeaderError, Header},
    http::{request::Parts, HeaderName, HeaderValue},
    RequestPartsExt, TypedHeader
};
use std::iter;

use crate::error::AppError;

static X_ADMIN_HEADER: HeaderName = HeaderName::from_static("x-admin-secret");

#[derive(Debug)]
pub struct AdminHeader(HeaderValue);

impl Header for AdminHeader {
    fn name() -> &'static HeaderName {
        &X_ADMIN_HEADER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, HeaderError>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>
    {
        values.next().cloned().map(AdminHeader).ok_or_else(HeaderError::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(iter::once(self.0.clone()))
    }
}

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AdminHeader
where
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(admin_header) =
            parts.extract::<TypedHeader<Self>>().await.map_err(AppError::from)?;

        Ok(admin_header)
    }
}
