#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StorageProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for StorageProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "storage_procedure_templates({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for StorageProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for StorageProcedureTemplateExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StorageProcedureTemplateAttribute {
    Extension(StorageProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    StoredIntoModel,
    ProcedureTemplateStoredIntoModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    StoredAssetModel,
    ProcedureTemplateStoredAssetModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for StorageProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "StoredIntoModel" => Ok(Self::StoredIntoModel),
            "ProcedureTemplateStoredIntoModel" => {
                Ok(
                    Self::ProcedureTemplateStoredIntoModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "StoredAssetModel" => Ok(Self::StoredAssetModel),
            "ProcedureTemplateStoredAssetModel" => {
                Ok(
                    Self::ProcedureTemplateStoredAssetModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "stored_into_model" => Ok(Self::StoredIntoModel),
            "procedure_template_stored_into_model" => {
                Ok(
                    Self::ProcedureTemplateStoredIntoModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "stored_asset_model" => Ok(Self::StoredAssetModel),
            "procedure_template_stored_asset_model" => {
                Ok(
                    Self::ProcedureTemplateStoredAssetModel(
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
for crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder<
    T1,
> {
    type Attribute = StorageProcedureTemplateAttribute;
}
impl web_common_traits::database::TableField for StorageProcedureTemplateAttribute {}
impl web_common_traits::database::HasTableType for StorageProcedureTemplateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    > for StorageProcedureTemplateAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        StorageProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for StorageProcedureTemplateAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        StorageProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for StorageProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "storage_procedure_templates.procedure_template")
            }
            Self::Kelvin => write!(f, "storage_procedure_templates.kelvin"),
            Self::KelvinTolerancePercentage => {
                write!(f, "storage_procedure_templates.kelvin_tolerance_percentage")
            }
            Self::StoredIntoModel => {
                write!(f, "storage_procedure_templates.stored_into_model")
            }
            Self::ProcedureTemplateStoredIntoModel(e) => {
                write!(f, "storage_procedure_templates.{e}")
            }
            Self::StoredAssetModel => {
                write!(f, "storage_procedure_templates.stored_asset_model")
            }
            Self::ProcedureTemplateStoredAssetModel(e) => {
                write!(f, "storage_procedure_templates.{e}")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) stored_into_model: i32,
    pub(crate) procedure_template_stored_into_model: i32,
    pub(crate) stored_asset_model: i32,
    pub(crate) procedure_template_stored_asset_model: i32,
}
impl InsertableStorageProcedureTemplate {
    #[cfg(feature = "postgres")]
    pub fn storage_procedure_templates_procedure_template_stored_ass_fkey1(
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
                    .eq(&self.procedure_template_stored_asset_model_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.stored_asset_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_stored_asset_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_stored_asset_model_id,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedure_templates_procedure_template_stored_int_fkey1(
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
                    .eq(&self.procedure_template_stored_into_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.stored_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_stored_into_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_stored_into_model,
            conn,
        )
    }
    pub fn stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::read(
            self.stored_asset_model_id,
            conn,
        )
    }
    pub fn stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
            self.stored_into_model,
            conn,
        )
    }
    pub fn storage_procedure_templates_stored_into_model_stored_asset_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule::read(
            (self.stored_into_model, self.stored_asset_model_id),
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`StorageProcedureTemplate`](crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::StorageProcedureTemplate;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use core_structures::tables::insertables::StorageProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let storage_procedure_template = StorageProcedureTemplate::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .procedure_template_stored_asset_model(procedure_template_stored_asset_model_id)?
///    .procedure_template_stored_into_model(procedure_template_stored_into_model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    .kelvin(kelvin)?
///    .kelvin_tolerance_percentage(kelvin_tolerance_percentage)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableStorageProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) stored_into_model: Option<i32>,
    pub(crate) procedure_template_stored_into_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) stored_asset_model: Option<i32>,
    pub(crate) procedure_template_stored_asset_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> diesel::associations::HasTable
    for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
{
    type Table = crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::table
    }
}
impl From<InsertableStorageProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableStorageProcedureTemplateBuilder>
{
    fn from(builder: InsertableStorageProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ProcedureTemplate> Default for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(1f32),
            stored_into_model: Default::default(),
            procedure_template_stored_into_model: Default::default(),
            stored_asset_model: Default::default(),
            procedure_template_stored_asset_model: Default::default(),
        }
    }
}
impl<ProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    ProcedureTemplate: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some()
            && (self.stored_into_model.is_some()
                || self.procedure_template_stored_into_model.is_complete())
            && self.procedure_template_stored_into_model.is_complete()
            && (self.stored_asset_model.is_some()
                || self.procedure_template_stored_asset_model.is_complete())
            && self.procedure_template_stored_asset_model.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `StorageProcedureTemplate` or descendant tables.
pub trait StorageProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.storage_procedure_templates.kelvin`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `public.storage_procedure_templates.kelvin` column.
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
    /// `public.storage_procedure_templates.kelvin_tolerance_percentage` column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `public.storage_procedure_templates.kelvin_tolerance_percentage`
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
    /// Sets the value of the
    /// `public.storage_procedure_templates.stored_into_model` column.
    ///
    /// # Arguments
    /// * `stored_into_model`: The value to set for the
    ///   `public.storage_procedure_templates.stored_into_model` column.
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
    fn stored_into_model<SIM>(self, stored_into_model: SIM) -> Result<Self, Self::Error>
    where
        SIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.storage_procedure_templates.
    /// procedure_template_stored_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_stored_into_model`: The value to set for the
    ///   `public.storage_procedure_templates.
    ///   procedure_template_stored_into_model` column.
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
    fn procedure_template_stored_into_model<PTSIM>(
        self,
        procedure_template_stored_into_model: PTSIM,
    ) -> Result<Self, Self::Error>
    where
        PTSIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.storage_procedure_templates.stored_asset_model` column.
    ///
    /// # Arguments
    /// * `stored_asset_model`: The value to set for the
    ///   `public.storage_procedure_templates.stored_asset_model` column.
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
    fn stored_asset_model<SAM>(self, stored_asset_model: SAM) -> Result<Self, Self::Error>
    where
        SAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.storage_procedure_templates.
    /// procedure_template_stored_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_stored_asset_model`: The value to set for the
    ///   `public.storage_procedure_templates.
    ///   procedure_template_stored_asset_model` column.
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
    fn procedure_template_stored_asset_model<PTSAM>(
        self,
        procedure_template_stored_asset_model: PTSAM,
    ) -> Result<Self, Self::Error>
    where
        PTSAM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> StorageProcedureTemplateSettable
for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.storage_procedure_templates.kelvin` column.
    fn kelvin<K>(mut self, kelvin: K) -> Result<Self, Self::Error>
    where
        K: TryInto<f32>,
        validation_errors::prelude::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(StorageProcedureTemplateAttribute::Kelvin)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    ///Sets the value of the `public.storage_procedure_templates.kelvin_tolerance_percentage` column.
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
                        StorageProcedureTemplateAttribute::KelvinTolerancePercentage,
                    )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::KelvinTolerancePercentage,
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
                                crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    ///Sets the value of the `public.storage_procedure_templates.stored_into_model` column.
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
    ///subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    ///class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`storage_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_stored_into_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "stored_into_model"}
    ///class v2 column-of-interest
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn stored_into_model<SIM>(
        mut self,
        stored_into_model: SIM,
    ) -> Result<Self, Self::Error>
    where
        SIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let stored_into_model = <SIM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &stored_into_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_stored_into_model,
        ) = self.procedure_template_stored_into_model
        {
            self.procedure_template_stored_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_stored_into_model,
                    stored_into_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateStoredIntoModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.stored_into_model = Some(stored_into_model);
        Ok(self)
    }
    ///Sets the value of the `public.storage_procedure_templates.procedure_template_stored_into_model` column.
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
    ///subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    ///class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`storage_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_stored_into_model"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "stored_into_model"}
    ///class v2 directly-involved-column
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn procedure_template_stored_into_model<PTSIM>(
        mut self,
        procedure_template_stored_into_model: PTSIM,
    ) -> Result<Self, Self::Error>
    where
        PTSIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_stored_into_model = procedure_template_stored_into_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_stored_into_model {
            procedure_template_stored_into_model = if let (
                Some(stored_into_model),
                Some(asset_model_id),
            ) = (self.stored_into_model, builder.asset_model_id) {
                if stored_into_model != asset_model_id {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::StoredIntoModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model_id) = builder.asset_model_id {
                self.stored_into_model = Some(asset_model_id);
                builder.into()
            } else if let Some(stored_into_model) = self.stored_into_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        stored_into_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateStoredIntoModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_stored_into_model = procedure_template_stored_into_model;
        Ok(self)
    }
    ///Sets the value of the `public.storage_procedure_templates.stored_asset_model` column.
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
    ///subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    ///class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`storage_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_stored_asset_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "stored_asset_model"}
    ///class v2 column-of-interest
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn stored_asset_model<SAM>(
        mut self,
        stored_asset_model: SAM,
    ) -> Result<Self, Self::Error>
    where
        SAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let stored_asset_model_id = <SAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &stored_asset_model_id,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_stored_asset_model_id,
        ) = self.procedure_template_stored_asset_model
        {
            self.procedure_template_stored_asset_model_id = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_stored_asset_model_id,
                    stored_asset_model_id,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateStoredAssetModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.stored_asset_model_id = Some(stored_asset_model_id);
        Ok(self)
    }
    ///Sets the value of the `public.storage_procedure_templates.procedure_template_stored_asset_model` column.
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
    ///subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    ///class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`storage_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_stored_asset_model"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "stored_asset_model"}
    ///class v2 directly-involved-column
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn procedure_template_stored_asset_model<PTSAM>(
        mut self,
        procedure_template_stored_asset_model: PTSAM,
    ) -> Result<Self, Self::Error>
    where
        PTSAM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_stored_asset_model_id = procedure_template_stored_asset_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_stored_asset_model_id {
            procedure_template_stored_asset_model_id = if let (
                Some(stored_asset_model_id),
                Some(asset_model_id),
            ) = (self.stored_asset_model_id, builder.asset_model_id) {
                if stored_asset_model_id != asset_model_id {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::StoredAssetModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model_id) = builder.asset_model_id {
                self.stored_asset_model_id = Some(asset_model_id);
                builder.into()
            } else if let Some(stored_asset_model_id) = self.stored_asset_model_id {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        stored_asset_model_id,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateStoredAssetModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_stored_asset_model_id = procedure_template_stored_asset_model;
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
for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
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
    for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableStorageProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
            Error = web_common_traits::database::InsertError<
                StorageProcedureTemplateAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        StorageProcedureTemplateAttribute,
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
