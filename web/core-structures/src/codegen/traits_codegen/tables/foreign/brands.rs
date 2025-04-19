#[cfg(feature = "postgres")]
impl
    web_common_traits::prelude::Foreign<
        crate::codegen::structs_codegen::tables::brand_states::BrandState,
    > for crate::codegen::structs_codegen::tables::brands::Brand
{
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::brand_states::BrandState,
        diesel::result::Error,
    > {
        self.brand_state(conn).await
    }
}
