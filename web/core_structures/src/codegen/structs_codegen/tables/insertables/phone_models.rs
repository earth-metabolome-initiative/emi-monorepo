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
            Self::CameraModel(e) => write!(f, "CameraModel.{e}"),
            Self::PositioningDeviceModel(e) => write!(f, "PositioningDeviceModel.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhoneModelAttributes {
    Extension(InsertablePhoneModelExtensionAttributes),
    Id,
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
    pub fn phone_models_id_fkey<C: diesel::connection::LoadConnection>(
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
    pub fn phone_models_id_fkey1<C: diesel::connection::LoadConnection>(
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
    PositioningDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
            Option<::rosetta_uuid::Uuid>,
        >,
> {
    pub(crate) phone_models_id_fkey: CameraModel,
    pub(crate) phone_models_id_fkey1: PositioningDeviceModel,
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
        self.phone_models_id_fkey = self
            .phone_models_id_fkey
            .extend_builder(other.phone_models_id_fkey)
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
        self.phone_models_id_fkey = self.phone_models_id_fkey.set_primary_key(primary_key);
        self.phone_models_id_fkey1 = self.phone_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `phone_models`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.created_at(created_at).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
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
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.created_by(created_by).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `phone_models`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.description(description).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.id` column from table `phone_models`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_id_fkey = self.phone_models_id_fkey.id(id).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `phone_models`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_id_fkey = self.phone_models_id_fkey.name(name).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
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
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.photograph(photograph_id).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
        >,
        PositioningDeviceModel,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `phone_models`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertablePhoneModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.updated_at(updated_at).map_err(|e| {
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
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
            >,
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
        self.phone_models_id_fkey =
            self.phone_models_id_fkey.updated_by(updated_by).map_err(|e| {
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
        self.phone_models_id_fkey.is_complete() && self.phone_models_id_fkey1.is_complete()
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
