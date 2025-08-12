use std::sync::Arc;

use salvo::{
    Depot, Request, Response, Writer, async_trait,
    http::{HeaderValue, StatusCode, StatusError, header::CONTENT_TYPE},
};
use serde::Serialize;

pub type Resp<T> = Result<Res<T>, Res<()>>;

#[derive(Debug, Serialize)]
pub struct Res<T = ()> {
    info: Arc<str>,
    code: u16,
    data: T,
}

impl<T: Serialize> Res<T> {
    pub fn new(code: u16, info: Arc<str>, data: T) -> Self {
        Self { code, info, data }
    }
}

#[async_trait]
impl<T: Serialize + Send> Writer for Res<T> {
    async fn write(self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match serde_json::to_vec(&self) {
            Ok(bytes) => {
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                );
                res.status_code(StatusCode::from_u16(self.code).unwrap_or_else(|e| {
                    tracing::error!(error = ?e, "StatusCode write error");
                    StatusCode::BAD_REQUEST
                }));
                res.write_body(bytes).ok();
            }
            Err(e) => {
                tracing::error!(error = ?e, "JsonContent write error");
                res.render(StatusError::internal_server_error());
            }
        }
        if self.code >= 400 {
            depot.inject(self.info);
        }
    }
}
