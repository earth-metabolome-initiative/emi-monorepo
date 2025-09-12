#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumetricContainerExtensionAttribute {
    Container(crate::codegen::structs_codegen::tables::insertables::ContainerAttribute),
}
impl core::fmt::Display for VolumetricContainerExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Container(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ContainerAttribute>
    for VolumetricContainerExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
    ) -> Self {
        Self::Container(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumetricContainerAttribute {
    Extension(VolumetricContainerExtensionAttribute),
    Id,
    VolumetricContainerModel,
}
impl core::str::FromStr for VolumetricContainerAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VolumetricContainerModel" => Ok(Self::VolumetricContainerModel),
            "volumetric_container_model" => Ok(Self::VolumetricContainerModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
    > for VolumetricContainerAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::ContainerAttribute::Id.into(),
        )
    }
}
impl<PhysicalAsset>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
            PhysicalAsset,
        >,
    > for VolumetricContainerAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::ContainerAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for VolumetricContainerAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "volumetric_containers.id"),
            Self::VolumetricContainerModel => {
                write!(f, "volumetric_containers.volumetric_container_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricContainer {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) volumetric_container_model: i32,
}
impl InsertableVolumetricContainer {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::containers::Container, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::containers::Container:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::containers::Container::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn volumetric_containers_id_volumetric_container_model_fkey(
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
                        .eq(&self.volumetric_container_model),
                ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
    }
    pub fn volumetric_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.volumetric_container_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new [`VolumetricContainer`].
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::VolumetricContainer;
/// use core_structures::tables::insertables::AssetSettable;
/// use core_structures::tables::insertables::VolumetricContainerSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let volumetric_container = VolumetricContainer::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .most_concrete_table(most_concrete_table)?
///    .updated_by(updated_by)?
///    .volumetric_container_model(volumetric_container_model)?
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
pub struct InsertableVolumetricContainerBuilder<
    Container = crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
    >,
> {
    pub(crate) volumetric_container_model: Option<i32>,
    pub(crate) id: Container,
}
impl From<InsertableVolumetricContainerBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableVolumetricContainerBuilder,
    >
{
    fn from(builder: InsertableVolumetricContainerBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Container> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder<
        Container,
    >
where
    Container: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.volumetric_container_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `VolumetricContainer` or descendant tables.
pub trait VolumetricContainerSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.volumetric_containers.volumetric_container_model` column.
    ///
    /// # Arguments
    /// * `volumetric_container_model`: The value to set for the
    ///   `public.volumetric_containers.volumetric_container_model` column.
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
    fn volumetric_container_model<VCM>(
        self,
        volumetric_container_model: VCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        VCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    Container: crate::codegen::structs_codegen::tables::insertables::ContainerSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        > + crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        >,
> VolumetricContainerSettable for InsertableVolumetricContainerBuilder<Container>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute;
    /// Sets the value of the
    /// `public.volumetric_containers.volumetric_container_model` column.
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
    /// subgraph v5 ["`containers`"]
    ///    v0@{shape: rounded, label: "container_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`physical_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`volumetric_containers`"]
    ///    v2@{shape: rounded, label: "volumetric_container_model"}
    /// class v2 column-of-interest
    /// end
    /// v0 --->|"`ancestral same as`"| v3
    /// v0 -.->|"`inferred ancestral same as`"| v1
    /// v1 --->|"`ancestral same as`"| v3
    /// v2 --->|"`ancestral same as`"| v3
    /// v2 -.->|"`inferred ancestral same as`"| v0
    /// v2 -.->|"`inferred ancestral same as`"| v1
    /// v5 --->|"`extends`"| v6
    /// v6 --->|"`extends`"| v4
    /// v7 --->|"`extends`"| v5
    /// ```
    fn volumetric_container_model<VCM>(
        mut self,
        volumetric_container_model: VCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        VCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let volumetric_container_model =
            <VCM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &volumetric_container_model,
            );
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::ContainerSettable>::container_model(
                self.id,
                volumetric_container_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable>::model(
                self.id,
                volumetric_container_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.volumetric_container_model = Some(volumetric_container_model);
        Ok(self)
    }
}
impl<
    Container: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ContainerAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
for InsertableVolumetricContainerBuilder<Container>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute;
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
impl<Container> crate::codegen::structs_codegen::tables::insertables::ContainerSettable
for InsertableVolumetricContainerBuilder<Container>
where
    Self: crate::codegen::structs_codegen::tables::insertables::VolumetricContainerSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute;
    #[inline]
    ///Sets the value of the `public.containers.container_model` column.
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
    ///subgraph v3 ["`containers`"]
    ///    v0@{shape: rounded, label: "container_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v4 ["`physical_assets`"]
    ///    v2@{shape: rounded, label: "model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v5 ["`volumetric_containers`"]
    ///    v1@{shape: rounded, label: "volumetric_container_model"}
    ///class v1 directly-involved-column
    ///end
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v3 --->|"`extends`"| v4
    ///v5 --->|"`extends`"| v3
    ///```
    fn container_model<CM>(
        self,
        container_model: CM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as VolumetricContainerSettable>::volumetric_container_model(
            self,
            container_model,
        )
    }
}
impl<
    Container,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetSettable
for InsertableVolumetricContainerBuilder<Container>
where
    Self: crate::codegen::structs_codegen::tables::insertables::VolumetricContainerSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerAttribute;
    #[inline]
    ///Sets the value of the `public.physical_assets.model` column.
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
    ///    v2@{shape: rounded, label: "model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v5 ["`containers`"]
    ///    v3@{shape: rounded, label: "container_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v6 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v7 ["`volumetric_containers`"]
    ///    v1@{shape: rounded, label: "volumetric_container_model"}
    ///class v1 directly-involved-column
    ///end
    ///v3 --->|"`ancestral same as`"| v2
    ///v3 -.->|"`inferred ancestral same as`"| v0
    ///v0 --->|"`ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v5 --->|"`extends`"| v6
    ///v6 --->|"`extends`"| v4
    ///v7 --->|"`extends`"| v5
    ///```
    fn model<M>(
        self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as VolumetricContainerSettable>::volumetric_container_model(self, model)
    }
}
impl<Container> web_common_traits::database::MostConcreteTable
    for InsertableVolumetricContainerBuilder<Container>
where
    Container: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<Container> web_common_traits::prelude::SetPrimaryKey
    for InsertableVolumetricContainerBuilder<Container>
where
    Container: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Container, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableVolumetricContainerBuilder<Container>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        Error = web_common_traits::database::InsertError<VolumetricContainerAttribute>,
    >,
    Container: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attribute = VolumetricContainerAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
