use std::sync::LazyLock;

use derive_more::derive::{Deref, DerefMut};
use salvo::{
    http::{
        headers::{ContentType, HeaderMapExt},
        HeaderMap,
    },
    Extractible,
};
use serde::Deserialize;
use validator::Validate;

use crate::global::METADATE;

use super::{parse_form_validate, parse_json_validate, parse_path_validate, parse_query_validate};

static JSON: LazyLock<ContentType> = LazyLock::new(ContentType::json);

/// 判断 json 请求头
pub fn is_json_content(headers: &HeaderMap) -> bool {
    headers
        .typed_get::<ContentType>()
        .map(|t| t == *JSON)
        .unwrap_or(false)
}

/// 提取 Json 数据并校验
#[derive(Debug, Deref, DerefMut)]
pub struct VJson<T: Validate>(pub T);

impl<'ex, T: Validate + for<'de> Deserialize<'de>> Extractible<'ex> for VJson<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(
        req: &'ex mut salvo::Request,
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static> {
        parse_json_validate(req).await.map(Self)
    }
}

/// 提取 Form 数据并校验``
#[derive(Debug, Deref, DerefMut)]
pub struct VForm<T: Validate>(pub T);

impl<'ex, T: Validate + for<'de> Deserialize<'de>> Extractible<'ex> for VForm<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(
        req: &'ex mut salvo::Request,
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static>
    where
        Self: Sized,
    {
        parse_form_validate(req).await.map(Self)
    }
}

/// 提取 Path 数据并校验
#[derive(Debug, Deref, DerefMut)]
pub struct VPath<T: Validate>(pub T);

impl<'ex, T: Validate + for<'de> Deserialize<'de>> Extractible<'ex> for VPath<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(
        req: &'ex mut salvo::Request,
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static>
    where
        Self: Sized,
    {
        parse_path_validate(req).await.map(Self)
    }
}

/// 提取 Path 数据并校验
#[derive(Debug, Deref, DerefMut)]
pub struct VQuery<T: Validate>(pub T);

impl<'ex, T: Validate + for<'de> Deserialize<'de>> Extractible<'ex> for VQuery<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(
        req: &'ex mut salvo::Request,
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static>
    where
        Self: Sized,
    {
        parse_query_validate(req).await.map(Self)
    }
}
