#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStorageContainerAttributes {
    Id,
    ContainerModelId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStorageContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStorageContainerAttributes::Id => write!(f, "id"),
            InsertableStorageContainerAttributes::ContainerModelId => {
                write!(f, "container_model_id")
            }
            InsertableStorageContainerAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStorageContainerAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::storage_containers::storage_containers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageContainer {
    id: ::rosetta_uuid::Uuid,
    container_model_id: i32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableStorageContainer {
    #[cfg(feature = "postgres")]
    pub async fn container_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_models::container_models::dsl::id
                    .eq(&self.container_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::container_models::ContainerModel>(
                conn,
            )
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
pub struct InsertableStorageContainerBuilder {
    id: Option<::rosetta_uuid::Uuid>,
    container_model_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStorageContainerBuilder {
    fn default() -> Self {
        Self {
            id: None,
            container_model_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStorageContainerBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableStorageContainerAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn container_model_id<P>(
        mut self,
        container_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let container_model_id =
            container_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(InsertableStorageContainerAttributes::ContainerModelId)
            })?;
        self.container_model_id = Some(container_model_id);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableStorageContainerAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableStorageContainerAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStorageContainerBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStorageContainerAttributes>;
    type Object = InsertableStorageContainer;
    type Attribute = InsertableStorageContainerAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStorageContainerAttributes::Id,
            ))?,
            container_model_id: self.container_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageContainerAttributes::ContainerModelId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageContainerAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStorageContainerAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStorageContainer> for InsertableStorageContainerBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStorageContainer) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .container_model_id(insertable_variant.container_model_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
