#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStorageRuleAttributes {
    ParentContainerId,
    ChildContainerId,
    Quantity,
}
impl core::fmt::Display for InsertableStorageRuleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStorageRuleAttributes::ParentContainerId => {
                write!(f, "parent_container_id")
            }
            InsertableStorageRuleAttributes::ChildContainerId => {
                write!(f, "child_container_id")
            }
            InsertableStorageRuleAttributes::Quantity => write!(f, "quantity"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::storage_rules::storage_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageRule {
    parent_container_id: ::rosetta_uuid::Uuid,
    child_container_id: ::rosetta_uuid::Uuid,
    quantity: i16,
}
impl InsertableStorageRule {
    pub fn parent_container<C: diesel::connection::LoadConnection>(
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
                self.parent_container_id,
            ),
            conn,
        )
    }
    pub fn child_container<C: diesel::connection::LoadConnection>(
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
                self.child_container_id,
            ),
            conn,
        )
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageRuleBuilder {
    pub(crate) parent_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) child_container_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) quantity: Option<i16>,
}
impl InsertableStorageRuleBuilder {
    pub fn parent_container_id<P>(
        mut self,
        parent_container_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableStorageRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_container_id = parent_container_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableStorageRuleAttributes::ParentContainerId)
            },
        )?;
        if let Some(child_container_id) = self.child_container_id {
            pgrx_validation::must_be_distinct_uuid(parent_container_id, child_container_id)
                .map_err(|e| {
                    e.rename_fields(
                        InsertableStorageRuleAttributes::ParentContainerId,
                        InsertableStorageRuleAttributes::ChildContainerId,
                    )
                })?;
        }
        self.parent_container_id = Some(parent_container_id);
        Ok(self)
    }
    pub fn child_container_id<P>(
        mut self,
        child_container_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableStorageRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let child_container_id = child_container_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableStorageRuleAttributes::ChildContainerId)
            },
        )?;
        if let Some(parent_container_id) = self.parent_container_id {
            pgrx_validation::must_be_distinct_uuid(parent_container_id, child_container_id)
                .map_err(|e| {
                    e.rename_fields(
                        InsertableStorageRuleAttributes::ParentContainerId,
                        InsertableStorageRuleAttributes::ChildContainerId,
                    )
                })?;
        }
        self.child_container_id = Some(child_container_id);
        Ok(self)
    }
    pub fn quantity<P>(
        mut self,
        quantity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableStorageRuleAttributes>>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let quantity = quantity.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableStorageRuleAttributes::Quantity)
        })?;
        pgrx_validation::must_be_strictly_positive_i16(quantity)
            .map_err(|e| e.rename_field(InsertableStorageRuleAttributes::Quantity))?;
        self.quantity = Some(quantity);
        Ok(self)
    }
}
impl TryFrom<InsertableStorageRuleBuilder> for InsertableStorageRule {
    type Error = common_traits::prelude::BuilderError<InsertableStorageRuleAttributes>;
    fn try_from(
        builder: InsertableStorageRuleBuilder,
    ) -> Result<InsertableStorageRule, Self::Error> {
        Ok(Self {
            parent_container_id: builder.parent_container_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageRuleAttributes::ParentContainerId,
                ),
            )?,
            child_container_id: builder.child_container_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageRuleAttributes::ChildContainerId,
                ),
            )?,
            quantity: builder.quantity.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageRuleAttributes::Quantity,
                ),
            )?,
        })
    }
}
