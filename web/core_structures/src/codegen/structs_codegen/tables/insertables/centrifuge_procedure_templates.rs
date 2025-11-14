#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CentrifugeProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for CentrifugeProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => {
                write!(f, "centrifuge_procedure_templates({e})")
            }
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for CentrifugeProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for CentrifugeProcedureTemplateExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CentrifugeProcedureTemplateAttribute {
    Extension(CentrifugeProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    RotationPerMinute,
    CentrifugedWithModel,
    ProcedureTemplateCentrifugedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    CentrifugedContainerModel,
    ProcedureTemplateCentrifugedContainerModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for CentrifugeProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "RotationPerMinute" => Ok(Self::RotationPerMinute),
            "CentrifugedWithModel" => Ok(Self::CentrifugedWithModel),
            "ProcedureTemplateCentrifugedWithModel" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "CentrifugedContainerModel" => Ok(Self::CentrifugedContainerModel),
            "ProcedureTemplateCentrifugedContainerModel" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "rotation_per_minute" => Ok(Self::RotationPerMinute),
            "centrifuged_with_model" => Ok(Self::CentrifugedWithModel),
            "procedure_template_centrifuged_with_model" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "centrifuged_container_model" => Ok(Self::CentrifugedContainerModel),
            "procedure_template_centrifuged_container_model" => {
                Ok(
                    Self::ProcedureTemplateCentrifugedContainerModel(
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
impl<T1> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    T1,
> {
    type Attribute = CentrifugeProcedureTemplateAttribute;
}
impl web_common_traits::database::TableField for CentrifugeProcedureTemplateAttribute {}
impl web_common_traits::database::HasTableType for CentrifugeProcedureTemplateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    > for CentrifugeProcedureTemplateAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        CentrifugeProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for CentrifugeProcedureTemplateAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        CentrifugeProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for CentrifugeProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "centrifuge_procedure_templates.procedure_template")
            }
            Self::Kelvin => write!(f, "centrifuge_procedure_templates.kelvin"),
            Self::KelvinTolerancePercentage => {
                write!(f, "centrifuge_procedure_templates.kelvin_tolerance_percentage")
            }
            Self::Seconds => write!(f, "centrifuge_procedure_templates.seconds"),
            Self::RotationPerMinute => {
                write!(f, "centrifuge_procedure_templates.rotation_per_minute")
            }
            Self::CentrifugedWithModel => {
                write!(f, "centrifuge_procedure_templates.centrifuged_with_model")
            }
            Self::ProcedureTemplateCentrifugedWithModel(e) => {
                write!(f, "centrifuge_procedure_templates.{e}")
            }
            Self::CentrifugedContainerModel => {
                write!(f, "centrifuge_procedure_templates.centrifuged_container_model")
            }
            Self::ProcedureTemplateCentrifugedContainerModel(e) => {
                write!(f, "centrifuge_procedure_templates.{e}")
            }
        }
    }
}
#[derive(Debug)]
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
    pub(crate) procedure_template_centrifuged_container_model: i32,
}
impl InsertableCentrifugeProcedureTemplate {
    pub fn centrifuge_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
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
            (self.centrifuged_with_model, self.centrifuged_container_model),
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
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_centrifuged_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedure_templat_procedure_template_centrifug_fkey2(
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
                    .eq(&self.procedure_template_centrifuged_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.centrifuged_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn centrifuge_procedure_templat_procedure_template_centrifug_fkey3(
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
                    .eq(&self.procedure_template_centrifuged_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.centrifuged_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
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
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_centrifuged_with_model,
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
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.centrifuged_container_model,
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
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::read(
            self.centrifuged_with_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CentrifugeProcedureTemplate`](crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CentrifugeProcedureTemplate;
/// use core_structures::tables::insertables::CentrifugeProcedureTemplateSettable;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let centrifuge_procedure_template = CentrifugeProcedureTemplate::new()
///    // Set mandatory fields
///    .procedure_template_centrifuged_container_model(procedure_template_centrifuged_container_model)?
///    .procedure_template_centrifuged_with_model(procedure_template_centrifuged_with_model)?
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .kelvin(kelvin)?
///    .kelvin_tolerance_percentage(kelvin_tolerance_percentage)?
///    .rotation_per_minute(rotation_per_minute)?
///    .seconds(seconds)?
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) rotation_per_minute: Option<f32>,
    pub(crate) centrifuged_with_model: Option<i32>,
    pub(crate) procedure_template_centrifuged_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) centrifuged_container_model: Option<i32>,
    pub(crate) procedure_template_centrifuged_container_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> diesel::associations::HasTable
    for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
{
    type Table = crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::centrifuge_procedure_templates::centrifuge_procedure_templates::table
    }
}
impl From<InsertableCentrifugeProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCentrifugeProcedureTemplateBuilder>
{
    fn from(builder: InsertableCentrifugeProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
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
            procedure_template_centrifuged_container_model: Default::default(),
        }
    }
}
impl<ProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    ProcedureTemplate: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.seconds.is_some()
            && self.rotation_per_minute.is_some()
            && (self.centrifuged_with_model.is_some()
                || self.procedure_template_centrifuged_with_model.is_complete())
            && self.procedure_template_centrifuged_with_model.is_complete()
            && (self.centrifuged_container_model.is_some()
                || self.procedure_template_centrifuged_container_model.is_complete())
            && self.procedure_template_centrifuged_container_model.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CentrifugeProcedureTemplate` or descendant tables.
pub trait CentrifugeProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
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
    fn kelvin<K>(self, kelvin: K) -> Result<Self, Self::Error>
    where
        K: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<K as TryInto<f32>>::Error>;
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
    ) -> Result<Self, Self::Error>
    where
        KTP: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<KTP as TryInto<f32>>::Error>;
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
    fn seconds<S>(self, seconds: S) -> Result<Self, Self::Error>
    where
        S: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<S as TryInto<f32>>::Error>;
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
    fn rotation_per_minute<RPM>(self, rotation_per_minute: RPM) -> Result<Self, Self::Error>
    where
        RPM: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<RPM as TryInto<f32>>::Error>;
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
    fn centrifuged_with_model<CWM>(self, centrifuged_with_model: CWM) -> Result<Self, Self::Error>
    where
        CWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
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
    fn procedure_template_centrifuged_with_model<PTCWM>(
        self,
        procedure_template_centrifuged_with_model: PTCWM,
    ) -> Result<Self, Self::Error>
    where
        PTCWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
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
    fn centrifuged_container_model<CCM>(
        self,
        centrifuged_container_model: CCM,
    ) -> Result<Self, Self::Error>
    where
        CCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
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
    fn procedure_template_centrifuged_container_model<PTCCM>(
        self,
        procedure_template_centrifuged_container_model: PTCCM,
    ) -> Result<Self, Self::Error>
    where
        PTCCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> CentrifugeProcedureTemplateSettable
for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.centrifuge_procedure_templates.kelvin` column.
    fn kelvin<K>(mut self, kelvin: K) -> Result<Self, Self::Error>
    where
        K: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(CentrifugeProcedureTemplateAttribute::Kelvin)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.kelvin_tolerance_percentage` column.
    fn kelvin_tolerance_percentage<KTP>(
        mut self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, Self::Error>
    where
        KTP: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<KTP as TryInto<f32>>::Error>,
    {
        let kelvin_tolerance_percentage = kelvin_tolerance_percentage
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(
                        CentrifugeProcedureTemplateAttribute::KelvinTolerancePercentage,
                    )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::KelvinTolerancePercentage,
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
                                crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.seconds` column.
    fn seconds<S>(mut self, seconds: S) -> Result<Self, Self::Error>
    where
        S: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<S as TryInto<f32>>::Error>,
    {
        let seconds = seconds
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(CentrifugeProcedureTemplateAttribute::Seconds)
            })?;
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(seconds, 1800f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.rotation_per_minute` column.
    fn rotation_per_minute<RPM>(
        mut self,
        rotation_per_minute: RPM,
    ) -> Result<Self, Self::Error>
    where
        RPM: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<RPM as TryInto<f32>>::Error>,
    {
        let rotation_per_minute = rotation_per_minute
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(
                        CentrifugeProcedureTemplateAttribute::RotationPerMinute,
                    )
            })?;
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::RotationPerMinute,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::RotationPerMinute,
                            )
                    })
            })?;
        self.rotation_per_minute = Some(rotation_per_minute);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.centrifuged_with_model` column.
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
    ///subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_with_model"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_with_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn centrifuged_with_model<CWM>(
        mut self,
        centrifuged_with_model: CWM,
    ) -> Result<Self, Self::Error>
    where
        CWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let centrifuged_with_model = <CWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &centrifuged_with_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_centrifuged_with_model,
        ) = self.procedure_template_centrifuged_with_model
        {
            self.procedure_template_centrifuged_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_centrifuged_with_model,
                    centrifuged_with_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateCentrifugedWithModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.centrifuged_with_model = Some(centrifuged_with_model);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.procedure_template_centrifuged_with_model` column.
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
    ///subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_with_model"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_with_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn procedure_template_centrifuged_with_model<PTCWM>(
        mut self,
        procedure_template_centrifuged_with_model: PTCWM,
    ) -> Result<Self, Self::Error>
    where
        PTCWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_centrifuged_with_model = procedure_template_centrifuged_with_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_centrifuged_with_model {
            procedure_template_centrifuged_with_model = if let (
                Some(centrifuged_with_model),
                Some(asset_model),
            ) = (self.centrifuged_with_model, builder.asset_model) {
                if centrifuged_with_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::CentrifugedWithModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.centrifuged_with_model = Some(asset_model);
                builder.into()
            } else if let Some(centrifuged_with_model) = self.centrifuged_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        centrifuged_with_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateCentrifugedWithModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_centrifuged_with_model = procedure_template_centrifuged_with_model;
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.centrifuged_container_model` column.
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
    ///subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_container_model"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_container_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn centrifuged_container_model<CCM>(
        mut self,
        centrifuged_container_model: CCM,
    ) -> Result<Self, Self::Error>
    where
        CCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let centrifuged_container_model = <CCM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &centrifuged_container_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_centrifuged_container_model,
        ) = self.procedure_template_centrifuged_container_model
        {
            self.procedure_template_centrifuged_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_centrifuged_container_model,
                    centrifuged_container_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateCentrifugedContainerModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.centrifuged_container_model = Some(centrifuged_container_model);
        Ok(self)
    }
    ///Sets the value of the `public.centrifuge_procedure_templates.procedure_template_centrifuged_container_model` column.
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
    ///subgraph v4 ["`centrifuge_procedure_templates`"]
    ///    v0@{shape: rounded, label: "centrifuged_container_model"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_centrifuged_container_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn procedure_template_centrifuged_container_model<PTCCM>(
        mut self,
        procedure_template_centrifuged_container_model: PTCCM,
    ) -> Result<Self, Self::Error>
    where
        PTCCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_centrifuged_container_model = procedure_template_centrifuged_container_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_centrifuged_container_model {
            procedure_template_centrifuged_container_model = if let (
                Some(centrifuged_container_model),
                Some(asset_model),
            ) = (self.centrifuged_container_model, builder.asset_model) {
                if centrifuged_container_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::CentrifugedContainerModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.centrifuged_container_model = Some(asset_model);
                builder.into()
            } else if let Some(centrifuged_container_model) = self
                .centrifuged_container_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        centrifuged_container_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateCentrifugedContainerModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_centrifuged_container_model = procedure_template_centrifuged_container_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertableCentrifugeProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                self.procedure_template,
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
    #[inline]
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(mut self, deprecated: D) -> Result<Self, Self::Error>
    where
        D: TryInto<bool>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                self.procedure_template,
                deprecated,
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
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
            Error = web_common_traits::database::InsertError<
                CentrifugeProcedureTemplateAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        CentrifugeProcedureTemplateAttribute,
    >;
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
