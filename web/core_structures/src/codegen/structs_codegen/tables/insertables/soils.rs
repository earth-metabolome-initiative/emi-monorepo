#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SoilExtensionAttribute {
    SampleSource(crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute),
}
impl core::fmt::Display for SoilExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::SampleSource(e) => write!(f, "soils({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute>
    for SoilExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
    ) -> Self {
        Self::SampleSource(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for SoilExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SoilAttribute {
    Extension(SoilExtensionAttribute),
    Id,
    Model,
}
impl core::str::FromStr for SoilAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Model" => Ok(Self::Model),
            "model" => Ok(Self::Model),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder<T1>
{
    type Attribute = SoilAttribute;
}
impl core::fmt::Display for SoilAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "soils.id"),
            Self::Model => write!(f, "soils.model"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::soils::soils)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSoil {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) model: i32,
}
impl InsertableSoil {
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::soil_models::SoilModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::soil_models::SoilModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::soil_models::SoilModel::read(self.model, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Soil`](crate::codegen::structs_codegen::tables::soils::Soil).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Soil;
/// use core_structures::tables::insertables::AssetSettable;
/// use core_structures::tables::insertables::SoilSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let soil = Soil::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .model(model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .id(id)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .description(description)?
///    .name(name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableSoilBuilder<
    SampleSource
        = crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
            >,
        >,
> {
    pub(crate) model: Option<i32>,
    pub(crate) id: SampleSource,
}
impl<SampleSource> diesel::associations::HasTable for InsertableSoilBuilder<SampleSource> {
    type Table = crate::codegen::diesel_codegen::tables::soils::soils::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::soils::soils::table
    }
}
impl From<InsertableSoilBuilder>
    for web_common_traits::database::IdOrBuilder<::rosetta_uuid::Uuid, InsertableSoilBuilder>
{
    fn from(builder: InsertableSoilBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<SampleSource> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder<SampleSource>
where
    SampleSource: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Soil` or descendant
/// tables.
pub trait SoilSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.soils.model` column.
    ///
    /// # Arguments
    /// * `model`: The value to set for the `public.soils.model` column.
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
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    SampleSource: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
            >,
        > + crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
            >,
        >,
> SoilSettable for InsertableSoilBuilder<SampleSource>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.soils.model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`assets`"]
    ///    v3@{shape: rounded, label: "model"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`sample_sources`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`soils`"]
    ///    v2@{shape: rounded, label: "model"}
    /// class v2 column-of-interest
    /// end
    /// v0 --->|"`ancestral same as`"| v3
    /// v1 --->|"`ancestral same as`"| v3
    /// v1 -.->|"`inferred ancestral same as`"| v0
    /// v2 --->|"`ancestral same as`"| v3
    /// v2 -.->|"`inferred ancestral same as`"| v0
    /// v2 -.->|"`inferred ancestral same as`"| v1
    /// v5 --->|"`extends`"| v4
    /// v6 --->|"`extends`"| v5
    /// v7 --->|"`extends`"| v6
    /// ```
    fn model<M>(mut self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let model = <M as web_common_traits::database::PrimaryKeyLike>::primary_key(&model);
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
                self.id,
                model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable>::model(
                self.id,
                model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.model = Some(model);
        Ok(self)
    }
}
impl<
    SampleSource: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
    for InsertableSoilBuilder<SampleSource>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
            >,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.assets.id` column.
    fn id<I>(mut self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                self.id,
                id,
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
    /// Sets the value of the `public.assets.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
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
    /// Sets the value of the `public.assets.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
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
    /// Sets the value of the `public.assets.model` column.
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
    /// subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v3 ["`physical_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`ancestral same as`"| v0
    /// v3 --->|"`extends`"| v2
    /// ```
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
            self, model,
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
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
    /// Sets the value of the `public.assets.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
    /// Sets the value of the `public.assets.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
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
    /// Sets the value of the `public.assets.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
impl<SampleSource> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable
    for InsertableSoilBuilder<SampleSource>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::SoilSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
            >,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.physical_assets.model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`assets`"]
    ///    v2@{shape: rounded, label: "model"}
    /// class v2 undirectly-involved-column
    /// end
    /// subgraph v5 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v6 ["`sample_sources`"]
    ///    v3@{shape: rounded, label: "model"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v7 ["`soils`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v2
    /// v3 --->|"`ancestral same as`"| v2
    /// v3 -.->|"`inferred ancestral same as`"| v0
    /// v1 --->|"`ancestral same as`"| v2
    /// v1 -.->|"`inferred ancestral same as`"| v0
    /// v1 -.->|"`inferred ancestral same as`"| v3
    /// v5 --->|"`extends`"| v4
    /// v6 --->|"`extends`"| v5
    /// v7 --->|"`extends`"| v6
    /// ```
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as SoilSettable>::model(self, model)
    }
}
impl<SampleSource> crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable
    for InsertableSoilBuilder<SampleSource>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::SoilSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SoilAttribute,
            >,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.sample_sources.model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v3 ["`physical_assets`"]
    ///    v2@{shape: rounded, label: "model"}
    /// class v2 undirectly-involved-column
    /// end
    /// subgraph v4 ["`sample_sources`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`soils`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v0 -.->|"`inferred ancestral same as`"| v2
    /// v1 -.->|"`inferred ancestral same as`"| v2
    /// v1 -.->|"`inferred ancestral same as`"| v0
    /// v4 --->|"`extends`"| v3
    /// v5 --->|"`extends`"| v4
    /// ```
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as SoilSettable>::model(self, model)
    }
}
impl<SampleSource> web_common_traits::database::MostConcreteTable
    for InsertableSoilBuilder<SampleSource>
where
    SampleSource: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<SampleSource> web_common_traits::prelude::SetPrimaryKey for InsertableSoilBuilder<SampleSource>
where
    SampleSource: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<SampleSource, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableSoilBuilder<SampleSource>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::soils::Soil,
            Error = web_common_traits::database::InsertError<SoilAttribute>,
        >,
    SampleSource:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<SoilAttribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::soils::Soil =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
