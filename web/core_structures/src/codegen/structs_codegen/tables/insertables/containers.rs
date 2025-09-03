#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerExtensionAttributes {
    PhysicalAsset(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes,
    ),
}
impl core::fmt::Display for InsertableContainerExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PhysicalAsset(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes>
    for InsertableContainerExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes,
    ) -> Self {
        Self::PhysicalAsset(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerAttributes {
    Extension(InsertableContainerExtensionAttributes),
    Id,
    ContainerModel,
}
impl core::str::FromStr for InsertableContainerAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ContainerModel" => Ok(Self::ContainerModel),
            "container_model" => Ok(Self::ContainerModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ContainerModel => write!(f, "container_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::containers::containers)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainer {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) container_model: i32,
}
impl InsertableContainer {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn container_model<C: diesel::connection::LoadConnection>(
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
                self.container_model,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerBuilder<
    PhysicalAsset
        = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
> {
    pub(crate) container_model: Option<i32>,
    pub(crate) id: PhysicalAsset,
}
/// Trait defining setters for attributes of an instance of `Container` or
/// descendant tables.
pub trait ContainerBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.containers.container_model` column.
    ///
    /// # Arguments
    /// * `container_model`: The value to set for the
    ///   `public.containers.container_model` column.
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
    fn container_model(
        self,
        container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    PhysicalAsset: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes,
        >,
> ContainerBuildable for InsertableContainerBuilder<PhysicalAsset> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes;
    ///Sets the value of the `public.containers.container_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`assets`"]
    ///    v2@{shape: rounded, label: "model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`containers`"]
    ///    v0@{shape: rounded, label: "container_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v2
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v4 --->|"`extends`"| v5
    ///v4 -.->|"`descendant of`"| v3
    ///v5 --->|"`extends`"| v3
    ///```
    fn container_model(
        mut self,
        container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable>::model(
                self.id,
                container_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.container_model = Some(container_model);
        Ok(self)
    }
}
impl<
    PhysicalAsset: crate::codegen::structs_codegen::tables::insertables::AssetBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetBuildable
for InsertableContainerBuilder<PhysicalAsset>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id(
        mut self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::id(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::name(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::description(
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
    ///flowchart LR
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
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable>::model(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_by(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_at(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_by(
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
        self.id = <PhysicalAsset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_at(
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
impl<
    PhysicalAsset,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable
for InsertableContainerBuilder<PhysicalAsset>
where
    Self: crate::codegen::structs_codegen::tables::insertables::ContainerBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes;
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
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`assets`"]
    ///    v2@{shape: rounded, label: "model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`containers`"]
    ///    v1@{shape: rounded, label: "container_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_assets`"]
    ///    v0@{shape: rounded, label: "model"}
    ///class v0 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v4 --->|"`extends`"| v5
    ///v4 -.->|"`descendant of`"| v3
    ///v5 --->|"`extends`"| v3
    ///```
    fn model(
        self,
        model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as ContainerBuildable>::container_model(self, model)
    }
}
impl<PhysicalAsset> web_common_traits::database::MostConcreteTable
    for InsertableContainerBuilder<PhysicalAsset>
where
    PhysicalAsset: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<PhysicalAsset> web_common_traits::prelude::SetPrimaryKey
    for InsertableContainerBuilder<PhysicalAsset>
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
    for InsertableContainerBuilder<PhysicalAsset>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::containers::Container,
            Error = web_common_traits::database::InsertError<InsertableContainerAttributes>,
        >,
    PhysicalAsset:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableContainerAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.container_model.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::containers::Container =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
