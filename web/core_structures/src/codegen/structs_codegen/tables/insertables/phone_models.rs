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
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes>
    for InsertablePhoneModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
    ) -> Self {
        Self::CameraModel(attribute)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
> for InsertablePhoneModelExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
    ) -> Self {
        Self::PositioningDeviceModel(attribute)
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
    pub(crate) id: i32,
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
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    PositioningDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) phone_models_camera: CameraModel,
    pub(crate) phone_models_positioning: PositioningDeviceModel,
}
/// Trait defining setters for attributes of an instance of `PhoneModel` or
/// descendant tables.
pub trait PhoneModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable
    + crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable
{
}
impl PhoneModelBuildable for Option<i32> {}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
        >,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
        >,
> PhoneModelBuildable
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel> {}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
        >,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<'N, N>(
        mut self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'N N as TryInto<Option<String>>>::Error,
        >,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.phone_models_camera,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<'D, D>(
        mut self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'D D as TryInto<Option<String>>>::Error,
        >,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.phone_models_camera,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model_id` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model_id"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model(
        self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
            self,
            parent_model_id,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.phone_models_camera,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<'CA, CA>(
        mut self,
        created_at: &'CA CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'CA CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'CA CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.phone_models_camera,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.phone_models_camera,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<'UA, UA>(
        mut self,
        updated_at: &'UA UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'UA UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'UA UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.phone_models_camera,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
        >,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel> {}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
        >,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.phone_models_camera,
                parent_model_id,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::CameraModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes,
        >,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelBuildable
for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel> {}
impl<CameraModel, PositioningDeviceModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    CameraModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.phone_models_camera = self.phone_models_camera.set_primary_key(primary_key);
        self.phone_models_positioning = self.phone_models_positioning.set_primary_key(primary_key);
        self
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
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
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
