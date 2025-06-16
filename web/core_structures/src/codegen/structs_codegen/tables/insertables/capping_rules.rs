#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCappingRuleAttributes {
    ContainerId,
    CapId,
}
impl core::fmt::Display for InsertableCappingRuleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCappingRuleAttributes::ContainerId => write!(f, "container_id"),
            InsertableCappingRuleAttributes::CapId => write!(f, "cap_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::capping_rules::capping_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCappingRule {
    container_id: ::rosetta_uuid::Uuid,
    cap_id: ::rosetta_uuid::Uuid,
}
impl InsertableCappingRule {
    pub fn container<C: diesel::connection::LoadConnection>(
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
                self.container_id,
            ),
            conn,
        )
    }
    pub fn cap<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.cap_id,
            ),
            conn,
        )
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCappingRuleBuilder {
    pub(crate) container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) cap_id: Option<::rosetta_uuid::Uuid>,
}
impl InsertableCappingRuleBuilder {
    pub fn container_id<P>(
        mut self,
        container_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCappingRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let container_id = container_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableCappingRuleAttributes::ContainerId)
            },
        )?;
        if let Some(cap_id) = self.cap_id {
            pgrx_validation::must_be_distinct_uuid(container_id, cap_id).map_err(|e| {
                e.rename_fields(
                    InsertableCappingRuleAttributes::ContainerId,
                    InsertableCappingRuleAttributes::CapId,
                )
            })?;
        }
        self.container_id = Some(container_id);
        Ok(self)
    }
    pub fn cap_id<P>(
        mut self,
        cap_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCappingRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let cap_id =
            cap_id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableCappingRuleAttributes::CapId)
            })?;
        if let Some(container_id) = self.container_id {
            pgrx_validation::must_be_distinct_uuid(container_id, cap_id).map_err(|e| {
                e.rename_fields(
                    InsertableCappingRuleAttributes::ContainerId,
                    InsertableCappingRuleAttributes::CapId,
                )
            })?;
        }
        self.cap_id = Some(cap_id);
        Ok(self)
    }
}
impl TryFrom<InsertableCappingRuleBuilder> for InsertableCappingRule {
    type Error = common_traits::prelude::BuilderError<InsertableCappingRuleAttributes>;
    fn try_from(
        builder: InsertableCappingRuleBuilder,
    ) -> Result<InsertableCappingRule, Self::Error> {
        Ok(Self {
            container_id: builder.container_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCappingRuleAttributes::ContainerId,
                ),
            )?,
            cap_id: builder.cap_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCappingRuleAttributes::CapId,
            ))?,
        })
    }
}
