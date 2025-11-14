#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WeighingProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for WeighingProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "weighing_procedure_templates({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for WeighingProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for WeighingProcedureTemplateExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WeighingProcedureTemplateAttribute {
    Extension(WeighingProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    WeighedContainerModel,
    ProcedureTemplateWeighedContainerModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    WeighedWithModel,
    ProcedureTemplateWeighedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for WeighingProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "WeighedContainerModel" => Ok(Self::WeighedContainerModel),
            "ProcedureTemplateWeighedContainerModel" => {
                Ok(
                    Self::ProcedureTemplateWeighedContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "WeighedWithModel" => Ok(Self::WeighedWithModel),
            "ProcedureTemplateWeighedWithModel" => {
                Ok(
                    Self::ProcedureTemplateWeighedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "weighed_container_model" => Ok(Self::WeighedContainerModel),
            "procedure_template_weighed_container_model" => {
                Ok(
                    Self::ProcedureTemplateWeighedContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "weighed_with_model" => Ok(Self::WeighedWithModel),
            "procedure_template_weighed_with_model" => {
                Ok(
                    Self::ProcedureTemplateWeighedWithModel(
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
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder<
    T1,
> {
    type Attribute = WeighingProcedureTemplateAttribute;
}
impl web_common_traits::database::TableField for WeighingProcedureTemplateAttribute {}
impl web_common_traits::database::HasTableType for WeighingProcedureTemplateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    > for WeighingProcedureTemplateAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        WeighingProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for WeighingProcedureTemplateAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        WeighingProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for WeighingProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "weighing_procedure_templates.procedure_template")
            }
            Self::WeighedContainerModel => {
                write!(f, "weighing_procedure_templates.weighed_container_model")
            }
            Self::ProcedureTemplateWeighedContainerModel(e) => {
                write!(f, "weighing_procedure_templates.{e}")
            }
            Self::WeighedWithModel => {
                write!(f, "weighing_procedure_templates.weighed_with_model")
            }
            Self::ProcedureTemplateWeighedWithModel(e) => {
                write!(f, "weighing_procedure_templates.{e}")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) weighed_container_model: i32,
    pub(crate) procedure_template_weighed_container_model: i32,
    pub(crate) weighed_with_model: i32,
    pub(crate) procedure_template_weighed_with_model: i32,
}
impl InsertableWeighingProcedureTemplate {
    #[cfg(feature = "postgres")]
    pub fn weighing_procedure_templates_procedure_template_weighed_c_fkey1(
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
                    .eq(&self.procedure_template_weighed_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.weighed_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_weighed_container_model<
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
            self.procedure_template_weighed_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn weighing_procedure_templates_procedure_template_weighed_w_fkey1(
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
                    .eq(&self.procedure_template_weighed_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.weighed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_weighed_with_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_weighed_with_model,
            conn,
        )
    }
    pub fn weighed_container_model<C: diesel::connection::LoadConnection>(
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
            self.weighed_container_model,
            conn,
        )
    }
    pub fn weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::read(
            self.weighed_with_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`WeighingProcedureTemplate`](crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::WeighingProcedureTemplate;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use core_structures::tables::insertables::WeighingProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let weighing_procedure_template = WeighingProcedureTemplate::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .procedure_template_weighed_container_model(procedure_template_weighed_container_model)?
///    .procedure_template_weighed_with_model(procedure_template_weighed_with_model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableWeighingProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) weighed_container_model: Option<i32>,
    pub(crate) procedure_template_weighed_container_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) weighed_with_model: Option<i32>,
    pub(crate) procedure_template_weighed_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> diesel::associations::HasTable
    for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
{
    type Table = crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::table
    }
}
impl From<InsertableWeighingProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableWeighingProcedureTemplateBuilder>
{
    fn from(builder: InsertableWeighingProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    ProcedureTemplate: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete()
            && (self.weighed_container_model.is_some()
                || self.procedure_template_weighed_container_model.is_complete())
            && self.procedure_template_weighed_container_model.is_complete()
            && (self.weighed_with_model.is_some()
                || self.procedure_template_weighed_with_model.is_complete())
            && self.procedure_template_weighed_with_model.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `WeighingProcedureTemplate` or descendant tables.
pub trait WeighingProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.weighing_procedure_templates.weighed_container_model` column.
    ///
    /// # Arguments
    /// * `weighed_container_model`: The value to set for the
    ///   `public.weighing_procedure_templates.weighed_container_model` column.
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
    fn weighed_container_model<WCM>(
        self,
        weighed_container_model: WCM,
    ) -> Result<Self, Self::Error>
    where
        WCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.weighing_procedure_templates.
    /// procedure_template_weighed_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_weighed_container_model`: The value to set for the
    ///   `public.weighing_procedure_templates.
    ///   procedure_template_weighed_container_model` column.
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
    fn procedure_template_weighed_container_model<PTWCM>(
        self,
        procedure_template_weighed_container_model: PTWCM,
    ) -> Result<Self, Self::Error>
    where
        PTWCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.weighing_procedure_templates.weighed_with_model` column.
    ///
    /// # Arguments
    /// * `weighed_with_model`: The value to set for the
    ///   `public.weighing_procedure_templates.weighed_with_model` column.
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
    fn weighed_with_model<WWM>(self, weighed_with_model: WWM) -> Result<Self, Self::Error>
    where
        WWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.weighing_procedure_templates.
    /// procedure_template_weighed_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_weighed_with_model`: The value to set for the
    ///   `public.weighing_procedure_templates.
    ///   procedure_template_weighed_with_model` column.
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
    fn procedure_template_weighed_with_model<PTWWM>(
        self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, Self::Error>
    where
        PTWWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> WeighingProcedureTemplateSettable
for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::WeighingProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.weighing_procedure_templates.weighed_container_model` column.
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
    ///subgraph v5 ["`weighing_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_weighed_container_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "weighed_container_model"}
    ///class v2 column-of-interest
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn weighed_container_model<WCM>(
        mut self,
        weighed_container_model: WCM,
    ) -> Result<Self, Self::Error>
    where
        WCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let weighed_container_model = <WCM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &weighed_container_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_weighed_container_model,
        ) = self.procedure_template_weighed_container_model
        {
            self.procedure_template_weighed_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_weighed_container_model,
                    weighed_container_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateWeighedContainerModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.weighed_container_model = Some(weighed_container_model);
        Ok(self)
    }
    ///Sets the value of the `public.weighing_procedure_templates.procedure_template_weighed_container_model` column.
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
    ///subgraph v5 ["`weighing_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_weighed_container_model"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "weighed_container_model"}
    ///class v2 directly-involved-column
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn procedure_template_weighed_container_model<PTWCM>(
        mut self,
        procedure_template_weighed_container_model: PTWCM,
    ) -> Result<Self, Self::Error>
    where
        PTWCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_weighed_container_model = procedure_template_weighed_container_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_weighed_container_model {
            procedure_template_weighed_container_model = if let (
                Some(weighed_container_model),
                Some(asset_model),
            ) = (self.weighed_container_model, builder.asset_model) {
                if weighed_container_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::WeighedContainerModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.weighed_container_model = Some(asset_model);
                builder.into()
            } else if let Some(weighed_container_model) = self.weighed_container_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        weighed_container_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateWeighedContainerModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_weighed_container_model = procedure_template_weighed_container_model;
        Ok(self)
    }
    ///Sets the value of the `public.weighing_procedure_templates.weighed_with_model` column.
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
    ///subgraph v5 ["`weighing_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_weighed_with_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "weighed_with_model"}
    ///class v2 column-of-interest
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn weighed_with_model<WWM>(
        mut self,
        weighed_with_model: WWM,
    ) -> Result<Self, Self::Error>
    where
        WWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let weighed_with_model = <WWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &weighed_with_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_weighed_with_model,
        ) = self.procedure_template_weighed_with_model
        {
            self.procedure_template_weighed_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_weighed_with_model,
                    weighed_with_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateWeighedWithModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.weighed_with_model = Some(weighed_with_model);
        Ok(self)
    }
    ///Sets the value of the `public.weighing_procedure_templates.procedure_template_weighed_with_model` column.
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
    ///subgraph v5 ["`weighing_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_weighed_with_model"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "weighed_with_model"}
    ///class v2 directly-involved-column
    ///end
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v0
    ///v5 ---o|"`associated with`"| v4
    ///```
    fn procedure_template_weighed_with_model<PTWWM>(
        mut self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, Self::Error>
    where
        PTWWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_weighed_with_model = procedure_template_weighed_with_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_weighed_with_model {
            procedure_template_weighed_with_model = if let (
                Some(weighed_with_model),
                Some(asset_model),
            ) = (self.weighed_with_model, builder.asset_model) {
                if weighed_with_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::WeighedWithModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.weighed_with_model = Some(asset_model);
                builder.into()
            } else if let Some(weighed_with_model) = self.weighed_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        weighed_with_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateWeighedWithModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_weighed_with_model = procedure_template_weighed_with_model;
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
for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::WeighingProcedureTemplateAttribute,
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
    for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableWeighingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
            Error = web_common_traits::database::InsertError<
                WeighingProcedureTemplateAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        WeighingProcedureTemplateAttribute,
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
