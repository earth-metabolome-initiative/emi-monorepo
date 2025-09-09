#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhysicalAssetModelExtensionAttribute {
    AssetModel(crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute),
}
impl core::fmt::Display for PhysicalAssetModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute>
    for PhysicalAssetModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    ) -> Self {
        Self::AssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhysicalAssetModelAttribute {
    Extension(PhysicalAssetModelExtensionAttribute),
    Id,
    ParentModel,
}
impl core::str::FromStr for PhysicalAssetModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentModel" => Ok(Self::ParentModel),
            "parent_model" => Ok(Self::ParentModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for PhysicalAssetModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ParentModel => write!(f, "parent_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhysicalAssetModel {
    pub(crate) id: i32,
    pub(crate) parent_model: Option<i32>,
}
impl InsertablePhysicalAssetModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn physical_asset_models_id_parent_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::AssetModel>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(parent_model) = self.parent_model else {
            return Ok(None);
        };
        crate::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(parent_model),
                    ),
            )
            .first::<crate::AssetModel>(conn)
            .map(Some)
    }
    pub fn parent_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::PhysicalAssetModel>, diesel::result::Error>
    where
        crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(parent_model) = self.parent_model else {
            return Ok(None);
        };
        crate::PhysicalAssetModel::read(parent_model, conn).map(Some)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhysicalAssetModelBuilder<
    AssetModel = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
> {
    pub(crate) parent_model: Option<i32>,
    pub(crate) id: AssetModel,
}
impl From<InsertablePhysicalAssetModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertablePhysicalAssetModelBuilder>
{
    fn from(builder: InsertablePhysicalAssetModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<AssetModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
        AssetModel,
    >
where
    AssetModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `PhysicalAssetModel`
/// or descendant tables.
pub trait PhysicalAssetModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.physical_asset_models.parent_model`
    /// column.
    ///
    /// # Arguments
    /// * `parent_model`: The value to set for the
    ///   `public.physical_asset_models.parent_model` column.
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
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
> PhysicalAssetModelSettable for InsertablePhysicalAssetModelBuilder<AssetModel>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute;
    /// Sets the value of the `public.physical_asset_models.parent_model`
    /// column.
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
    /// class v0 directly-involved-column
    /// end
    /// subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    /// class v1 column-of-interest
    /// end
    /// v1 --->|"`ancestral same as`"| v0
    /// v3 --->|"`extends`"| v2
    /// ```
    fn parent_model(
        mut self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                self.id,
                parent_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.parent_model = parent_model;
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute;
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
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
        <Self as PhysicalAssetModelSettable>::parent_model(self, parent_model)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
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
impl<AssetModel> web_common_traits::database::MostConcreteTable
    for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    AssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<AssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    AssetModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<AssetModel, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::PhysicalAssetModel,
            Error = web_common_traits::database::InsertError<PhysicalAssetModelAttribute>,
        >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = PhysicalAssetModelAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::PhysicalAssetModel = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
