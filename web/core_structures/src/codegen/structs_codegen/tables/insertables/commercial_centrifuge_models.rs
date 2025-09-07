#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialCentrifugeModelExtensionAttribute {
    CentrifugeModel(crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ),
}
impl core::fmt::Display for CommercialCentrifugeModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CentrifugeModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute>
    for CommercialCentrifugeModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
    ) -> Self {
        Self::CentrifugeModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute>
    for CommercialCentrifugeModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialCentrifugeModelAttribute {
    Extension(CommercialCentrifugeModelExtensionAttribute),
    Id,
    CentrifugeModel,
}
impl core::str::FromStr for CommercialCentrifugeModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CentrifugeModel" => Ok(Self::CentrifugeModel),
            "centrifuge_model" => Ok(Self::CentrifugeModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CommercialCentrifugeModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::CentrifugeModel => write!(f, "centrifuge_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_centrifuge_models::commercial_centrifuge_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialCentrifugeModel {
    pub(crate) id: i32,
    pub(crate) centrifuge_model: i32,
}
impl InsertableCommercialCentrifugeModel {
    pub fn centrifuge_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CentrifugeModel, diesel::result::Error>
    where
        crate::CentrifugeModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CentrifugeModel::read(self.centrifuge_model, conn)
    }
    pub fn commercial_centrifuge_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CentrifugeModel, diesel::result::Error>
    where
        crate::CentrifugeModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CentrifugeModel::read(self.id, conn)
    }
    pub fn commercial_centrifuge_models_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialProduct, diesel::result::Error>
    where
        crate::CommercialProduct: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialProduct::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_centrifuge_models_id_centrifuge_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::AssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(&self.centrifuge_model),
                    ),
            )
            .first::<crate::AssetModel>(conn)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialCentrifugeModelBuilder<
    CentrifugeModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            Option<i32>,
        >,
> {
    pub(crate) centrifuge_model: Option<i32>,
    pub(crate) commercial_centrifuge_models_id_fkey: CentrifugeModel,
    pub(crate) commercial_centrifuge_models_id_fkey1: CommercialProduct,
}
impl From<InsertableCommercialCentrifugeModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialCentrifugeModelBuilder>
{
    fn from(builder: InsertableCommercialCentrifugeModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialCentrifugeModel` or descendant tables.
pub trait CommercialCentrifugeModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.commercial_centrifuge_models.centrifuge_model` column.
    ///
    /// # Arguments
    /// * `centrifuge_model`: The value to set for the
    ///   `public.commercial_centrifuge_models.centrifuge_model` column.
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
    fn centrifuge_model(
        self,
        centrifuge_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    CentrifugeModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
        >,
    CommercialProduct,
> CommercialCentrifugeModelSettable
for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute;
    ///Sets the value of the `public.commercial_centrifuge_models.centrifuge_model` column.
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
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_centrifuge_models`"]
    ///    v0@{shape: rounded, label: "centrifuge_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v2
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v5 --->|"`extends`"| v3
    ///```
    fn centrifuge_model(
        mut self,
        centrifuge_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_centrifuge_models_id_fkey,
                Some(centrifuge_model),
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.centrifuge_model = Some(centrifuge_model);
        Ok(self)
    }
}
impl<
    CentrifugeModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute,
        >,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute;
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_centrifuge_models_id_fkey,
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_centrifuge_models_id_fkey,
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_centrifuge_models_id_fkey,
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_centrifuge_models_id_fkey,
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_centrifuge_models_id_fkey,
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
        self.commercial_centrifuge_models_id_fkey = <CentrifugeModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_centrifuge_models_id_fkey,
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
impl<CentrifugeModel, CommercialProduct>
    crate::codegen::structs_codegen::tables::insertables::CentrifugeModelSettable
    for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute;
}
impl<
    CentrifugeModel,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable
for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute;
    #[inline]
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<DD>(
        mut self,
        deprecation_date: DD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_centrifuge_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::deprecation_date(
                self.commercial_centrifuge_models_id_fkey1,
                deprecation_date,
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
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand(
        mut self,
        brand_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_centrifuge_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::brand(
                self.commercial_centrifuge_models_id_fkey1,
                brand_id,
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
    CentrifugeModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
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
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_centrifuge_models`"]
    ///    v1@{shape: rounded, label: "centrifuge_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v5 --->|"`extends`"| v3
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialCentrifugeModelSettable>::centrifuge_model(
            self,
            parent_model
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::CentrifugeModel,
                    ),
                )?,
        )
    }
}
impl<CentrifugeModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
where
    CentrifugeModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_centrifuge_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_centrifuge_models_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CentrifugeModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
where
    CentrifugeModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_centrifuge_models_id_fkey =
            self.commercial_centrifuge_models_id_fkey.set_primary_key(primary_key);
        self.commercial_centrifuge_models_id_fkey1 =
            self.commercial_centrifuge_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<CentrifugeModel, CommercialProduct, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCommercialCentrifugeModelBuilder<CentrifugeModel, CommercialProduct>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::CommercialCentrifugeModel,
            Error = web_common_traits::database::InsertError<CommercialCentrifugeModelAttribute>,
        >,
    CentrifugeModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = CommercialCentrifugeModelAttribute;
    fn is_complete(&self) -> bool {
        self.commercial_centrifuge_models_id_fkey.is_complete()
            && self.commercial_centrifuge_models_id_fkey1.is_complete()
            && self.centrifuge_model.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::CommercialCentrifugeModel = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
