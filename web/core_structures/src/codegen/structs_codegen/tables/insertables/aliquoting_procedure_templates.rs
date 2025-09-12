#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliquotingProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for AliquotingProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for AliquotingProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliquotingProcedureTemplateAttribute {
    Extension(AliquotingProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    Liters,
    AliquotedFromModel,
    ProcedureTemplateAliquotedFromModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    AliquotedIntoModel,
    ProcedureTemplateAliquotedIntoModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    AliquotedWithModel,
    ProcedureTemplateAliquotedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    PipetteTipModel,
    ProcedureTemplatePipetteTipModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for AliquotingProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Liters" => Ok(Self::Liters),
            "AliquotedFromModel" => Ok(Self::AliquotedFromModel),
            "ProcedureTemplateAliquotedFromModel" => {
                Ok(
                    Self::ProcedureTemplateAliquotedFromModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "AliquotedIntoModel" => Ok(Self::AliquotedIntoModel),
            "ProcedureTemplateAliquotedIntoModel" => {
                Ok(
                    Self::ProcedureTemplateAliquotedIntoModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "AliquotedWithModel" => Ok(Self::AliquotedWithModel),
            "ProcedureTemplateAliquotedWithModel" => {
                Ok(
                    Self::ProcedureTemplateAliquotedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "ProcedureTemplatePipetteTipModel" => {
                Ok(
                    Self::ProcedureTemplatePipetteTipModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "liters" => Ok(Self::Liters),
            "aliquoted_from_model" => Ok(Self::AliquotedFromModel),
            "procedure_template_aliquoted_from_model" => {
                Ok(
                    Self::ProcedureTemplateAliquotedFromModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "aliquoted_into_model" => Ok(Self::AliquotedIntoModel),
            "procedure_template_aliquoted_into_model" => {
                Ok(
                    Self::ProcedureTemplateAliquotedIntoModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "aliquoted_with_model" => Ok(Self::AliquotedWithModel),
            "procedure_template_aliquoted_with_model" => {
                Ok(
                    Self::ProcedureTemplateAliquotedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            "procedure_template_pipette_tip_model" => {
                Ok(
                    Self::ProcedureTemplatePipetteTipModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
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
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    > for AliquotingProcedureTemplateAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::ProcedureTemplate
                .into(),
        )
    }
}
impl
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
    > for AliquotingProcedureTemplateAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for AliquotingProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "aliquoting_procedure_templates.procedure_template")
            }
            Self::Liters => write!(f, "aliquoting_procedure_templates.liters"),
            Self::AliquotedFromModel => {
                write!(f, "aliquoting_procedure_templates.aliquoted_from_model")
            }
            Self::ProcedureTemplateAliquotedFromModel(e) => {
                write!(f, "aliquoting_procedure_templates.{e}")
            }
            Self::AliquotedIntoModel => {
                write!(f, "aliquoting_procedure_templates.aliquoted_into_model")
            }
            Self::ProcedureTemplateAliquotedIntoModel(e) => {
                write!(f, "aliquoting_procedure_templates.{e}")
            }
            Self::AliquotedWithModel => {
                write!(f, "aliquoting_procedure_templates.aliquoted_with_model")
            }
            Self::ProcedureTemplateAliquotedWithModel(e) => {
                write!(f, "aliquoting_procedure_templates.{e}")
            }
            Self::PipetteTipModel => {
                write!(f, "aliquoting_procedure_templates.pipette_tip_model")
            }
            Self::ProcedureTemplatePipetteTipModel(e) => {
                write!(f, "aliquoting_procedure_templates.{e}")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) liters: f32,
    pub(crate) aliquoted_from_model: i32,
    pub(crate) procedure_template_aliquoted_from_model: i32,
    pub(crate) aliquoted_into_model: i32,
    pub(crate) procedure_template_aliquoted_into_model: i32,
    pub(crate) aliquoted_with_model: i32,
    pub(crate) procedure_template_aliquoted_with_model: i32,
    pub(crate) pipette_tip_model: i32,
    pub(crate) procedure_template_pipette_tip_model: i32,
}
impl InsertableAliquotingProcedureTemplate {
    pub fn procedure_template_aliquoted_into_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_aliquoted_into_model,
            conn,
        )
    }
    pub fn procedure_template_aliquoted_with_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_aliquoted_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey3(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_aliquoted_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey4(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_aliquoted_from_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_aliquoted_fkey5(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_aliquoted_into_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedure_templat_procedure_template_pipette_t_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_pipette_tip_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn aliquoting_procedure_template_aliquoted_with_model_pipette_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::read(
            (self.aliquoted_with_model, self.pipette_tip_model),
            conn,
        )
    }
    pub fn procedure_template_aliquoted_from_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_aliquoted_from_model,
            conn,
        )
    }
    pub fn procedure_template_pipette_tip_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_pipette_tip_model,
            conn,
        )
    }
    pub fn aliquoted_from_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.aliquoted_from_model,
            conn,
        )
    }
    pub fn aliquoted_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.aliquoted_into_model,
            conn,
        )
    }
    pub fn aliquoted_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::read(
            self.aliquoted_with_model,
            conn,
        )
    }
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
            self.pipette_tip_model,
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`AliquotingProcedureTemplate`](crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::AliquotingProcedureTemplate;
/// use core_structures::tables::insertables::AliquotingProcedureTemplateSettable;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let aliquoting_procedure_template = AliquotingProcedureTemplate::new()
///    // Set mandatory fields
///    .liters(liters)?
///    .procedure_template_aliquoted_from_model(procedure_template_aliquoted_from_model)?
///    .procedure_template_aliquoted_into_model(procedure_template_aliquoted_into_model)?
///    .procedure_template_aliquoted_with_model(procedure_template_aliquoted_with_model)?
///    .procedure_template_pipette_tip_model(procedure_template_pipette_tip_model)?
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .icon(icon)?
///    .number_of_subprocedure_templates(number_of_subprocedure_templates)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAliquotingProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) aliquoted_from_model: Option<i32>,
    pub(crate) procedure_template_aliquoted_from_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) aliquoted_into_model: Option<i32>,
    pub(crate) procedure_template_aliquoted_into_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) aliquoted_with_model: Option<i32>,
    pub(crate) procedure_template_aliquoted_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) procedure_template_pipette_tip_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl From<InsertableAliquotingProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableAliquotingProcedureTemplateBuilder>
{
    fn from(builder: InsertableAliquotingProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    ProcedureTemplate: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.liters.is_some()
            && (self.aliquoted_from_model.is_some()
                || self.procedure_template_aliquoted_from_model.is_complete())
            && self.procedure_template_aliquoted_from_model.is_complete()
            && (self.aliquoted_into_model.is_some()
                || self.procedure_template_aliquoted_into_model.is_complete())
            && self.procedure_template_aliquoted_into_model.is_complete()
            && (self.aliquoted_with_model.is_some()
                || self.procedure_template_aliquoted_with_model.is_complete())
            && self.procedure_template_aliquoted_with_model.is_complete()
            && (self.pipette_tip_model.is_some()
                || self.procedure_template_pipette_tip_model.is_complete())
            && self.procedure_template_pipette_tip_model.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `AliquotingProcedureTemplate` or descendant tables.
pub trait AliquotingProcedureTemplateSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.aliquoting_procedure_templates.liters`
    /// column.
    ///
    /// # Arguments
    /// * `liters`: The value to set for the
    ///   `public.aliquoting_procedure_templates.liters` column.
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
    fn liters<L>(
        self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_from_model` column.
    ///
    /// # Arguments
    /// * `aliquoted_from_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.aliquoted_from_model` column.
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
    fn aliquoted_from_model<AFM>(
        self,
        aliquoted_from_model: AFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_from_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_from_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.
    ///   procedure_template_aliquoted_from_model` column.
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
    fn procedure_template_aliquoted_from_model<PTAFM>(
        self,
        procedure_template_aliquoted_from_model: PTAFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAFM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_into_model` column.
    ///
    /// # Arguments
    /// * `aliquoted_into_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.aliquoted_into_model` column.
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
    fn aliquoted_into_model<AIM>(
        self,
        aliquoted_into_model: AIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_into_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.
    ///   procedure_template_aliquoted_into_model` column.
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
    fn procedure_template_aliquoted_into_model<PTAIM>(
        self,
        procedure_template_aliquoted_into_model: PTAIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_with_model` column.
    ///
    /// # Arguments
    /// * `aliquoted_with_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.aliquoted_with_model` column.
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
    fn aliquoted_with_model<AWM>(
        self,
        aliquoted_with_model: AWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_with_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.
    ///   procedure_template_aliquoted_with_model` column.
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
    fn procedure_template_aliquoted_with_model<PTAWM>(
        self,
        procedure_template_aliquoted_with_model: PTAWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.pipette_tip_model` column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.pipette_tip_model` column.
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
    fn pipette_tip_model<PTM>(
        self,
        pipette_tip_model: PTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_pipette_tip_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_pipette_tip_model`: The value to set for the
    ///   `public.aliquoting_procedure_templates.
    ///   procedure_template_pipette_tip_model` column.
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
    fn procedure_template_pipette_tip_model<PTPTM>(
        self,
        procedure_template_pipette_tip_model: PTPTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPTM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> AliquotingProcedureTemplateSettable
    for InsertableAliquotingProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute;
    /// Sets the value of the `public.aliquoting_procedure_templates.liters`
    /// column.
    fn liters<L>(
        mut self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>,
    {
        let liters = liters.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AliquotingProcedureTemplateAttribute::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::Liters,
                    )
            })?;
        self.liters = Some(liters);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_from_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_from_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_from_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_from_model<AFM>(
        mut self,
        aliquoted_from_model: AFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let aliquoted_from_model =
            <AFM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &aliquoted_from_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_aliquoted_from_model,
        ) = self.procedure_template_aliquoted_from_model
        {
            self.procedure_template_aliquoted_from_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_aliquoted_from_model,
                    aliquoted_from_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateAliquotedFromModel(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_from_model = Some(aliquoted_from_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_from_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_from_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_from_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_from_model<PTAFM>(
        mut self,
        procedure_template_aliquoted_from_model: PTAFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAFM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_aliquoted_from_model =
            procedure_template_aliquoted_from_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_aliquoted_from_model
        {
            procedure_template_aliquoted_from_model = if let (
                Some(aliquoted_from_model),
                Some(asset_model),
            ) =
                (self.aliquoted_from_model, builder.asset_model)
            {
                if aliquoted_from_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedFromModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.aliquoted_from_model = Some(asset_model);
                builder.into()
            } else if let Some(aliquoted_from_model) = self.aliquoted_from_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        aliquoted_from_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateAliquotedFromModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_aliquoted_from_model = procedure_template_aliquoted_from_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_into_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_into_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_into_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_into_model<AIM>(
        mut self,
        aliquoted_into_model: AIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let aliquoted_into_model =
            <AIM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &aliquoted_into_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_aliquoted_into_model,
        ) = self.procedure_template_aliquoted_into_model
        {
            self.procedure_template_aliquoted_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_aliquoted_into_model,
                    aliquoted_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateAliquotedIntoModel(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_into_model = Some(aliquoted_into_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_into_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_into_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_into_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_into_model<PTAIM>(
        mut self,
        procedure_template_aliquoted_into_model: PTAIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_aliquoted_into_model =
            procedure_template_aliquoted_into_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_aliquoted_into_model
        {
            procedure_template_aliquoted_into_model = if let (
                Some(aliquoted_into_model),
                Some(asset_model),
            ) =
                (self.aliquoted_into_model, builder.asset_model)
            {
                if aliquoted_into_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.aliquoted_into_model = Some(asset_model);
                builder.into()
            } else if let Some(aliquoted_into_model) = self.aliquoted_into_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        aliquoted_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateAliquotedIntoModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_aliquoted_into_model = procedure_template_aliquoted_into_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.aliquoted_with_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_with_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_with_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_with_model<AWM>(
        mut self,
        aliquoted_with_model: AWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let aliquoted_with_model =
            <AWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &aliquoted_with_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_aliquoted_with_model,
        ) = self.procedure_template_aliquoted_with_model
        {
            self.procedure_template_aliquoted_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_aliquoted_with_model,
                    aliquoted_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateAliquotedWithModel(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_with_model = Some(aliquoted_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_aliquoted_with_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "aliquoted_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_with_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_with_model<PTAWM>(
        mut self,
        procedure_template_aliquoted_with_model: PTAWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_aliquoted_with_model =
            procedure_template_aliquoted_with_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_aliquoted_with_model
        {
            procedure_template_aliquoted_with_model = if let (
                Some(aliquoted_with_model),
                Some(asset_model),
            ) =
                (self.aliquoted_with_model, builder.asset_model)
            {
                if aliquoted_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.aliquoted_with_model = Some(asset_model);
                builder.into()
            } else if let Some(aliquoted_with_model) = self.aliquoted_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        aliquoted_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateAliquotedWithModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_aliquoted_with_model = procedure_template_aliquoted_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.pipette_tip_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_pipette_tip_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn pipette_tip_model<PTM>(
        mut self,
        pipette_tip_model: PTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let pipette_tip_model =
            <PTM as web_common_traits::database::PrimaryKeyLike>::primary_key(&pipette_tip_model);
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_pipette_tip_model,
        ) = self.procedure_template_pipette_tip_model
        {
            self.procedure_template_pipette_tip_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_pipette_tip_model,
                    pipette_tip_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplatePipetteTipModel(attribute)
                    })
                })?
                .into();
        }
        self.pipette_tip_model = Some(pipette_tip_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedure_templates.
    /// procedure_template_pipette_tip_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`aliquoting_procedure_templates`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_pipette_tip_model"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_pipette_tip_model<PTPTM>(
        mut self,
        procedure_template_pipette_tip_model: PTPTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPTM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_pipette_tip_model = procedure_template_pipette_tip_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_pipette_tip_model
        {
            procedure_template_pipette_tip_model = if let (
                Some(pipette_tip_model),
                Some(asset_model),
            ) =
                (self.pipette_tip_model, builder.asset_model)
            {
                if pipette_tip_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PipetteTipModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.pipette_tip_model = Some(asset_model);
                builder.into()
            } else if let Some(pipette_tip_model) = self.pipette_tip_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        pipette_tip_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplatePipetteTipModel(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_pipette_tip_model = procedure_template_pipette_tip_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertableAliquotingProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute;
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
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
    ///Sets the value of the `public.procedure_templates.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::icon(
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
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
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
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
    ///Sets the value of the `public.procedure_templates.number_of_subprocedure_templates` column.
    fn number_of_subprocedure_templates<NOST>(
        mut self,
        number_of_subprocedure_templates: NOST,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOST: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOST as TryInto<i16>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::number_of_subprocedure_templates(
                self.procedure_template,
                number_of_subprocedure_templates,
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
    for InsertableAliquotingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableAliquotingProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableAliquotingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            AliquotingProcedureTemplateAttribute,
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
    type Attribute = AliquotingProcedureTemplateAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
