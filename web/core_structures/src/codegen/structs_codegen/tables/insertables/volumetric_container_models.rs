#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricContainerModelExtensionAttributes {
    ContainerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes,
    ),
}
impl core::fmt::Display for InsertableVolumetricContainerModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ContainerModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes>
    for InsertableVolumetricContainerModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes,
    ) -> Self {
        Self::ContainerModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricContainerModelAttributes {
    Extension(InsertableVolumetricContainerModelExtensionAttributes),
    Id,
    Liters,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes>
    for InsertableVolumetricContainerModelAttributes
{
    fn from(
        container_models: crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes,
    ) -> Self {
        Self::Extension(InsertableVolumetricContainerModelExtensionAttributes::ContainerModel(
            container_models,
        ))
    }
}
impl core::str::FromStr for InsertableVolumetricContainerModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Liters" => Ok(Self::Liters),
            "liters" => Ok(Self::Liters),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableVolumetricContainerModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Liters => write!(f, "liters"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricContainerModel {
    pub(crate) id: i32,
    pub(crate) liters: f32,
}
impl InsertableVolumetricContainerModel {
    pub fn id<C: diesel::connection::LoadConnection>(
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
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricContainerModelBuilder<
    ContainerModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) id: ContainerModel,
}
/// Trait defining setters for attributes of an instance of
/// `VolumetricContainerModel` or descendant tables.
pub trait VolumetricContainerModelBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.volumetric_container_models.liters`
    /// column.
    ///
    /// # Arguments
    /// * `liters`: The value to set for the
    ///   `public.volumetric_container_models.liters` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn liters<L>(
        self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>;
}
impl<ContainerModel> VolumetricContainerModelBuildable
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes;
    /// Sets the value of the `public.volumetric_container_models.liters`
    /// column.
    fn liters<L>(
        mut self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>,
    {
        let liters = liters.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableVolumetricContainerModelAttributes::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes::Liters,
                    )
            })?;
        self.liters = Some(liters);
        Ok(self)
    }
}
impl<
    ContainerModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.id,
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
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.id,
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
    ///Sets the value of the `public.asset_models.parent_model` column.
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
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
            self,
            parent_model,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.id,
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
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.id,
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
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.id,
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
impl<ContainerModel> crate::codegen::structs_codegen::tables::insertables::ContainerModelBuildable
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes;
}
impl<
    ContainerModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableVolumetricContainerModelBuilder<ContainerModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelAttributes;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
    fn parent_model(
        mut self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.id,
                parent_model,
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
impl<ContainerModel> web_common_traits::database::MostConcreteTable
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    ContainerModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<ContainerModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    ContainerModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<ContainerModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        Error = web_common_traits::database::InsertError<
            InsertableVolumetricContainerModelAttributes,
        >,
    >,
    ContainerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableVolumetricContainerModelAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.liters.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
