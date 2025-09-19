#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleModelExtensionAttribute {
    PhysicalAssetModel(
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    ),
}
impl core::fmt::Display for SampleModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PhysicalAssetModel(e) => write!(f, "sample_models({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute>
    for SampleModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    ) -> Self {
        Self::PhysicalAssetModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for SampleModelExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleModelAttribute {
    Extension(SampleModelExtensionAttribute),
    Id,
    SampleSourceModel,
}
impl core::str::FromStr for SampleModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "SampleSourceModel" => Ok(Self::SampleSourceModel),
            "id" => Ok(Self::Id),
            "sample_source_model" => Ok(Self::SampleSourceModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder<T1>
{
    type Attribute = SampleModelAttribute;
}
impl core::fmt::Display for SampleModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "sample_models.id"),
            Self::SampleSourceModel => write!(f, "sample_models.sample_source_model"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::sample_models::sample_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSampleModel {
    pub(crate) id: i32,
    pub(crate) sample_source_model: i32,
}
impl InsertableSampleModel {
    pub fn sample_source_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel::read(
            self.sample_source_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`SampleModel`](crate::codegen::structs_codegen::tables::sample_models::SampleModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::SampleModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::PhysicalAssetModelSettable;
/// use core_structures::tables::insertables::SampleModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let sample_model = SampleModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .sample_source_model(sample_source_model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableSampleModelBuilder<
    PhysicalAssetModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
        >,
> {
    pub(crate) sample_source_model: Option<i32>,
    pub(crate) id: PhysicalAssetModel,
}
impl<PhysicalAssetModel> diesel::associations::HasTable
    for InsertableSampleModelBuilder<PhysicalAssetModel>
{
    type Table = crate::codegen::diesel_codegen::tables::sample_models::sample_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::sample_models::sample_models::table
    }
}
impl From<InsertableSampleModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableSampleModelBuilder>
{
    fn from(builder: InsertableSampleModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PhysicalAssetModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder<
        PhysicalAssetModel,
    >
where
    PhysicalAssetModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.sample_source_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `SampleModel` or
/// descendant tables.
pub trait SampleModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.sample_models.sample_source_model` column.
    ///
    /// # Arguments
    /// * `sample_source_model`: The value to set for the
    ///   `public.sample_models.sample_source_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn sample_source_model<SSM>(self, sample_source_model: SSM) -> Result<Self, Self::Error>
    where
        SSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<PhysicalAssetModel> SampleModelSettable for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.sample_models.sample_source_model` column.
    fn sample_source_model<SSM>(mut self, sample_source_model: SSM) -> Result<Self, Self::Error>
    where
        SSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let sample_source_model =
            <SSM as web_common_traits::database::PrimaryKeyLike>::primary_key(&sample_source_model);
        self.sample_source_model = Some(sample_source_model);
        Ok(self)
    }
}
impl<
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
    for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.id,
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
impl<
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
    for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.id,
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
impl<PhysicalAssetModel> web_common_traits::database::MostConcreteTable
    for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    PhysicalAssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<PhysicalAssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    PhysicalAssetModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<PhysicalAssetModel, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableSampleModelBuilder<PhysicalAssetModel>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::sample_models::SampleModel,
            Error = web_common_traits::database::InsertError<SampleModelAttribute>,
        >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<SampleModelAttribute>>
    {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::sample_models::SampleModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
