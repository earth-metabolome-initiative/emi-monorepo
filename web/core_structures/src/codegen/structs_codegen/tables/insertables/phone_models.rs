#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhoneModelExtensionAttribute {
    CameraModel(crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute),
    PositioningDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
    ),
}
impl core::fmt::Display for PhoneModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CameraModel(e) => write!(f, "phone_models({e})"),
            Self::PositioningDeviceModel(e) => write!(f, "phone_models({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute>
    for PhoneModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
    ) -> Self {
        Self::CameraModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute>
    for PhoneModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
    ) -> Self {
        Self::PositioningDeviceModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for PhoneModelExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhoneModelAttribute {
    Extension(PhoneModelExtensionAttribute),
    Id,
}
impl core::str::FromStr for PhoneModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned()))
    }
}
impl<T1, T2> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<T1, T2>
{
    type Attribute = PhoneModelAttribute;
}
impl core::fmt::Display for PhoneModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "phone_models.id"),
        }
    }
}
#[derive(Debug)]
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
impl InsertablePhoneModel {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`PhoneModel`](crate::codegen::structs_codegen::tables::phone_models::PhoneModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::PhoneModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::PhysicalAssetModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let phone_model = PhoneModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
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
impl<CameraModel, PositioningDeviceModel> diesel::associations::HasTable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
{
    type Table = crate::codegen::diesel_codegen::tables::phone_models::phone_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::phone_models::phone_models::table
    }
}
impl From<InsertablePhoneModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertablePhoneModelBuilder>
{
    fn from(builder: InsertablePhoneModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<CameraModel, PositioningDeviceModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        CameraModel,
        PositioningDeviceModel,
    >
where
    CameraModel: common_traits::builder::IsCompleteBuilder,
    PositioningDeviceModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.phone_models_camera.is_complete() && self.phone_models_positioning.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `PhoneModel` or
/// descendant tables.
pub trait PhoneModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
}
impl<CameraModel, PositioningDeviceModel> PhoneModelSettable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
            >,
        >,
    PositioningDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
            >,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.asset_models.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.phone_models_camera,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.asset_models.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.phone_models_camera,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.asset_models.parent_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`ancestral same as`"| v0
    /// v3 --->|"`extends`"| v2
    /// ```
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
            self,
            parent_model,
        )
    }
    #[inline]
    /// Sets the value of the `public.asset_models.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.phone_models_camera,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.asset_models.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.phone_models_camera,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.phone_models_camera,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.phone_models_camera,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<CameraModel, PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::CameraModelSettable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<
    CameraModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
            >,
        >,
    PositioningDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.physical_asset_models.parent_model`
    /// column.
    fn parent_model<PM>(mut self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.phone_models_camera = <CameraModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.phone_models_camera,
                parent_model,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<CameraModel, PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelSettable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<CameraModel, PositioningDeviceModel> web_common_traits::database::MostConcreteTable
    for InsertablePhoneModelBuilder<CameraModel, PositioningDeviceModel>
where
    CameraModel: web_common_traits::database::MostConcreteTable,
    PositioningDeviceModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.phone_models_camera.set_most_concrete_table(table_name);
        self.phone_models_positioning.set_most_concrete_table(table_name);
    }
}
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
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel,
            Error = web_common_traits::database::InsertError<PhoneModelAttribute>,
        >,
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<PhoneModelAttribute>>
    {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::phone_models::PhoneModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
