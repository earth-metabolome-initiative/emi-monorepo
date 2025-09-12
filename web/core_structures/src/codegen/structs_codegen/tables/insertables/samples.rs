#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleExtensionAttribute {
    PhysicalAsset(crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute),
}
impl core::fmt::Display for SampleExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PhysicalAsset(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute>
    for SampleExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
    ) -> Self {
        Self::PhysicalAsset(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SampleAttribute {
    Extension(SampleExtensionAttribute),
    Id,
    Model,
    SampleSource,
    SampleSourceModel,
}
impl core::str::FromStr for SampleAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Model" => Ok(Self::Model),
            "SampleSource" => Ok(Self::SampleSource),
            "SampleSourceModel" => Ok(Self::SampleSourceModel),
            "model" => Ok(Self::Model),
            "sample_source" => Ok(Self::SampleSource),
            "sample_source_model" => Ok(Self::SampleSourceModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
    > for SampleAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute::Id.into(),
        )
    }
}
impl<Asset>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<Asset>,
    > for SampleAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for SampleAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "samples.id"),
            Self::Model => write!(f, "samples.model"),
            Self::SampleSource => write!(f, "samples.sample_source"),
            Self::SampleSourceModel => write!(f, "samples.sample_source_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::samples::samples)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSample {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) model: i32,
    pub(crate) sample_source: ::rosetta_uuid::Uuid,
    pub(crate) sample_source_model: i32,
}
impl InsertableSample {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn samples_id_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::assets::Asset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id.eq(&self.id).and(
                    crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                        .eq(&self.model),
                ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
    }
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_models::SampleModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_models::SampleModel::read(self.model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn samples_model_sample_source_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::sample_models::SampleModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::sample_models::sample_models::dsl::id
                    .eq(&self.model)
                    .and(
                        crate::codegen::diesel_codegen::tables::sample_models::sample_models::dsl::sample_source_model
                            .eq(&self.sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::sample_models::SampleModel,
            >(conn)
    }
    pub fn sample_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(
            self.sample_source,
            conn,
        )
    }
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
    #[cfg(feature = "postgres")]
    pub fn samples_sample_source_sample_source_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::assets::Asset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id
                    .eq(&self.sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                            .eq(&self.sample_source_model),
                    ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Sample`](crate::codegen::structs_codegen::tables::samples::Sample).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Sample;
/// use core_structures::tables::insertables::AssetSettable;
/// use core_structures::tables::insertables::SampleSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let sample = Sample::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .most_concrete_table(most_concrete_table)?
///    .updated_by(updated_by)?
///    .model(model)?
///    .sample_source(sample_source)?
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
pub struct InsertableSampleBuilder<
    PhysicalAsset
        = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
> {
    pub(crate) model: Option<i32>,
    pub(crate) sample_source: Option<::rosetta_uuid::Uuid>,
    pub(crate) sample_source_model: Option<i32>,
    pub(crate) id: PhysicalAsset,
}
impl From<InsertableSampleBuilder>
    for web_common_traits::database::IdOrBuilder<::rosetta_uuid::Uuid, InsertableSampleBuilder>
{
    fn from(builder: InsertableSampleBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PhysicalAsset> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder<PhysicalAsset>
where
    PhysicalAsset: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
            && self.model.is_some()
            && self.sample_source.is_some()
            && (self.sample_source_model.is_some()
                || self.model.is_some()
                || self.sample_source.is_some())
    }
}
/// Trait defining setters for attributes of an instance of `Sample` or
/// descendant tables.
pub trait SampleSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.samples.model` column.
    ///
    /// # Arguments
    /// * `model`: The value to set for the `public.samples.model` column.
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
    fn model<M>(
        self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.samples.sample_source` column.
    ///
    /// # Arguments
    /// * `sample_source`: The value to set for the
    ///   `public.samples.sample_source` column.
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
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn sample_source<SS>(
        self,
        sample_source: SS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SS: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.samples.sample_source_model` column.
    ///
    /// # Arguments
    /// * `sample_source_model`: The value to set for the
    ///   `public.samples.sample_source_model` column.
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
    fn sample_source_model<SSM>(
        self,
        sample_source_model: SSM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    PhysicalAsset: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
> SampleSettable for InsertableSampleBuilder<PhysicalAsset> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute;
    ///Sets the value of the `public.samples.model` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v4 ["`assets`"]
    ///    v3@{shape: rounded, label: "model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v6 ["`samples`"]
    ///    v1@{shape: rounded, label: "model"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "sample_source_model"}
    ///class v2 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v3
    ///v1 --->|"`ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v1 -.->|"`foreign defines`"| v2
    ///v5 --->|"`extends`"| v4
    ///v6 --->|"`extends`"| v5
    ///```
    fn model<M>(
        mut self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let model = <M as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &model,
        );
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
                self.id,
                model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.model = Some(model);
        Ok(self)
    }
    ///Sets the value of the `public.samples.sample_source` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///v0@{shape: rounded, label: "sample_source"}
    ///class v0 column-of-interest
    ///v1@{shape: rounded, label: "sample_source_model"}
    ///class v1 directly-involved-column
    ///v0 -.->|"`foreign defines`"| v1
    ///```
    fn sample_source<SS>(
        mut self,
        sample_source: SS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SS: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let sample_source = <SS as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &sample_source,
        );
        self.sample_source = Some(sample_source);
        Ok(self)
    }
    ///Sets the value of the `public.samples.sample_source_model` column.
    fn sample_source_model<SSM>(
        mut self,
        sample_source_model: SSM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let sample_source_model = <SSM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &sample_source_model,
        );
        self.sample_source_model = Some(sample_source_model);
        Ok(self)
    }
}
impl<
    PhysicalAsset: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
for InsertableSampleBuilder<PhysicalAsset>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id<I>(
        mut self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                self.id,
                id,
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
    ///Sets the value of the `public.assets.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
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
    ///Sets the value of the `public.assets.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
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
    ///Sets the value of the `public.assets.model` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn model<M>(
        self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
            self,
            model,
        )
    }
    #[inline]
    ///Sets the value of the `public.assets.created_by` column.
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
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
    ///Sets the value of the `public.assets.created_at` column.
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
    ///Sets the value of the `public.assets.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
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
    ///Sets the value of the `public.assets.updated_at` column.
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
impl<PhysicalAsset> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable
    for InsertableSampleBuilder<PhysicalAsset>
where
    Self: crate::codegen::structs_codegen::tables::insertables::SampleSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
        >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute;
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
    /// subgraph v3 ["`assets`"]
    ///    v2@{shape: rounded, label: "model"}
    /// class v2 undirectly-involved-column
    /// end
    /// subgraph v4 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`samples`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v2
    /// v1 --->|"`ancestral same as`"| v2
    /// v1 -.->|"`inferred ancestral same as`"| v0
    /// v4 --->|"`extends`"| v3
    /// v5 --->|"`extends`"| v4
    /// ```
    fn model<M>(
        self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as SampleSettable>::model(self, model)
    }
}
impl<PhysicalAsset> web_common_traits::database::MostConcreteTable
    for InsertableSampleBuilder<PhysicalAsset>
where
    PhysicalAsset: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<PhysicalAsset> web_common_traits::prelude::SetPrimaryKey
    for InsertableSampleBuilder<PhysicalAsset>
where
    PhysicalAsset: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<PhysicalAsset, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableSampleBuilder<PhysicalAsset>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::samples::Sample,
            Error = web_common_traits::database::InsertError<SampleAttribute>,
        >,
    PhysicalAsset:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attribute = SampleAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::samples::Sample =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
