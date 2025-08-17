use salvo::Request;
use serde::Deserialize;
use std::io::Write;
use validator::Validate;

use crate::{reject, resp::Res};

pub mod extractor;

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

pub async fn parse_json_validate<T: Validate + for<'a> Deserialize<'a>>(
    req: &mut Request,
) -> Result<T, Res> {
    let data: T = req.parse_json().await?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_form_validate<T: Validate + for<'a> Deserialize<'a>>(
    req: &mut Request,
) -> Result<T, Res> {
    let data: T = req.parse_form().await?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_path_validate<T: Validate + for<'a> Deserialize<'a>>(
    req: &mut Request,
) -> Result<T, Res> {
    let data: T = req.parse_params()?;
    validate(&data)?;
    Ok(data)
}

pub async fn parse_query_validate<T: Validate + for<'a> Deserialize<'a>>(
    req: &mut Request,
) -> Result<T, Res> {
    let data: T = req.parse_queries()?;
    validate(&data)?;
    Ok(data)
}
