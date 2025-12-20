//! Submodule implementing the `From<Ident>` trait to convert an `Ident` into
//! an `InternalToken`.

use common_traits::prelude::Builder;
use proc_macro2::Ident;
use quote::ToTokens;

use super::InternalToken;

impl From<Ident> for InternalToken {
    fn from(ident: Ident) -> Self {
        InternalToken::new()
            .private()
            .stream(ident.into_token_stream())
            .build()
            .expect("Failed to build InternalToken from Ident")
    }
}
