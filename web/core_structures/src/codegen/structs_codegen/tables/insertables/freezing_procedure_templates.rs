#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezingProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableFreezingProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableFreezingProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezingProcedureTemplateAttributes {
    Extension(InsertableFreezingProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    FrozenWithModel,
    ProcedureTemplateFrozenWithModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    FrozenContainerModel,
    ForeignProcedureTemplate,
    ProcedureTemplateFrozenContainerModel,
}
impl core::str::FromStr for InsertableFreezingProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "FrozenWithModel" => Ok(Self::FrozenWithModel),
            "ProcedureTemplateFrozenWithModel" => {
                Ok(
                    Self::ProcedureTemplateFrozenWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "FrozenContainerModel" => Ok(Self::FrozenContainerModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateFrozenContainerModel" => {
                Ok(Self::ProcedureTemplateFrozenContainerModel)
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "frozen_with_model" => Ok(Self::FrozenWithModel),
            "procedure_template_frozen_with_model" => {
                Ok(
                    Self::ProcedureTemplateFrozenWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "frozen_container_model" => Ok(Self::FrozenContainerModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_frozen_container_model" => {
                Ok(Self::ProcedureTemplateFrozenContainerModel)
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
impl core::fmt::Display for InsertableFreezingProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Seconds => write!(f, "seconds"),
            Self::FrozenWithModel => write!(f, "frozen_with_model"),
            Self::ProcedureTemplateFrozenWithModel(e) => write!(f, "{e}"),
            Self::FrozenContainerModel => write!(f, "frozen_container_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateFrozenContainerModel => {
                write!(f, "procedure_template_frozen_container_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::freezing_procedure_templates::freezing_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezingProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) seconds: Option<f32>,
    pub(crate) frozen_with_model: i32,
    pub(crate) procedure_template_frozen_with_model: i32,
    pub(crate) frozen_container_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_frozen_container_model: i32,
}
impl InsertableFreezingProcedureTemplate {
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
    pub fn frozen_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::table(),
                self.frozen_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_frozen_with_model<C: diesel::connection::LoadConnection>(
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
                self.procedure_template_frozen_with_model,
            ),
            conn,
        )
    }
    pub fn frozen_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::container_models::ContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::container_models::ContainerModel::table(),
                self.frozen_container_model,
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
    pub fn procedure_template_frozen_container_model<
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
                self.procedure_template_frozen_container_model,
            ),
            conn,
        )
    }
    pub fn freezing_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
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
                (self.frozen_with_model, self.frozen_container_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) frozen_with_model: Option<i32>,
    pub(crate) procedure_template_frozen_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) frozen_container_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_frozen_container_model: Option<i32>,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> Default for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(203.15f32),
            kelvin_tolerance_percentage: Some(5f32),
            seconds: Some(43200f32),
            frozen_with_model: Default::default(),
            procedure_template_frozen_with_model: Default::default(),
            frozen_container_model: Default::default(),
            foreign_procedure_template: Default::default(),
            procedure_template_frozen_container_model: Default::default(),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `FreezingProcedureTemplate` or descendant tables.
pub trait FreezingProcedureTemplateBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.freezing_procedure_templates.kelvin`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `public.freezing_procedure_templates.kelvin` column.
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
    /// `public.freezing_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `public.freezing_procedure_templates.kelvin_tolerance_percentage`
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
    /// Sets the value of the `public.freezing_procedure_templates.seconds`
    /// column.
    ///
    /// # Arguments
    /// * `seconds`: The value to set for the
    ///   `public.freezing_procedure_templates.seconds` column.
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
        S: TryInto<Option<f32>>,
        validation_errors::SingleFieldError: From<<S as TryInto<Option<f32>>>::Error>;
    /// Sets the value of the
    /// `public.freezing_procedure_templates.frozen_with_model` column.
    ///
    /// # Arguments
    /// * `frozen_with_model`: The value to set for the
    ///   `public.freezing_procedure_templates.frozen_with_model` column.
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
    fn frozen_with_model(
        self,
        frozen_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.freezing_procedure_templates.
    /// procedure_template_frozen_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_frozen_with_model`: The value to set for the
    ///   `public.freezing_procedure_templates.
    ///   procedure_template_frozen_with_model` column.
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
    fn procedure_template_frozen_with_model(
        self,
        procedure_template_frozen_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.freezing_procedure_templates.frozen_container_model` column.
    ///
    /// # Arguments
    /// * `frozen_container_model`: The value to set for the
    ///   `public.freezing_procedure_templates.frozen_container_model` column.
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
    fn frozen_container_model(
        self,
        frozen_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.freezing_procedure_templates.foreign_procedure_template` column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.freezing_procedure_templates.foreign_procedure_template`
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
    /// `public.freezing_procedure_templates.
    /// procedure_template_frozen_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_frozen_container_model`: The value to set for the
    ///   `public.freezing_procedure_templates.
    ///   procedure_template_frozen_container_model` column.
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
    fn procedure_template_frozen_container_model(
        self,
        procedure_template_frozen_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<ProcedureTemplate> FreezingProcedureTemplateBuildable
    for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes;
    /// Sets the value of the `public.freezing_procedure_templates.kelvin`
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
                .rename_field(InsertableFreezingProcedureTemplateAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.kelvin_tolerance_percentage`
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
                    InsertableFreezingProcedureTemplateAttributes::KelvinTolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes::KelvinTolerancePercentage,
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
                                crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    /// Sets the value of the `public.freezing_procedure_templates.seconds`
    /// column.
    fn seconds<S>(
        mut self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<Option<f32>>,
        validation_errors::SingleFieldError: From<<S as TryInto<Option<f32>>>::Error>,
    {
        let seconds = seconds.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableFreezingProcedureTemplateAttributes::Seconds)
        })?;
        if let Some(seconds) = seconds {
            pgrx_validation::must_be_strictly_positive_f32(seconds)
                .map_err(|e| {
                    e
                        .rename_field(
                            crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes::Seconds,
                        )
                })
                .and_then(|_| {
                    pgrx_validation::must_be_strictly_greater_than_f32(seconds, 1800f32)
                        .map_err(|e| {
                            e
                                .rename_field(
                                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes::Seconds,
                                )
                        })
                })?;
        }
        self.seconds = seconds;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.frozen_with_model` column.
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
    /// subgraph v4 ["`freezing_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_frozen_with_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "frozen_with_model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn frozen_with_model(
        mut self,
        frozen_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_frozen_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_frozen_with_model,
                frozen_with_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateFrozenWithModel(attribute)
                })
            })?;
        self.frozen_with_model = Some(frozen_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.
    /// procedure_template_frozen_with_model` column.
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
    /// subgraph v4 ["`freezing_procedure_templates`"]
    ///    v0@{shape: rounded, label: "frozen_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_frozen_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_frozen_with_model(
        mut self,
        mut procedure_template_frozen_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.frozen_with_model, procedure_template_frozen_with_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::FrozenWithModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_frozen_with_model.asset_model {
            self.frozen_with_model = Some(asset_model);
        } else if let Some(local) = self.frozen_with_model {
            procedure_template_frozen_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_frozen_with_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateFrozenWithModel(attribute)
                    })
                })?;
        }
        self.procedure_template_frozen_with_model = procedure_template_frozen_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.frozen_container_model` column.
    fn frozen_container_model(
        mut self,
        frozen_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.frozen_container_model = Some(frozen_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.foreign_procedure_template` column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.freezing_procedure_templates.
    /// procedure_template_frozen_container_model` column.
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
    /// v0@{shape: rounded, label: "foreign_procedure_template"}
    /// class v0 directly-involved-column
    /// v1@{shape: rounded, label: "frozen_container_model"}
    /// class v1 directly-involved-column
    /// v2@{shape: rounded, label: "procedure_template_frozen_container_model"}
    /// class v2 column-of-interest
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// ```
    fn procedure_template_frozen_container_model(
        mut self,
        procedure_template_frozen_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_frozen_container_model =
            Some(procedure_template_frozen_container_model);
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateAttributes;
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
    for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableFreezingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableFreezingProcedureTemplateAttributes,
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
    type Attributes = InsertableFreezingProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some()
            && self.frozen_with_model.is_some()
            && self.procedure_template_frozen_with_model.is_complete()
            && self.frozen_container_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_frozen_container_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
