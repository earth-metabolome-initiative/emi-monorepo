#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::step_models::StepModel,
>
for crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
        diesel::result::Error,
    > {
        self.id(conn).await
    }
}
#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Foreign<
    crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
>
for crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory {
    type Conn = diesel_async::AsyncPgConnection;
    async fn foreign(
        &self,
        conn: &mut Self::Conn,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
        diesel::result::Error,
    > {
        self.trackable_category(conn).await
    }
}
