#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepStorageContainerAttributes {
    Id,
    StepId,
    StorageContainerId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStepStorageContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepStorageContainerAttributes::Id => write!(f, "id"),
            InsertableStepStorageContainerAttributes::StepId => write!(f, "step_id"),
            InsertableStepStorageContainerAttributes::StorageContainerId => {
                write!(f, "storage_container_id")
            }
            InsertableStepStorageContainerAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableStepStorageContainerAttributes::CreatedAt => {
                write!(f, "created_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_storage_containers::step_storage_containers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepStorageContainer {
    id: rosetta_uuid::Uuid,
    step_id: rosetta_uuid::Uuid,
    storage_container_id: rosetta_uuid::Uuid,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepStorageContainer {
    #[cfg(feature = "postgres")]
    pub async fn step(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::steps::Step, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::steps::Step::table()
            .filter(crate::codegen::diesel_codegen::tables::steps::steps::dsl::id.eq(&self.step_id))
            .first::<crate::codegen::structs_codegen::tables::steps::Step>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn storage_container(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::storage_containers::StorageContainer::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_containers::storage_containers::dsl::id
                    .eq(&self.storage_container_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableStepStorageContainerBuilder {
    id: Option<rosetta_uuid::Uuid>,
    step_id: Option<rosetta_uuid::Uuid>,
    storage_container_id: Option<rosetta_uuid::Uuid>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableStepStorageContainerBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_id(
        mut self,
        step_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_id = Some(step_id);
        Ok(self)
    }
    pub fn storage_container_id(
        mut self,
        storage_container_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.storage_container_id = Some(storage_container_id);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStepStorageContainerBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepStorageContainerAttributes>;
    type Object = InsertableStepStorageContainer;
    type Attribute = InsertableStepStorageContainerAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepStorageContainerAttributes::Id,
                )
            })?,
            step_id: self.step_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepStorageContainerAttributes::StepId,
                )
            })?,
            storage_container_id: self.storage_container_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepStorageContainerAttributes::StorageContainerId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepStorageContainerAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepStorageContainerAttributes::CreatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableStepStorageContainer> for InsertableStepStorageContainerBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepStorageContainer) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .step_id(insertable_variant.step_id)?
            .storage_container_id(insertable_variant.storage_container_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?)
    }
}
