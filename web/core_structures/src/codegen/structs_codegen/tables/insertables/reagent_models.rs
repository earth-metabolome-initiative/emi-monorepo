#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentModelExtensionAttributes {
    AssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
    ),
}
impl core::fmt::Display for InsertableReagentModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes>
    for InsertableReagentModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
    ) -> Self {
        Self::AssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentModelAttributes {
    Extension(InsertableReagentModelExtensionAttributes),
    Id,
    Purity,
    CasCode,
    MolecularFormula,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes>
    for InsertableReagentModelAttributes
{
    fn from(
        asset_models: crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
    ) -> Self {
        Self::Extension(InsertableReagentModelExtensionAttributes::AssetModel(asset_models))
    }
}
impl core::str::FromStr for InsertableReagentModelAttributes {
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
impl core::fmt::Display for InsertableReagentModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Purity => write!(f, "purity"),
            Self::CasCode => write!(f, "cas_code"),
            Self::MolecularFormula => write!(f, "molecular_formula"),
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
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableReagentModelBuilder<
    AssetModel = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
> {
    pub(crate) purity: Option<f32>,
    pub(crate) cas_code: Option<::cas_codes::CAS>,
    pub(crate) molecular_formula: Option<::molecular_formulas::MolecularFormula>,
    pub(crate) id: AssetModel,
}
/// Trait defining setters for attributes of an instance of `ReagentModel` or
/// descendant tables.
pub trait ReagentModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
{
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
impl ReagentModelBuildable for Option<i32> {
    fn purity<P>(
        self,
        _purity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<P as TryInto<f32>>::Error>,
    {
        Ok(self)
    }
    fn cas_code<CC>(
        self,
        _cas_code: CC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CC: TryInto<::cas_codes::CAS>,
        validation_errors::SingleFieldError: From<<CC as TryInto<::cas_codes::CAS>>::Error>,
    {
        Ok(self)
    }
    fn molecular_formula<MF>(
        self,
        _molecular_formula: MF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MF: TryInto<::molecular_formulas::MolecularFormula>,
        validation_errors::SingleFieldError:
            From<<MF as TryInto<::molecular_formulas::MolecularFormula>>::Error>,
    {
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        >,
> ReagentModelBuildable for InsertableReagentModelBuilder<AssetModel> {
    ///Sets the value of the `public.reagent_models.purity` column.
    fn purity<P>(
        mut self,
        purity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<P as TryInto<f32>>::Error>,
    {
        let purity = purity
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableReagentModelAttributes::Purity)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(purity)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::Purity,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(purity, 100f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes::Purity,
                            )
                    })
            })?;
        self.purity = Some(purity);
        Ok(self)
    }
    ///Sets the value of the `public.reagent_models.cas_code` column.
    fn cas_code<CC>(
        mut self,
        cas_code: CC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CC: TryInto<::cas_codes::CAS>,
        validation_errors::SingleFieldError: From<
            <CC as TryInto<::cas_codes::CAS>>::Error,
        >,
    {
        let cas_code = cas_code
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableReagentModelAttributes::CasCode)
            })?;
        self.cas_code = Some(cas_code);
        Ok(self)
    }
    ///Sets the value of the `public.reagent_models.molecular_formula` column.
    fn molecular_formula<MF>(
        mut self,
        molecular_formula: MF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MF: TryInto<::molecular_formulas::MolecularFormula>,
        validation_errors::SingleFieldError: From<
            <MF as TryInto<::molecular_formulas::MolecularFormula>>::Error,
        >,
    {
        let molecular_formula = molecular_formula
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableReagentModelAttributes::MolecularFormula)
            })?;
        self.molecular_formula = Some(molecular_formula);
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableReagentModelBuilder<AssetModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableReagentModelAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
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
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
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
    ///Sets the value of the `public.asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::parent_model(
                self.id,
                parent_model_id,
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
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
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
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
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
            Error = web_common_traits::database::InsertError<InsertableReagentModelAttributes>,
        >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableReagentModelAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete()
            && self.purity.is_some()
            && self.cas_code.is_some()
            && self.molecular_formula.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
