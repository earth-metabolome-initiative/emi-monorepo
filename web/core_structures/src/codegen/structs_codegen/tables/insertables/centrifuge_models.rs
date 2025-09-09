#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CentrifugeModelExtensionAttribute {
    PhysicalAssetModel(
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    ),
}
impl core::fmt::Display for CentrifugeModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PhysicalAssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute>
    for CentrifugeModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    ) -> Self {
        Self::PhysicalAssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CentrifugeModelAttribute {
    Extension(CentrifugeModelExtensionAttribute),
    Id,
}
impl core::str::FromStr for CentrifugeModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CentrifugeModelAttribute {
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
        table_name = crate::codegen::diesel_codegen::tables::centrifuge_models::centrifuge_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeModel {
    pub(crate) id: i32,
}
impl InsertableCentrifugeModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAssetModel, diesel::result::Error>
    where
        crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAssetModel::read(self.id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeModelBuilder<
    PhysicalAssetModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
        >,
> {
    pub(crate) id: PhysicalAssetModel,
}
impl From<InsertableCentrifugeModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCentrifugeModelBuilder>
{
    fn from(builder: InsertableCentrifugeModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PhysicalAssetModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder<
        PhysicalAssetModel,
    >
where
    PhysicalAssetModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `CentrifugeModel` or
/// descendant tables.
pub trait CentrifugeModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
}
impl<PhysicalAssetModel> CentrifugeModelSettable
    for InsertableCentrifugeModelBuilder<PhysicalAssetModel>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute;
}
impl<
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCentrifugeModelBuilder<PhysicalAssetModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
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
    ///flowchart BT
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
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
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
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCentrifugeModelBuilder<PhysicalAssetModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
    fn parent_model(
        mut self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
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
impl<PhysicalAssetModel> web_common_traits::database::MostConcreteTable
    for InsertableCentrifugeModelBuilder<PhysicalAssetModel>
where
    PhysicalAssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<PhysicalAssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCentrifugeModelBuilder<PhysicalAssetModel>
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
    for InsertableCentrifugeModelBuilder<PhysicalAssetModel>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::CentrifugeModel,
            Error = web_common_traits::database::InsertError<CentrifugeModelAttribute>,
        >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = CentrifugeModelAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::CentrifugeModel = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
