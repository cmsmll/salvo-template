use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use salvo::{
    Request,
    http::headers::{Authorization, HeaderMapExt, authorization::Bearer},
};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use crate::{res, resp::Res};

/// 秘钥
#[derive(Clone)]
pub struct JwtConfig {
    header: Header,
    duration: i64,
    validation: Validation,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtConfig {
    pub fn new(secret: &str, duration: i64) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            validation: Validation::default(),
            header: Header::default(),
            duration,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims<T> {
    exp: i64,
    data: T,
}

pub trait JwtToken
where
    Self: Serialize + for<'a> Deserialize<'a> + Send + Sync + Clone + 'static,
{
    fn config() -> &'static JwtConfig;

    fn parse(req: &mut Request) -> Result<Self, Res> {
        if let Some(value) = req.extensions_mut().remove::<Self>() {
            return Ok(value);
        }
        let token = req
            .headers()
            .typed_get::<Authorization<Bearer>>()
            .ok_or(res!(401, "身份认证失败: 请求未携带有效token"))?;
        Ok(Self::decode(token.token())?)
    }

    fn encode(self) -> jsonwebtoken::errors::Result<String> {
        let config = Self::config();
        let exp = UtcDateTime::now().unix_timestamp() + config.duration;
        let claims = Claims { data: self, exp };
        jsonwebtoken::encode(&config.header, &claims, &config.encoding_key)
    }

    fn decode(token: &str) -> jsonwebtoken::errors::Result<Self> {
        let config = Self::config();
        let data = jsonwebtoken::decode::<Claims<Self>>(token, &config.decoding_key, &config.validation)?;
        Ok(data.claims.data)
    }
}