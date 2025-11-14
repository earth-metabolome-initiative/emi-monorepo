#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialProductExtensionAttribute {
    AssetModel(crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute),
}
impl core::fmt::Display for CommercialProductExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "commercial_products({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute>
    for CommercialProductExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    ) -> Self {
        Self::AssetModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for CommercialProductExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialProductAttribute {
    Extension(CommercialProductExtensionAttribute),
    Id,
    DeprecationDate,
    BrandId,
}
impl core::str::FromStr for CommercialProductAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "DeprecationDate" => Ok(Self::DeprecationDate),
            "BrandId" => Ok(Self::BrandId),
            "id" => Ok(Self::Id),
            "deprecation_date" => Ok(Self::DeprecationDate),
            "brand_id" => Ok(Self::BrandId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<T1>
{
    type Attribute = CommercialProductAttribute;
}
impl web_common_traits::database::TableField for CommercialProductAttribute {}
impl web_common_traits::database::HasTableType for CommercialProductAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    > for CommercialProductAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    ) -> Self {
        CommercialProductAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for CommercialProductAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        CommercialProductAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for CommercialProductAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_products.id"),
            Self::DeprecationDate => write!(f, "commercial_products.deprecation_date"),
            Self::BrandId => write!(f, "commercial_products.brand_id"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_products::commercial_products
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProduct {
    pub(crate) id: i32,
    pub(crate) deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) brand_id: i32,
}
impl InsertableCommercialProduct {
    pub fn brand<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::brands::Brand, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::brands::Brand:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::brands::Brand::read(self.brand_id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialProduct`](crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialProduct;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::CommercialProductSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_product = CommercialProduct::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .brand(brand_id)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    .deprecation_date(deprecation_date)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCommercialProductBuilder<
    AssetModel = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
> {
    pub(crate) deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) brand_id: Option<i32>,
    pub(crate) id: AssetModel,
}
impl<AssetModel> diesel::associations::HasTable for InsertableCommercialProductBuilder<AssetModel> {
    type Table =
        crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::table
    }
}
impl From<InsertableCommercialProductBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialProductBuilder>
{
    fn from(builder: InsertableCommercialProductBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<AssetModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        AssetModel,
    >
where
    AssetModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.brand_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `CommercialProduct`
/// or descendant tables.
pub trait CommercialProductSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.commercial_products.deprecation_date`
    /// column.
    ///
    /// # Arguments
    /// * `deprecation_date`: The value to set for the
    ///   `public.commercial_products.deprecation_date` column.
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn deprecation_date<DD>(self, deprecation_date: DD) -> Result<Self, Self::Error>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::prelude::SingleFieldError:
            From<<DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error>;
    /// Sets the value of the `public.commercial_products.brand_id` column.
    ///
    /// # Arguments
    /// * `brand_id`: The value to set for the
    ///   `public.commercial_products.brand_id` column.
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
    fn brand<BI>(self, brand_id: BI) -> Result<Self, Self::Error>
    where
        BI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<AssetModel> CommercialProductSettable
for InsertableCommercialProductBuilder<AssetModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<DD>(mut self, deprecation_date: DD) -> Result<Self, Self::Error>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::prelude::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        let deprecation_date = deprecation_date
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(CommercialProductAttribute::DeprecationDate)
            })?;
        self.deprecation_date = deprecation_date;
        Ok(self)
    }
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand<BI>(mut self, brand_id: BI) -> Result<Self, Self::Error>
    where
        BI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let brand_id = <BI as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &brand_id,
        );
        self.brand_id = Some(brand_id);
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialProductBuilder<AssetModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.id,
                name,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.id,
                description,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model` column.
    fn parent_model<PM>(mut self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
                self.id,
                parent_model,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.id,
                created_by,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.id,
                created_at,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.id,
                updated_by,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.id,
                updated_at,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<AssetModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialProductBuilder<AssetModel>
where
    AssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<AssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialProductBuilder<AssetModel>
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
    for InsertableCommercialProductBuilder<AssetModel>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            Error = web_common_traits::database::InsertError<CommercialProductAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<CommercialProductAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
