#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrganismExtensionAttribute {
    SampleSource(crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute),
}
impl core::fmt::Display for OrganismExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::SampleSource(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute>
    for OrganismExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
    ) -> Self {
        Self::SampleSource(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrganismAttribute {
    Extension(OrganismExtensionAttribute),
    Id,
}
impl core::str::FromStr for OrganismAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for OrganismAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "organisms.id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::organisms::organisms)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganism {
    pub(crate) id: ::rosetta_uuid::Uuid,
}
impl InsertableOrganism {
    pub fn id<C: diesel::connection::LoadConnection>(
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
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(self.id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismBuilder<
    SampleSource
        = crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
            >,
        >,
> {
    pub(crate) id: SampleSource,
}
impl From<InsertableOrganismBuilder>
    for web_common_traits::database::IdOrBuilder<::rosetta_uuid::Uuid, InsertableOrganismBuilder>
{
    fn from(builder: InsertableOrganismBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<SampleSource> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        SampleSource,
    >
where
    SampleSource: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `Organism` or
/// descendant tables.
pub trait OrganismSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
}
impl<SampleSource> OrganismSettable for InsertableOrganismBuilder<SampleSource> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute;
}
impl<
    SampleSource: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
for InsertableOrganismBuilder<SampleSource>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id(
        mut self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
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
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
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
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
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
    fn model(
        self,
        model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
            self,
            model,
        )
    }
    #[inline]
    ///Sets the value of the `public.assets.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
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
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
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
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
impl<SampleSource> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable
    for InsertableOrganismBuilder<SampleSource>
where
    Self: crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
        >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute;
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
    /// subgraph v5 ["`sample_sources`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v2
    /// v1 --->|"`ancestral same as`"| v2
    /// v1 -.->|"`inferred ancestral same as`"| v0
    /// v4 --->|"`extends`"| v3
    /// v5 --->|"`extends`"| v4
    /// ```
    fn model(
        self,
        model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable>::model(
            self, model,
        )
    }
}
impl<
    SampleSource: crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable
for InsertableOrganismBuilder<SampleSource> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::OrganismAttribute;
    #[inline]
    ///Sets the value of the `public.sample_sources.model` column.
    fn model(
        mut self,
        model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <SampleSource as crate::codegen::structs_codegen::tables::insertables::SampleSourceSettable>::model(
                self.id,
                model,
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
impl<SampleSource> web_common_traits::database::MostConcreteTable
    for InsertableOrganismBuilder<SampleSource>
where
    SampleSource: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<SampleSource> web_common_traits::prelude::SetPrimaryKey
    for InsertableOrganismBuilder<SampleSource>
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
    for InsertableOrganismBuilder<SampleSource>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::organisms::Organism,
            Error = web_common_traits::database::InsertError<OrganismAttribute>,
        >,
    SampleSource:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = OrganismAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::organisms::Organism =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
