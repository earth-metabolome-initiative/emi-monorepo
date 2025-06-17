#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugableContainerModelAttributes {
    CentrifugedWith,
    ContainerModelId,
}
impl core::fmt::Display for InsertableCentrifugableContainerModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCentrifugableContainerModelAttributes::CentrifugedWith => {
                write!(f, "centrifuged_with")
            }
            InsertableCentrifugableContainerModelAttributes::ContainerModelId => {
                write!(f, "container_model_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::centrifugable_container_models::centrifugable_container_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugableContainerModel {
    centrifuged_with: ::rosetta_uuid::Uuid,
    container_model_id: ::rosetta_uuid::Uuid,
}
impl InsertableCentrifugableContainerModel {
    pub fn centrifuged_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::table(
                ),
                self.centrifuged_with,
            ),
            conn,
        )
    }
    pub fn container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::container_models::ContainerModel::table(),
                self.container_model_id,
            ),
            conn,
        )
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugableContainerModelBuilder {
    pub(crate) centrifuged_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) container_model_id: Option<::rosetta_uuid::Uuid>,
}
impl InsertableCentrifugableContainerModelBuilder {
    pub fn centrifuged_with<P>(
        mut self,
        centrifuged_with: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugableContainerModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let centrifuged_with = centrifuged_with.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugableContainerModelAttributes::CentrifugedWith)
            },
        )?;
        self.centrifuged_with = Some(centrifuged_with);
        Ok(self)
    }
    pub fn container_model_id<P>(
        mut self,
        container_model_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableCentrifugableContainerModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let container_model_id = container_model_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugableContainerModelAttributes::ContainerModelId)
            },
        )?;
        self.container_model_id = Some(container_model_id);
        Ok(self)
    }
}
impl TryFrom<InsertableCentrifugableContainerModelBuilder>
    for InsertableCentrifugableContainerModel
{
    type Error =
        common_traits::prelude::BuilderError<InsertableCentrifugableContainerModelAttributes>;
    fn try_from(
        builder: InsertableCentrifugableContainerModelBuilder,
    ) -> Result<InsertableCentrifugableContainerModel, Self::Error> {
        Ok(Self {
            centrifuged_with: builder.centrifuged_with.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugableContainerModelAttributes::CentrifugedWith,
                ),
            )?,
            container_model_id: builder.container_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugableContainerModelAttributes::ContainerModelId,
                ),
            )?,
        })
    }
}
