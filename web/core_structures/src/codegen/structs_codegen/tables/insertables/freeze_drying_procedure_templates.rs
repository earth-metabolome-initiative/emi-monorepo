#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableFreezeDryingProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableFreezeDryingProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingProcedureTemplateAttributes {
    Extension(InsertableFreezeDryingProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Pascal,
    Seconds,
    FreezeDriedWithModel,
    ProcedureTemplateFreezeDriedWithModel,
    FreezeDriedContainerModel,
    ForeignProcedureTemplate,
    ProcedureTemplateFreezeDriedContainerModel,
}
impl core::str::FromStr for InsertableFreezeDryingProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Pascal" => Ok(Self::Pascal),
            "Seconds" => Ok(Self::Seconds),
            "FreezeDriedWithModel" => Ok(Self::FreezeDriedWithModel),
            "ProcedureTemplateFreezeDriedWithModel" => {
                Ok(Self::ProcedureTemplateFreezeDriedWithModel)
            }
            "FreezeDriedContainerModel" => Ok(Self::FreezeDriedContainerModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateFreezeDriedContainerModel" => {
                Ok(Self::ProcedureTemplateFreezeDriedContainerModel)
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "pascal" => Ok(Self::Pascal),
            "seconds" => Ok(Self::Seconds),
            "freeze_dried_with_model" => Ok(Self::FreezeDriedWithModel),
            "procedure_template_freeze_dried_with_model" => {
                Ok(Self::ProcedureTemplateFreezeDriedWithModel)
            }
            "freeze_dried_container_model" => Ok(Self::FreezeDriedContainerModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_freeze_dried_container_model" => {
                Ok(Self::ProcedureTemplateFreezeDriedContainerModel)
            }
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableFreezeDryingProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Pascal => write!(f, "pascal"),
            Self::Seconds => write!(f, "seconds"),
            Self::FreezeDriedWithModel => write!(f, "freeze_dried_with_model"),
            Self::ProcedureTemplateFreezeDriedWithModel => {
                write!(f, "procedure_template_freeze_dried_with_model")
            }
            Self::FreezeDriedContainerModel => write!(f, "freeze_dried_container_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateFreezeDriedContainerModel => {
                write!(f, "procedure_template_freeze_dried_container_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::freeze_drying_procedure_templates::freeze_drying_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) pascal: f32,
    pub(crate) seconds: f32,
    pub(crate) freeze_dried_with_model: i32,
    pub(crate) procedure_template_freeze_dried_with_model: i32,
    pub(crate) freeze_dried_container_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_freeze_dried_container_model: i32,
}
impl InsertableFreezeDryingProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn freeze_dried_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel::table(),
                self.freeze_dried_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_freeze_dried_with_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_freeze_dried_with_model,
            ),
            conn,
        )
    }
    pub fn freeze_dried_container_model<C: diesel::connection::LoadConnection>(
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
                self.freeze_dried_container_model,
            ),
            conn,
        )
    }
    pub fn foreign_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.foreign_procedure_template,
            ),
            conn,
        )
    }
    pub fn procedure_template_freeze_dried_container_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_freeze_dried_container_model,
            ),
            conn,
        )
    }
    pub fn freeze_drying_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.freeze_dried_with_model, self.freeze_dried_container_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) pascal: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) freeze_dried_with_model: Option<i32>,
    pub(crate) procedure_template_freeze_dried_with_model: Option<i32>,
    pub(crate) freeze_dried_container_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_freeze_dried_container_model: Option<i32>,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> Default
    for InsertableFreezeDryingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(203.15f32),
            kelvin_tolerance_percentage: Some(5f32),
            pascal: Some(4f32),
            seconds: Some(259200f32),
            freeze_dried_with_model: Default::default(),
            procedure_template_freeze_dried_with_model: Default::default(),
            freeze_dried_container_model: Default::default(),
            foreign_procedure_template: Default::default(),
            procedure_template_freeze_dried_container_model: Default::default(),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `FreezeDryingProcedureTemplate` or descendant tables.
pub trait FreezeDryingProcedureTemplateBuildable:
    crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
{
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.kelvin` column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.kelvin` column.
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
    fn kelvin<'K, K>(
        self,
        kelvin: &'K K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'K K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'K K as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// kelvin_tolerance_percentage` column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.
    ///   kelvin_tolerance_percentage` column.
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
    fn kelvin_tolerance_percentage<'KTP, KTP>(
        self,
        kelvin_tolerance_percentage: &'KTP KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'KTP KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'KTP KTP as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.pascal` column.
    ///
    /// # Arguments
    /// * `pascal`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.pascal` column.
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
    fn pascal<'P, P>(
        self,
        pascal: &'P P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'P P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'P P as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.seconds` column.
    ///
    /// # Arguments
    /// * `seconds`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.seconds`
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
    fn seconds<'S, S>(
        self,
        seconds: &'S S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'S S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'S S as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// freeze_dried_with_model` column.
    ///
    /// # Arguments
    /// * `freeze_dried_with_model`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.
    ///   freeze_dried_with_model` column.
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
    fn freeze_dried_with_model(
        self,
        freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// procedure_template_freeze_dried_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_freeze_dried_with_model`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.
    ///   procedure_template_freeze_dried_with_model` column.
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
    fn procedure_template_freeze_dried_with_model(
        self,
        procedure_template_freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// freeze_dried_container_model` column.
    ///
    /// # Arguments
    /// * `freeze_dried_container_model`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.
    ///   freeze_dried_container_model` column.
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
    fn freeze_dried_container_model(
        self,
        freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// foreign_procedure_template` column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `procedure_templates.freeze_drying_procedure_templates.
    ///   foreign_procedure_template` column.
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
    fn foreign_procedure_template(
        self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `procedure_templates.freeze_drying_procedure_templates.
    /// procedure_template_freeze_dried_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_freeze_dried_container_model`: The value to set
    ///   for the `procedure_templates.freeze_drying_procedure_templates.
    ///   procedure_template_freeze_dried_container_model` column.
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
    fn procedure_template_freeze_dried_container_model(
        self,
        procedure_template_freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl FreezeDryingProcedureTemplateBuildable for Option<i32> {
    fn kelvin<'K, K>(
        self,
        _kelvin: &'K K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'K K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'K K as TryInto<f32>>::Error>,
    {
        Ok(self)
    }
    fn kelvin_tolerance_percentage<'KTP, KTP>(
        self,
        _kelvin_tolerance_percentage: &'KTP KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'KTP KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'KTP KTP as TryInto<f32>>::Error>,
    {
        Ok(self)
    }
    fn pascal<'P, P>(
        self,
        _pascal: &'P P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'P P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'P P as TryInto<f32>>::Error>,
    {
        Ok(self)
    }
    fn seconds<'S, S>(
        self,
        _seconds: &'S S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'S S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'S S as TryInto<f32>>::Error>,
    {
        Ok(self)
    }
    fn freeze_dried_with_model(
        self,
        _freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn procedure_template_freeze_dried_with_model(
        self,
        _procedure_template_freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn freeze_dried_container_model(
        self,
        _freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn foreign_procedure_template(
        self,
        _foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn procedure_template_freeze_dried_container_model(
        self,
        _procedure_template_freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> FreezeDryingProcedureTemplateBuildable
for InsertableFreezeDryingProcedureTemplateBuilder<ProcedureTemplate> {
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.kelvin` column.
    fn kelvin<'K, K>(
        mut self,
        kelvin: &'K K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'K K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'K K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::Kelvin,
                    )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.kelvin_tolerance_percentage` column.
    fn kelvin_tolerance_percentage<'KTP, KTP>(
        mut self,
        kelvin_tolerance_percentage: &'KTP KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'KTP KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'KTP KTP as TryInto<f32>>::Error>,
    {
        let kelvin_tolerance_percentage = kelvin_tolerance_percentage
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::KelvinTolerancePercentage,
                    )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::KelvinTolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(
                        kelvin_tolerance_percentage,
                        100f32,
                    )
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.pascal` column.
    fn pascal<'P, P>(
        mut self,
        pascal: &'P P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'P P: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'P P as TryInto<f32>>::Error>,
    {
        let pascal = pascal
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::Pascal,
                    )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(pascal)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Pascal,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(pascal, 500f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Pascal,
                            )
                    })
            })?;
        self.pascal = Some(pascal);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.seconds` column.
    fn seconds<'S, S>(
        mut self,
        seconds: &'S S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'S S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<&'S S as TryInto<f32>>::Error>,
    {
        let seconds = seconds
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::Seconds,
                    )
            })?;
        pgrx_validation::must_be_strictly_greater_than_f32(seconds, 7200f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 604800f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.freeze_dried_with_model` column.
    fn freeze_dried_with_model(
        mut self,
        freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let freeze_dried_with_model = freeze_dried_with_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::FreezeDriedWithModel,
                    )
            })?;
        self.freeze_dried_with_model = Some(freeze_dried_with_model);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.procedure_template_freeze_dried_with_model` column.
    fn procedure_template_freeze_dried_with_model(
        mut self,
        procedure_template_freeze_dried_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let procedure_template_freeze_dried_with_model = procedure_template_freeze_dried_with_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::ProcedureTemplateFreezeDriedWithModel,
                    )
            })?;
        self.procedure_template_freeze_dried_with_model = Some(
            procedure_template_freeze_dried_with_model,
        );
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.freeze_dried_container_model` column.
    fn freeze_dried_container_model(
        mut self,
        freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let freeze_dried_container_model = freeze_dried_container_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::FreezeDriedContainerModel,
                    )
            })?;
        self.freeze_dried_container_model = Some(freeze_dried_container_model);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.foreign_procedure_template` column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let foreign_procedure_template = foreign_procedure_template
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::ForeignProcedureTemplate,
                    )
            })?;
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    ///Sets the value of the `procedure_templates.freeze_drying_procedure_templates.procedure_template_freeze_dried_container_model` column.
    fn procedure_template_freeze_dried_container_model(
        mut self,
        procedure_template_freeze_dried_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let procedure_template_freeze_dried_container_model = procedure_template_freeze_dried_container_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableFreezeDryingProcedureTemplateAttributes::ProcedureTemplateFreezeDriedContainerModel,
                    )
            })?;
        self.procedure_template_freeze_dried_container_model = Some(
            procedure_template_freeze_dried_container_model,
        );
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableFreezeDryingProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes;
    #[inline]
    ///Sets the value of the `procedure_templates.procedure_templates.name` column.
    fn name<'N, N>(
        mut self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'N N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::name(
                self.procedure_template,
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
    ///Sets the value of the `procedure_templates.procedure_templates.description` column.
    fn description<'D, D>(
        mut self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'D D as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::description(
                self.procedure_template,
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
    ///Sets the value of the `procedure_templates.procedure_templates.deprecated` column.
    fn deprecated<'D, D>(
        mut self,
        deprecated: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<&'D D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::deprecated(
                self.procedure_template,
                deprecated,
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
    ///Sets the value of the `procedure_templates.procedure_templates.icon` column.
    fn icon<'I, I>(
        mut self,
        icon: &'I I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'I I: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'I I as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::icon(
                self.procedure_template,
                icon,
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
    ///Sets the value of the `procedure_templates.procedure_templates.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_by(
                self.procedure_template,
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
    ///Sets the value of the `procedure_templates.procedure_templates.created_at` column.
    fn created_at<'CA, CA>(
        mut self,
        created_at: &'CA CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'CA CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'CA CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_at(
                self.procedure_template,
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
    ///Sets the value of the `procedure_templates.procedure_templates.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_by(
                self.procedure_template,
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
    ///Sets the value of the `procedure_templates.procedure_templates.updated_at` column.
    fn updated_at<'UA, UA>(
        mut self,
        updated_at: &'UA UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'UA UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'UA UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_at(
                self.procedure_template,
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
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableFreezeDryingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_template = self.procedure_template.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureTemplate, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableFreezeDryingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableFreezeDryingProcedureTemplateAttributes,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableFreezeDryingProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.pascal.is_some()
            && self.seconds.is_some() && self.freeze_dried_with_model.is_some()
            && self.procedure_template_freeze_dried_with_model.is_some()
            && self.freeze_dried_container_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_freeze_dried_container_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
