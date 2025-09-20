#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HarvestingProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for HarvestingProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "harvesting_procedures({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for HarvestingProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for HarvestingProcedureExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HarvestingProcedureAttribute {
    Extension(HarvestingProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    SampleSource,
    ProcedureTemplateSampleSourceModel,
    ProcedureSampleSource(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    Sample,
    ProcedureTemplateSampleModel,
    ProcedureSample(crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute),
}
impl core::str::FromStr for HarvestingProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Procedure" => Ok(Self::Procedure),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "SampleSource" => Ok(Self::SampleSource),
            "ProcedureTemplateSampleSourceModel" => Ok(Self::ProcedureTemplateSampleSourceModel),
            "ProcedureSampleSource" => Ok(Self::ProcedureSampleSource(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "Sample" => Ok(Self::Sample),
            "ProcedureTemplateSampleModel" => Ok(Self::ProcedureTemplateSampleModel),
            "ProcedureSample" => Ok(Self::ProcedureSample(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure" => Ok(Self::Procedure),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "sample_source" => Ok(Self::SampleSource),
            "procedure_template_sample_source_model" => {
                Ok(Self::ProcedureTemplateSampleSourceModel)
            }
            "procedure_sample_source" => Ok(Self::ProcedureSampleSource(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "sample" => Ok(Self::Sample),
            "procedure_template_sample_model" => Ok(Self::ProcedureTemplateSampleModel),
            "procedure_sample" => Ok(Self::ProcedureSample(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
        T1,
    >
{
    type Attribute = HarvestingProcedureAttribute;
}
impl web_common_traits::database::TableField for HarvestingProcedureAttribute {}
impl web_common_traits::database::HasTableType for HarvestingProcedureAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    > for HarvestingProcedureAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        HarvestingProcedureAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for HarvestingProcedureAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        HarvestingProcedureAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for HarvestingProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "harvesting_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "harvesting_procedures.procedure_template")
            }
            Self::SampleSource => write!(f, "harvesting_procedures.sample_source"),
            Self::ProcedureTemplateSampleSourceModel => {
                write!(f, "harvesting_procedures.procedure_template_sample_source_model")
            }
            Self::ProcedureSampleSource(e) => write!(f, "harvesting_procedures.{e}"),
            Self::Sample => write!(f, "harvesting_procedures.sample"),
            Self::ProcedureTemplateSampleModel => {
                write!(f, "harvesting_procedures.procedure_template_sample_model")
            }
            Self::ProcedureSample(e) => write!(f, "harvesting_procedures.{e}"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableHarvestingProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) sample_source: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_sample_source_model: i32,
    pub(crate) procedure_sample_source: ::rosetta_uuid::Uuid,
    pub(crate) sample: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_sample_model: i32,
    pub(crate) procedure_sample: ::rosetta_uuid::Uuid,
}
impl InsertableHarvestingProcedure {
    pub fn procedure_sample<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_sample,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_sample_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_sample_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.sample),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_sample_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_sample_source,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_source_procedure_te_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_source_sample_sourc_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.sample_source),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template_sample_model
                            .eq(&self.procedure_template_sample_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template_sample_source_model
                            .eq(&self.procedure_template_sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
            >(conn)
    }
    pub fn procedure_template_sample_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_sample_model,
            conn,
        )
    }
    pub fn procedure_template_sample_source_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_sample_source_model,
            conn,
        )
    }
    pub fn sample<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::samples::Sample, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::samples::Sample:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::samples::Sample::read(self.sample, conn)
    }
    pub fn sample_source<C: diesel::connection::LoadConnection>(
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
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(
            self.sample_source,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`HarvestingProcedure`](crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::HarvestingProcedure;
/// use core_structures::tables::insertables::HarvestingProcedureSettable;
/// use core_structures::tables::insertables::ProcedureSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let harvesting_procedure = HarvestingProcedure::new()
///    // Set mandatory fields
///    .procedure_sample(procedure_sample)?
///    .procedure_sample_source(procedure_sample_source)?
///    .procedure_template(procedure_template)?
///    .created_by(created_by)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .procedure(procedure)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_procedure(parent_procedure)?
///    .predecessor_procedure(predecessor_procedure)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableHarvestingProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) sample_source: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_sample_source_model: Option<i32>,
    pub(crate) procedure_sample_source: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) sample: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_sample_model: Option<i32>,
    pub(crate) procedure_sample: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl<Procedure> diesel::associations::HasTable for InsertableHarvestingProcedureBuilder<Procedure> {
    type Table =
        crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::table
    }
}
impl From<InsertableHarvestingProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableHarvestingProcedureBuilder,
    >
{
    fn from(builder: InsertableHarvestingProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
        Procedure,
    >
where
    Procedure: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && (self.sample_source.is_some() || self.procedure_sample_source.is_complete())
            && (self.procedure_template_sample_source_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_sample_source.is_complete())
            && self.procedure_sample_source.is_complete()
            && (self.sample.is_some() || self.procedure_sample.is_complete())
            && (self.procedure_template_sample_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_sample.is_complete())
            && self.procedure_sample.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `HarvestingProcedure` or descendant tables.
pub trait HarvestingProcedureSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.harvesting_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.harvesting_procedures.procedure_template` column.
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
    fn procedure_template<PT>(self, procedure_template: PT) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.harvesting_procedures.sample_source`
    /// column.
    ///
    /// # Arguments
    /// * `sample_source`: The value to set for the
    ///   `public.harvesting_procedures.sample_source` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn sample_source<SS>(self, sample_source: SS) -> Result<Self, Self::Error>
    where
        SS: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.harvesting_procedures.procedure_template_sample_source_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_sample_source_model`: The value to set for the
    ///   `public.harvesting_procedures.procedure_template_sample_source_model`
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_sample_source_model<PTSSM>(
        self,
        procedure_template_sample_source_model: PTSSM,
    ) -> Result<Self, Self::Error>
    where
        PTSSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.harvesting_procedures.procedure_sample_source` column.
    ///
    /// # Arguments
    /// * `procedure_sample_source`: The value to set for the
    ///   `public.harvesting_procedures.procedure_sample_source` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_sample_source<PSS>(
        self,
        procedure_sample_source: PSS,
    ) -> Result<Self, Self::Error>
    where
        PSS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.harvesting_procedures.sample` column.
    ///
    /// # Arguments
    /// * `sample`: The value to set for the
    ///   `public.harvesting_procedures.sample` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn sample<S>(self, sample: S) -> Result<Self, Self::Error>
    where
        S: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.harvesting_procedures.procedure_template_sample_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_sample_model`: The value to set for the
    ///   `public.harvesting_procedures.procedure_template_sample_model` column.
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
    fn procedure_template_sample_model<PTSM>(
        self,
        procedure_template_sample_model: PTSM,
    ) -> Result<Self, Self::Error>
    where
        PTSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.harvesting_procedures.procedure_sample`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_sample`: The value to set for the
    ///   `public.harvesting_procedures.procedure_sample` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_sample<PS>(self, procedure_sample: PS) -> Result<Self, Self::Error>
    where
        PS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
            >,
        >,
> HarvestingProcedureSettable for InsertableHarvestingProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.harvesting_procedures.procedure_template` column.
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
    ///subgraph v5 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_sample_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_sample_source_model"}
    ///class v2 directly-involved-column
    ///end
    ///subgraph v6 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v4 undirectly-involved-column
    ///end
    ///subgraph v7 ["`procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template"}
    ///class v3 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v3
    ///v0 -.->|"`foreign defines`"| v1
    ///v0 -.->|"`foreign defines`"| v2
    ///v1 --->|"`associated same as`"| v4
    ///v2 --->|"`associated same as`"| v4
    ///v5 --->|"`extends`"| v7
    ///v5 ---o|"`associated with`"| v6
    ///```
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template = <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template,
        );
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                self.procedure,
                procedure_template,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.sample_source` column.
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
    ///subgraph v4 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample_source"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "sample_source"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 -.->|"`foreign defines`"| v1
    ///v1 --->|"`associated same as`"| v2
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn sample_source<SS>(mut self, sample_source: SS) -> Result<Self, Self::Error>
    where
        SS: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let sample_source = <SS as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &sample_source,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_sample_source,
        ) = self.procedure_sample_source
        {
            self.procedure_sample_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_sample_source,
                    sample_source,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureSampleSource(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.sample_source = Some(sample_source);
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.procedure_template_sample_source_model` column.
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
    ///subgraph v4 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample_source"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_sample_source_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 -.->|"`foreign defines`"| v1
    ///v1 --->|"`associated same as`"| v2
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn procedure_template_sample_source_model<PTSSM>(
        mut self,
        procedure_template_sample_source_model: PTSSM,
    ) -> Result<Self, Self::Error>
    where
        PTSSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_sample_source_model = <PTSSM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_sample_source_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_sample_source,
        ) = self.procedure_sample_source
        {
            self.procedure_sample_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_sample_source,
                    procedure_template_sample_source_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureSampleSource(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_sample_source_model = Some(
            procedure_template_sample_source_model,
        );
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.procedure_sample_source` column.
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
    ///subgraph v6 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample_source"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_sample_source_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "sample_source"}
    ///class v2 directly-involved-column
    ///end
    ///subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    ///class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    ///class v5 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v5
    ///v0 --->|"`associated same as`"| v5
    ///v0 --->|"`associated same as`"| v5
    ///v0 -.->|"`foreign defines`"| v1
    ///v0 -.->|"`foreign defines`"| v2
    ///v1 --->|"`associated same as`"| v4
    ///v2 --->|"`associated same as`"| v3
    ///v6 ---o|"`associated with`"| v7
    ///```
    fn procedure_sample_source<PSS>(
        mut self,
        procedure_sample_source: PSS,
    ) -> Result<Self, Self::Error>
    where
        PSS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_sample_source = procedure_sample_source.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_sample_source {
            procedure_sample_source = if let (
                Some(procedure_template_sample_source_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_sample_source_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_sample_source_model
                    != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateSampleSourceModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_sample_source_model = Some(
                    procedure_template_asset_model,
                );
                builder.into()
            } else if let Some(procedure_template_sample_source_model) = self
                .procedure_template_sample_source_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_sample_source_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureSampleSource(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_sample_source {
            procedure_sample_source = if let (Some(sample_source), Some(asset)) = (
                self.sample_source,
                builder.asset,
            ) {
                if sample_source != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::SampleSource,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.sample_source = Some(asset);
                builder.into()
            } else if let Some(sample_source) = self.sample_source {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        sample_source,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureSampleSource(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_sample_source = procedure_sample_source;
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.sample` column.
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
    ///subgraph v4 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "sample"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 -.->|"`foreign defines`"| v1
    ///v1 --->|"`associated same as`"| v2
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn sample<S>(mut self, sample: S) -> Result<Self, Self::Error>
    where
        S: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let sample = <S as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &sample,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_sample) = self
            .procedure_sample
        {
            self.procedure_sample = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_sample,
                    sample,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureSample(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.sample = Some(sample);
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.procedure_template_sample_model` column.
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
    ///subgraph v4 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_sample_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 --->|"`associated same as`"| v3
    ///v0 -.->|"`foreign defines`"| v1
    ///v1 --->|"`associated same as`"| v2
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn procedure_template_sample_model<PTSM>(
        mut self,
        procedure_template_sample_model: PTSM,
    ) -> Result<Self, Self::Error>
    where
        PTSM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_sample_model = <PTSM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_sample_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_sample) = self
            .procedure_sample
        {
            self.procedure_sample = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_sample,
                    procedure_template_sample_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureSample(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_sample_model = Some(procedure_template_sample_model);
        Ok(self)
    }
    ///Sets the value of the `public.harvesting_procedures.procedure_sample` column.
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
    ///subgraph v6 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_sample"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_sample_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "sample"}
    ///class v2 directly-involved-column
    ///end
    ///subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    ///class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    ///class v5 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v5
    ///v0 --->|"`associated same as`"| v5
    ///v0 --->|"`associated same as`"| v5
    ///v0 -.->|"`foreign defines`"| v1
    ///v0 -.->|"`foreign defines`"| v2
    ///v1 --->|"`associated same as`"| v4
    ///v2 --->|"`associated same as`"| v3
    ///v6 ---o|"`associated with`"| v7
    ///```
    fn procedure_sample<PS>(mut self, procedure_sample: PS) -> Result<Self, Self::Error>
    where
        PS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_sample = procedure_sample.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_sample {
            procedure_sample = if let (
                Some(procedure_template_sample_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_sample_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_sample_model != procedure_template_asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateSampleModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_sample_model = Some(
                    procedure_template_asset_model,
                );
                builder.into()
            } else if let Some(procedure_template_sample_model) = self
                .procedure_template_sample_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_sample_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureSample(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_sample {
            procedure_sample = if let (Some(sample), Some(asset)) = (
                self.sample,
                builder.asset,
            ) {
                if sample != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::Sample,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.sample = Some(asset);
                builder.into()
            } else if let Some(sample) = self.sample {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        sample,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureSample(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_sample = procedure_sample;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableHarvestingProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(mut self, procedure: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                self.procedure,
                procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.procedure_template` column.
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
    ///subgraph v2 ["`harvesting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v1
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template<PT>(self, procedure_template: PT) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as HarvestingProcedureSettable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure<PP>(mut self, parent_procedure: PP) -> Result<Self, Self::Error>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                self.procedure,
                parent_procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure_template` column.
    fn parent_procedure_template<PPT>(
        mut self,
        parent_procedure_template: PPT,
    ) -> Result<Self, Self::Error>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                self.procedure,
                parent_procedure_template,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.predecessor_procedure` column.
    fn predecessor_procedure<PP>(
        mut self,
        predecessor_procedure: PP,
    ) -> Result<Self, Self::Error>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                self.procedure,
                predecessor_procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.predecessor_procedure_template` column.
    fn predecessor_procedure_template<PPT>(
        mut self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, Self::Error>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                self.procedure,
                predecessor_procedure_template,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                self.procedure,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                self.procedure,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                self.procedure,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                self.procedure,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertableHarvestingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableHarvestingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = self.procedure.set_primary_key(primary_key);
        self
    }
}
impl<Procedure, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableHarvestingProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
            Error = web_common_traits::database::InsertError<
                HarvestingProcedureAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<HarvestingProcedureAttribute>;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
