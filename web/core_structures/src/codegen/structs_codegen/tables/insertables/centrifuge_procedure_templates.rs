#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableCentrifugeProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableCentrifugeProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeProcedureTemplateAttributes {
    Extension(InsertableCentrifugeProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    RotationPerMinute,
    CentrifugedWithModel,
    ProcedureTemplateCentrifugedWithModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    CentrifugedContainerModel,
    ForeignProcedureTemplate,
    ProcedureTemplateCentrifugedContainerModel,
}
impl core::str::FromStr for InsertableCentrifugeProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "RotationPerMinute" => Ok(Self::RotationPerMinute),
            "CentrifugedWithModel" => Ok(Self::CentrifugedWithModel),
            "ProcedureTemplateCentrifugedWithModel" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "CentrifugedContainerModel" => Ok(Self::CentrifugedContainerModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateCentrifugedContainerModel" => {
                Ok(Self::ProcedureTemplateCentrifugedContainerModel)
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "rotation_per_minute" => Ok(Self::RotationPerMinute),
            "centrifuged_with_model" => Ok(Self::CentrifugedWithModel),
            "procedure_template_centrifuged_with_model" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "centrifuged_container_model" => Ok(Self::CentrifugedContainerModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_centrifuged_container_model" => {
                Ok(Self::ProcedureTemplateCentrifugedContainerModel)
            }
            _ => {
                Err(
                    web_common_traits::database::InsertError::UnknownAttribute(
                        s.to_owned(),
                    ),
                )
            }
        }
    }
}
impl core::fmt::Display for InsertableCentrifugeProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Seconds => write!(f, "seconds"),
            Self::RotationPerMinute => write!(f, "rotation_per_minute"),
            Self::CentrifugedWithModel => write!(f, "centrifuged_with_model"),
            Self::ProcedureTemplateCentrifugedWithModel(e) => write!(f, "{e}"),
            Self::CentrifugedContainerModel => write!(f, "centrifuged_container_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateCentrifugedContainerModel => {
                write!(f, "procedure_template_centrifuged_container_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) seconds: f32,
    pub(crate) rotation_per_minute: f32,
    pub(crate) centrifuged_with_model: i32,
    pub(crate) procedure_template_centrifuged_with_model: i32,
    pub(crate) centrifuged_container_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_centrifuged_container_model: i32,
}
impl InsertableCentrifugeProcedureTemplate {
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
    pub fn centrifuged_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::table(
                ),
                self.centrifuged_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_centrifuged_with_model<
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
                self.procedure_template_centrifuged_with_model,
            ),
            conn,
        )
    }
    pub fn centrifuged_container_model<C: diesel::connection::LoadConnection>(
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
                self.centrifuged_container_model,
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
    pub fn procedure_template_centrifuged_container_model<
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
                self.procedure_template_centrifuged_container_model,
            ),
            conn,
        )
    }
    pub fn centrifuge_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
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
                (self.centrifuged_with_model, self.centrifuged_container_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with_model: Option<i32>,
    pub(crate) procedure_template_centrifuged_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) centrifuged_container_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_centrifuged_container_model: Option<i32>,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> Default for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(1f32),
            seconds: Some(120f32),
            rotation_per_minute: Some(13000f32),
            centrifuged_with_model: Default::default(),
            procedure_template_centrifuged_with_model: Default::default(),
            centrifuged_container_model: Default::default(),
            foreign_procedure_template: Default::default(),
            procedure_template_centrifuged_container_model: Default::default(),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `CentrifugeProcedureTemplate` or descendant tables.
pub trait CentrifugeProcedureTemplateBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.centrifuge_procedure_templates.kelvin`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `public.centrifuge_procedure_templates.kelvin` column.
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
    fn kelvin<K>(
        self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `public.centrifuge_procedure_templates.kelvin_tolerance_percentage`
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
    fn kelvin_tolerance_percentage<KTP>(
        self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>;
    /// Sets the value of the `public.centrifuge_procedure_templates.seconds`
    /// column.
    ///
    /// # Arguments
    /// * `seconds`: The value to set for the
    ///   `public.centrifuge_procedure_templates.seconds` column.
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
    fn seconds<S>(
        self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.rotation_per_minute` column.
    ///
    /// # Arguments
    /// * `rotation_per_minute`: The value to set for the
    ///   `public.centrifuge_procedure_templates.rotation_per_minute` column.
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
    fn rotation_per_minute<RPM>(
        self,
        rotation_per_minute: RPM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        RPM: TryInto<f32>,
        validation_errors::SingleFieldError: From<<RPM as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.centrifuged_with_model` column.
    ///
    /// # Arguments
    /// * `centrifuged_with_model`: The value to set for the
    ///   `public.centrifuge_procedure_templates.centrifuged_with_model` column.
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
    fn centrifuged_with_model(
        self,
        centrifuged_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.
    /// procedure_template_centrifuged_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_centrifuged_with_model`: The value to set for the
    ///   `public.centrifuge_procedure_templates.
    ///   procedure_template_centrifuged_with_model` column.
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
    fn procedure_template_centrifuged_with_model(
        self,
        procedure_template_centrifuged_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.centrifuged_container_model`
    /// column.
    ///
    /// # Arguments
    /// * `centrifuged_container_model`: The value to set for the
    ///   `public.centrifuge_procedure_templates.centrifuged_container_model`
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
    fn centrifuged_container_model(
        self,
        centrifuged_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.foreign_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.centrifuge_procedure_templates.foreign_procedure_template`
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
    fn foreign_procedure_template(
        self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.
    /// procedure_template_centrifuged_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_centrifuged_container_model`: The value to set for
    ///   the `public.centrifuge_procedure_templates.
    ///   procedure_template_centrifuged_container_model` column.
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
    fn procedure_template_centrifuged_container_model(
        self,
        procedure_template_centrifuged_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<ProcedureTemplate> CentrifugeProcedureTemplateBuildable
    for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes;
    /// Sets the value of the `public.centrifuge_procedure_templates.kelvin`
    /// column.
    fn kelvin<K>(
        mut self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCentrifugeProcedureTemplateAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    fn kelvin_tolerance_percentage<KTP>(
        mut self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>,
    {
        let kelvin_tolerance_percentage =
            kelvin_tolerance_percentage.try_into().map_err(|err| {
                validation_errors::SingleFieldError::from(err).rename_field(
                    InsertableCentrifugeProcedureTemplateAttributes::KelvinTolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::KelvinTolerancePercentage,
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
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    /// Sets the value of the `public.centrifuge_procedure_templates.seconds`
    /// column.
    fn seconds<S>(
        mut self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>,
    {
        let seconds = seconds.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCentrifugeProcedureTemplateAttributes::Seconds)
        })?;
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(seconds, 1800f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.rotation_per_minute` column.
    fn rotation_per_minute<RPM>(
        mut self,
        rotation_per_minute: RPM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        RPM: TryInto<f32>,
        validation_errors::SingleFieldError: From<<RPM as TryInto<f32>>::Error>,
    {
        let rotation_per_minute = rotation_per_minute.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCentrifugeProcedureTemplateAttributes::RotationPerMinute)
        })?;
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::RotationPerMinute,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::RotationPerMinute,
                            )
                    })
            })?;
        self.rotation_per_minute = Some(rotation_per_minute);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.centrifuged_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_with_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_with_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn centrifuged_with_model(
        mut self,
        centrifuged_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_centrifuged_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_centrifuged_with_model,
                centrifuged_with_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateCentrifugedWithModel(attribute)
                })
            })?;
        self.centrifuged_with_model = Some(centrifuged_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.
    /// procedure_template_centrifuged_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_centrifuged_with_model(
        mut self,
        mut procedure_template_centrifuged_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.centrifuged_with_model, procedure_template_centrifuged_with_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::CentrifugedWithModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_centrifuged_with_model.asset_model {
            self.centrifuged_with_model = Some(asset_model);
        } else if let Some(local) = self.centrifuged_with_model {
            procedure_template_centrifuged_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_centrifuged_with_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateCentrifugedWithModel(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_template_centrifuged_with_model = procedure_template_centrifuged_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.centrifuged_container_model`
    /// column.
    fn centrifuged_container_model(
        mut self,
        centrifuged_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.centrifuged_container_model = Some(centrifuged_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.foreign_procedure_template`
    /// column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.centrifuge_procedure_templates.
    /// procedure_template_centrifuged_container_model` column.
    fn procedure_template_centrifuged_container_model(
        mut self,
        procedure_template_centrifuged_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_centrifuged_container_model =
            Some(procedure_template_centrifuged_container_model);
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
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
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
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
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(
        mut self,
        deprecated: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
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
    ///Sets the value of the `public.procedure_templates.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
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
    ///Sets the value of the `public.procedure_templates.created_by` column.
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
    ///Sets the value of the `public.procedure_templates.created_at` column.
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
    ///Sets the value of the `public.procedure_templates.updated_by` column.
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
    ///Sets the value of the `public.procedure_templates.updated_at` column.
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
impl<ProcedureTemplate> web_common_traits::database::MostConcreteTable
    for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableCentrifugeProcedureTemplateAttributes,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableCentrifugeProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.seconds.is_some()
            && self.rotation_per_minute.is_some()
            && self.centrifuged_with_model.is_some()
            && self.procedure_template_centrifuged_with_model.is_complete()
            && self.centrifuged_container_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_centrifuged_container_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
