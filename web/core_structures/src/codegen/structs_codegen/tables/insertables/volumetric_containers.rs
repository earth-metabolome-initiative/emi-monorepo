#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricContainerExtensionAttributes {
    Container(crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes),
}
impl core::fmt::Display for InsertableVolumetricContainerExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Container(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes>
    for InsertableVolumetricContainerExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
    ) -> Self {
        Self::Container(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricContainerAttributes {
    Extension(InsertableVolumetricContainerExtensionAttributes),
    Id,
    VolumetricContainerModelId,
}
impl core::str::FromStr for InsertableVolumetricContainerAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VolumetricContainerModelId" => Ok(Self::VolumetricContainerModelId),
            "volumetric_container_model_id" => Ok(Self::VolumetricContainerModelId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableVolumetricContainerAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::VolumetricContainerModelId => {
                write!(f, "volumetric_container_model_id")
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
    pub(crate) volumetric_container_model_id: i32,
}
impl InsertableVolumetricContainer {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::containers::Container,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::containers::Container: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::containers::Container,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::containers::Container::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn volumetric_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.volumetric_container_model_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricContainerBuilder<
    Container = crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
    >,
> {
    pub(crate) volumetric_container_model_id: Option<i32>,
    pub(crate) id: Container,
}
/// Trait defining setters for attributes of an instance of
/// `VolumetricContainer` or descendant tables.
pub trait VolumetricContainerBuildable:
    crate::codegen::structs_codegen::tables::insertables::ContainerBuildable
{
    /// Sets the value of the
    /// `public.volumetric_containers.volumetric_container_model_id` column.
    ///
    /// # Arguments
    /// * `volumetric_container_model_id`: The value to set for the
    ///   `public.volumetric_containers.volumetric_container_model_id` column.
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
    fn volumetric_container_model(
        self,
        volumetric_container_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl VolumetricContainerBuildable for Option<::rosetta_uuid::Uuid> {
    fn volumetric_container_model(
        self,
        _volumetric_container_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    Container: crate::codegen::structs_codegen::tables::insertables::ContainerBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
        >,
> VolumetricContainerBuildable for InsertableVolumetricContainerBuilder<Container> {
    ///Sets the value of the `public.volumetric_containers.volumetric_container_model_id` column.
    fn volumetric_container_model(
        mut self,
        volumetric_container_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let volumetric_container_model_id = volumetric_container_model_id
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableVolumetricContainerAttributes::VolumetricContainerModelId,
                    )
            })?;
        self.volumetric_container_model_id = Some(volumetric_container_model_id);
        Ok(self)
    }
}
impl<
    Container: crate::codegen::structs_codegen::tables::insertables::ContainerBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetBuildable
for InsertableVolumetricContainerBuilder<Container> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerAttributes;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id(
        mut self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::id(
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
    ///Sets the value of the `public.assets.most_concrete_table` column.
    fn most_concrete_table<MCT>(
        mut self,
        most_concrete_table: MCT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MCT: TryInto<String>,
        validation_errors::SingleFieldError: From<<MCT as TryInto<String>>::Error>,
    {
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::most_concrete_table(
                self.id,
                most_concrete_table,
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::name(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::description(
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
    ///Sets the value of the `public.assets.model_id` column.
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
    ///    v0@{shape: rounded, label: "model_id"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`volumetric_containers`"]
    ///    v1@{shape: rounded, label: "volumetric_container_model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 -.->|"`descendant of`"| v2
    ///```
    fn model(
        self,
        model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as VolumetricContainerBuildable>::volumetric_container_model(
            self,
            model_id,
        )
    }
    #[inline]
    ///Sets the value of the `public.assets.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_by(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_at(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_by(
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
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_at(
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
    Container: crate::codegen::structs_codegen::tables::insertables::ContainerBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ContainerBuildable
for InsertableVolumetricContainerBuilder<Container> {
    #[inline]
    ///Sets the value of the `public.containers.container_model_id` column.
    fn container_model(
        mut self,
        container_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::ContainerBuildable>::container_model(
                self.id,
                container_model_id,
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
    Container: crate::codegen::structs_codegen::tables::insertables::ContainerBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableContainerAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable
for InsertableVolumetricContainerBuilder<Container> {
    #[inline]
    ///Sets the value of the `public.physical_assets.model_id` column.
    fn model(
        mut self,
        model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Container as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetBuildable>::model(
                self.id,
                model_id,
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
        Error = web_common_traits::database::InsertError<
            InsertableVolumetricContainerAttributes,
        >,
    >,
    Container: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertableVolumetricContainerAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.volumetric_container_model_id.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
