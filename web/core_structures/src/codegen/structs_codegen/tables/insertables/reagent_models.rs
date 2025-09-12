#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReagentModelExtensionAttribute {
    AssetModel(crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute),
}
impl core::fmt::Display for ReagentModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute>
    for ReagentModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    ) -> Self {
        Self::AssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReagentModelAttribute {
    Extension(ReagentModelExtensionAttribute),
    Id,
    Purity,
    CasCode,
    MolecularFormula,
}
impl core::str::FromStr for ReagentModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Purity" => Ok(Self::Purity),
            "CasCode" => Ok(Self::CasCode),
            "MolecularFormula" => Ok(Self::MolecularFormula),
            "purity" => Ok(Self::Purity),
            "cas_code" => Ok(Self::CasCode),
            "molecular_formula" => Ok(Self::MolecularFormula),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
    > for ReagentModelAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Id.into(),
        )
    }
}
impl
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
    > for ReagentModelAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for ReagentModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "reagent_models.id"),
            Self::Purity => write!(f, "reagent_models.purity"),
            Self::CasCode => write!(f, "reagent_models.cas_code"),
            Self::MolecularFormula => write!(f, "reagent_models.molecular_formula"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::reagent_models::reagent_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableReagentModel {
    pub(crate) id: i32,
    pub(crate) purity: f32,
    pub(crate) cas_code: ::cas_codes::CAS,
    pub(crate) molecular_formula: ::molecular_formulas::MolecularFormula,
}
impl InsertableReagentModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(self.id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`ReagentModel`](crate::codegen::structs_codegen::tables::reagent_models::ReagentModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::ReagentModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::ReagentModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let reagent_model = ReagentModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .cas_code(cas_code)?
///    .molecular_formula(molecular_formula)?
///    .purity(purity)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableReagentModelBuilder<
    AssetModel = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
> {
    pub(crate) purity: Option<f32>,
    pub(crate) cas_code: Option<::cas_codes::CAS>,
    pub(crate) molecular_formula: Option<::molecular_formulas::MolecularFormula>,
    pub(crate) id: AssetModel,
}
impl From<InsertableReagentModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableReagentModelBuilder>
{
    fn from(builder: InsertableReagentModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<AssetModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelBuilder<
        AssetModel,
    >
where
    AssetModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
            && self.purity.is_some()
            && self.cas_code.is_some()
            && self.molecular_formula.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `ReagentModel` or
/// descendant tables.
pub trait ReagentModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.reagent_models.purity` column.
    ///
    /// # Arguments
    /// * `purity`: The value to set for the `public.reagent_models.purity`
    ///   column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn purity<P>(
        self,
        purity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<P as TryInto<f32>>::Error>;
    /// Sets the value of the `public.reagent_models.cas_code` column.
    ///
    /// # Arguments
    /// * `cas_code`: The value to set for the `public.reagent_models.cas_code`
    ///   column.
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
    ///   `::cas_codes::CAS`.
    /// * If the provided value does not pass schema-defined validation.
    fn cas_code<CC>(
        self,
        cas_code: CC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CC: TryInto<::cas_codes::CAS>,
        validation_errors::SingleFieldError: From<<CC as TryInto<::cas_codes::CAS>>::Error>;
    /// Sets the value of the `public.reagent_models.molecular_formula` column.
    ///
    /// # Arguments
    /// * `molecular_formula`: The value to set for the
    ///   `public.reagent_models.molecular_formula` column.
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
    ///   `::molecular_formulas::MolecularFormula`.
    /// * If the provided value does not pass schema-defined validation.
    fn molecular_formula<MF>(
        self,
        molecular_formula: MF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MF: TryInto<::molecular_formulas::MolecularFormula>,
        validation_errors::SingleFieldError:
            From<<MF as TryInto<::molecular_formulas::MolecularFormula>>::Error>;
}
impl<AssetModel> ReagentModelSettable for InsertableReagentModelBuilder<AssetModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute;
    /// Sets the value of the `public.reagent_models.purity` column.
    fn purity<P>(
        mut self,
        purity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<P as TryInto<f32>>::Error>,
    {
        let purity = purity.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ReagentModelAttribute::Purity)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(purity)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute::Purity,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(purity, 100f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute::Purity,
                            )
                    })
            })?;
        self.purity = Some(purity);
        Ok(self)
    }
    /// Sets the value of the `public.reagent_models.cas_code` column.
    fn cas_code<CC>(
        mut self,
        cas_code: CC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CC: TryInto<::cas_codes::CAS>,
        validation_errors::SingleFieldError: From<<CC as TryInto<::cas_codes::CAS>>::Error>,
    {
        let cas_code = cas_code.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ReagentModelAttribute::CasCode)
        })?;
        self.cas_code = Some(cas_code);
        Ok(self)
    }
    /// Sets the value of the `public.reagent_models.molecular_formula` column.
    fn molecular_formula<MF>(
        mut self,
        molecular_formula: MF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MF: TryInto<::molecular_formulas::MolecularFormula>,
        validation_errors::SingleFieldError:
            From<<MF as TryInto<::molecular_formulas::MolecularFormula>>::Error>,
    {
        let molecular_formula = molecular_formula.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ReagentModelAttribute::MolecularFormula)
        })?;
        self.molecular_formula = Some(molecular_formula);
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
    for InsertableReagentModelBuilder<AssetModel>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ReagentModelAttribute;
    #[inline]
    /// Sets the value of the `public.asset_models.name` column.
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
    /// Sets the value of the `public.asset_models.description` column.
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
    /// Sets the value of the `public.asset_models.parent_model` column.
    fn parent_model<PM>(
        mut self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::parent_model(
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
    #[inline]
    /// Sets the value of the `public.asset_models.created_by` column.
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    /// Sets the value of the `public.asset_models.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
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
    /// Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    /// Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
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
    for InsertableReagentModelBuilder<AssetModel>
where
    AssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<AssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableReagentModelBuilder<AssetModel>
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
    for InsertableReagentModelBuilder<AssetModel>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::reagent_models::ReagentModel,
            Error = web_common_traits::database::InsertError<ReagentModelAttribute>,
        >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attribute = ReagentModelAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
