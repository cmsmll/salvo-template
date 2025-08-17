use std::io;

use crate::resp::Res;
use jsonwebtoken::errors::Error as JwtError;
use salvo::http::ParseError;

macro_rules! erro_from_res{
    ($ty:path, $code:expr, $msg:expr) => {
        #[allow(unused_variables)]
        impl From<$ty> for Res<()> {
            fn from(value: $ty) -> Self {
                Res::msg($code, $msg.into())
            }
        }
    };

    ($ty:path, $code:expr, $msg:expr, this) => {
        impl From<$ty> for Res<()> {
            fn from(value: $ty) -> Self {
                Res::new($code, format!($msg, value).into(), ())
            }
        }
    };

    ($ty:path, $code:expr, $msg:expr, $($field:ident $(,)?)+) => {
        impl From<$ty> for Res<()> {
            fn from(value: $ty) -> Self {
                Res::new($code, format!($msg$(, value.$field)+), )
            }
        }
    };
}

erro_from_res!(io::Error, 400, "IoError: {}", this);
erro_from_res!(JwtError, 401, "身份认证失败: {}", this);
erro_from_res!(ParseError, 415, "数据解析失败: {}", this);