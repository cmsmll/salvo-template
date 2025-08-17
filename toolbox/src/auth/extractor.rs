use derive_more::{Deref, DerefMut};
use salvo::{Extractible, Request, Writer};

use crate::{auth::jwt_config::JwtToken, global::METADATE};

#[derive(Debug, Deref, DerefMut)]
pub struct Jwt<T: JwtToken>(pub T);

impl<'ex, T: JwtToken> Extractible<'ex> for Jwt<T> {
    fn metadata() -> &'ex salvo::extract::Metadata {
        &METADATE
    }

    async fn extract(req: &'ex mut Request) -> Result<Self, impl Writer + Send + std::fmt::Debug + 'static> {
        T::parse(req).map(Self)
    }
}
