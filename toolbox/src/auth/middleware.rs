use std::marker::PhantomData;

use salvo::{Depot, FlowCtrl, Handler, Request, Response, Writer, async_trait};

use crate::{
    auth::jwt_config::JwtToken,
    compare::{always_false, str::CompareStr},
};

#[derive(Debug)]
pub struct JwtAuth<T, A> {
    _marker: PhantomData<T>,
    allow: A,
}

impl<T: JwtToken, A: CompareStr> JwtAuth<T, A> {
    /// allow 返回 true 时免验证
    pub fn new(allow: A) -> Self {
        Self {
            allow,
            _marker: PhantomData::<T>,
        }
    }
}

impl<T: JwtToken> Default for JwtAuth<T, fn(&str) -> bool> {
    fn default() -> Self {
        Self::new(always_false)
    }
}

#[async_trait]
impl<T, A> Handler for JwtAuth<T, A>
where
    T: JwtToken + Send + Sync + 'static,
    A: CompareStr + Send + Sync + 'static,
{
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        if self.allow.compare(req.uri().path()) {
            return;
        }

        match T::parse(req) {
            Ok(value) => {
                req.extensions_mut().insert(value);
            }
            Err(err) => {
                err.write(req, depot, res).await;
                ctrl.skip_rest();
            }
        }
    }
}
