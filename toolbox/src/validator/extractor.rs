use std::{io::Write, sync::LazyLock};

use derive_more::derive::{Deref, DerefMut};
use salvo::{
    Extractible, Request,
    http::{
        HeaderMap,
        headers::{ContentType, HeaderMapExt},
    },
};
use serde::Deserialize;
use validator::Validate;

use crate::{global::METADATE, reject, resp::Res};

/// 数据验证
pub fn validate(data: impl Validate) -> Result<(), Res> {
    if let Err(err) = data.validate() {
        let mut msg: Vec<u8> = Vec::new();
        write!(msg, "数据验证失败: ")?;
        for (key, value) in err.field_errors() {
            write!(msg, "{key}<")?;
            for field in value {
                write!(msg, "{}, ", field.code)?
            }
            msg.truncate(msg.len() - 2);
            msg.extend(b">; ");
        }
        msg.pop();
        return reject!(422, unsafe { String::from_utf8_unchecked(msg) });
    }
    Ok(())
}

pub async fn parse_json_validate<T: Validate + for<'a> Deserialize<'a>>(req: &mut Request) -> Result<T, Res> {
    let data: T = req.parse_json().await?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_form_validate<T: Validate + for<'a> Deserialize<'a>>(req: &mut Request) -> Result<T, Res> {
    let data: T = req.parse_form().await?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_path_validate<T: Validate + for<'a> Deserialize<'a>>(req: &mut Request) -> Result<T, Res> {
    let data: T = req.parse_params()?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_query_validate<T: Validate + for<'a> Deserialize<'a>>(req: &mut Request) -> Result<T, Res> {
    let data: T = req.parse_queries()?;
    validate(&data)?;
    Ok(data)
}

static JSON: LazyLock<ContentType> = LazyLock::new(ContentType::json);

/// 判断 json 请求头
pub fn is_json_content(headers: &HeaderMap) -> bool {
    headers.typed_get::<ContentType>().map(|t| t == *JSON).unwrap_or(false)
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

/// 提取 Form 数据并校验
#[derive(Debug, Deref, DerefMut)]
pub struct VForm<T: Validate>(pub T);

impl<'ex, T: Validate + for<'de> Deserialize<'de>> Extractible<'ex> for VForm<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(
        req: &'ex mut salvo::Request,
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static> {
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
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static> {
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
    ) -> Result<Self, impl salvo::Writer + Send + std::fmt::Debug + 'static> {
        parse_query_validate(req).await.map(Self)
    }
}
