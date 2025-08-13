#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhoneModelExtensionAttributes {
    CameraModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
    ),
    PositioningDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
    ),
}
impl core::fmt::Display for InsertablePhoneModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CameraModel(e) => write!(f, "{e}"),
            Self::PositioningDeviceModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhoneModelAttributes {
    Extension(InsertablePhoneModelExtensionAttributes),
    Id,
}
impl core::str::FromStr for InsertablePhoneModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertablePhoneModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::phone_models::phone_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhoneModel {
    pub(crate) id: ::rosetta_uuid::Uuid,
}
impl InsertablePhoneModel {
    pub fn phone_models_camera<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::camera_models::CameraModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::camera_models::CameraModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn phone_models_positioning<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhoneModelBuilder<
    CameraModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    PositioningDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
            Option<::rosetta_uuid::Uuid>,
        >,
> {
    pub(crate) phone_models_camera: CameraModel,
    pub(crate) phone_models_positioning: PositioningDeviceModel,
}
impl<CameraModel, PositioningDeviceModel> web_common_traits::database::ExtendableBuilder
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    CameraModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
    >,
    PositioningDeviceModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
    >,
{
    type Attributes = InsertablePhoneModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.phone_models_camera = self
            .phone_models_camera
            .extend_builder(other.phone_models_camera)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertablePhoneModelAttributes::Extension(
                    InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                ))
            })?;
        Ok(self)
    }
}
impl<CameraModel, PositioningDeviceModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    CameraModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
    PositioningDeviceModel:
        web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.phone_models_camera = self.phone_models_camera.set_primary_key(primary_key);
        self.phone_models_positioning = self.phone_models_positioning.set_primary_key(primary_key);
        self
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `phone_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera =
            self.phone_models_camera.created_at(created_at).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.created_at`, `trackables.updated_at`
    /// columns from table `phone_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera = self
            .phone_models_camera
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `phone_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    {
        self.phone_models_camera =
            self.phone_models_camera.created_by(created_by).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `phone_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        Description: TryInto<Option<String>>,
        <Description as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera =
            self.phone_models_camera.description(description).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.id` column from table `phone_models`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera = self.phone_models_camera.id(id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePhoneModelAttributes::Extension(
                    InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `phone_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        Name: TryInto<Option<String>>,
        <Name as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera = self.phone_models_camera.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePhoneModelAttributes::Extension(
                    InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.name`, `trackables.description`
    /// columns from table `phone_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera =
            self.phone_models_camera.name_and_description(name, description).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `phone_models`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    {
        self.phone_models_camera = self.phone_models_camera.parent(parent_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertablePhoneModelAttributes::Extension(
                    InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.parent_id`, `trackables.id` columns
    /// from table `phone_models`.
    pub fn parent_and_id<Id>(
        mut self,
        parent_id: ::rosetta_uuid::Uuid,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera =
            self.phone_models_camera.parent_and_id(parent_id, id).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `phone_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    {
        self.phone_models_camera =
            self.phone_models_camera.photograph(photograph_id).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `phone_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_camera =
            self.phone_models_camera.updated_at(updated_at).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `phone_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    {
        self.phone_models_camera =
            self.phone_models_camera.updated_by(updated_by).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertablePhoneModelAttributes::Extension(
                        InsertablePhoneModelExtensionAttributes::CameraModel(attribute),
                    )
                })
            })?;
        Ok(self)
    }
}
impl<CameraModel, PositioningDeviceModel, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel,
            Error = web_common_traits::database::InsertError<InsertablePhoneModelAttributes>,
        >,
    CameraModel:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    PositioningDeviceModel:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertablePhoneModelAttributes;
    fn is_complete(&self) -> bool {
        self.phone_models_camera.is_complete() && self.phone_models_positioning.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::phone_models::PhoneModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
