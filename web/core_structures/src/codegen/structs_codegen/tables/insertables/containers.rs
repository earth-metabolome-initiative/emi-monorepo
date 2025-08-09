#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerExtensionAttributes {
    Trackable(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
}
impl core::fmt::Display for InsertableContainerExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Trackable(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerAttributes {
    Extension(InsertableContainerExtensionAttributes),
    Id,
    ContainerModelId,
}
impl core::fmt::Display for InsertableContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ContainerModelId => write!(f, "container_model_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::containers::containers)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainer {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) container_model_id: ::rosetta_uuid::Uuid,
}
impl InsertableContainer {
    pub fn id<C: diesel::connection::LoadConnection>(
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
                self.id,
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerBuilder<
    Trackable = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
> {
    pub(crate) container_model_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) id: Trackable,
}
impl<Trackable> web_common_traits::database::ExtendableBuilder
for InsertableContainerBuilder<Trackable>
where
    Trackable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    >,
{
    type Attributes = InsertableContainerAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                ))
            })?;
        if let Some(container_model_id) = other.container_model_id {
            self = self.container_model(container_model_id)?;
        }
        Ok(self)
    }
}
impl<Trackable> web_common_traits::prelude::SetPrimaryKey for InsertableContainerBuilder<Trackable>
where
    Trackable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<Trackable>
{
    /// Sets the value of the `containers.container_model_id` column from table
    /// `containers`.
    pub fn container_model(
        mut self,
        container_model_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>> {
        self.container_model_id = Some(container_model_id);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `containers`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at`, `trackables.updated_at`
    /// columns from table `containers`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at_and_updated_at(created_at, updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `containers`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>> {
        self.id = self.id.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `containers`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        Description: TryInto<Option<String>>,
        <Description as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.id` column from table `containers`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name` column from table `containers`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        Name: TryInto<Option<String>>,
        <Name as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name`, `trackables.description`
    /// columns from table `containers`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name_and_description(name, description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `containers`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>> {
        self.id = self.id.parent(parent_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id`, `trackables.id` columns
    /// from table `containers`.
    pub fn parent_and_id<Id>(
        mut self,
        parent_id: ::rosetta_uuid::Uuid,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.parent_and_id(parent_id, id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `containers`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>> {
        self.id = self.id.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `containers`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>>
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `containers`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerAttributes>> {
        self.id = self.id.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableContainerAttributes::Extension(
                    InsertableContainerExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<Trackable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableContainerBuilder<Trackable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::containers::Container,
            Error = web_common_traits::database::InsertError<InsertableContainerAttributes>,
        >,
    Trackable: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableContainerAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.container_model_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::containers::Container =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
