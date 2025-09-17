#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GeolocationProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for GeolocationProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => {
                write!(f, "geolocation_procedure_templates({e})")
            }
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for GeolocationProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for GeolocationProcedureTemplateExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GeolocationProcedureTemplateAttribute {
    Extension(GeolocationProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    GeolocatedWithModel,
    ProcedureTemplateGeolocatedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    GeolocatedAssetModel,
    ProcedureTemplateGeolocatedAssetModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for GeolocationProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GeolocatedWithModel" => Ok(Self::GeolocatedWithModel),
            "ProcedureTemplateGeolocatedWithModel" => {
                Ok(
                    Self::ProcedureTemplateGeolocatedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "GeolocatedAssetModel" => Ok(Self::GeolocatedAssetModel),
            "ProcedureTemplateGeolocatedAssetModel" => {
                Ok(
                    Self::ProcedureTemplateGeolocatedAssetModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "geolocated_with_model" => Ok(Self::GeolocatedWithModel),
            "procedure_template_geolocated_with_model" => {
                Ok(
                    Self::ProcedureTemplateGeolocatedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "geolocated_asset_model" => Ok(Self::GeolocatedAssetModel),
            "procedure_template_geolocated_asset_model" => {
                Ok(
                    Self::ProcedureTemplateGeolocatedAssetModel(
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
for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder<
    T1,
> {
    type Attribute = GeolocationProcedureTemplateAttribute;
}
impl core::fmt::Display for GeolocationProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "geolocation_procedure_templates.procedure_template")
            }
            Self::GeolocatedWithModel => {
                write!(f, "geolocation_procedure_templates.geolocated_with_model")
            }
            Self::ProcedureTemplateGeolocatedWithModel(e) => {
                write!(f, "geolocation_procedure_templates.{e}")
            }
            Self::GeolocatedAssetModel => {
                write!(f, "geolocation_procedure_templates.geolocated_asset_model")
            }
            Self::ProcedureTemplateGeolocatedAssetModel(e) => {
                write!(f, "geolocation_procedure_templates.{e}")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableGeolocationProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) geolocated_with_model: i32,
    pub(crate) procedure_template_geolocated_with_model: i32,
    pub(crate) geolocated_asset_model: i32,
    pub(crate) procedure_template_geolocated_asset_model: i32,
}
impl InsertableGeolocationProcedureTemplate {
    pub fn procedure_template_geolocated_asset_model<
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
            self.procedure_template_geolocated_asset_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedure_templa_procedure_template_geolocate_fkey2(
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
                    .eq(&self.procedure_template_geolocated_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.geolocated_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedure_templa_procedure_template_geolocate_fkey3(
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
                    .eq(&self.procedure_template_geolocated_asset_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.geolocated_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_geolocated_with_model<
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
            self.procedure_template_geolocated_with_model,
            conn,
        )
    }
    pub fn geolocated_asset_model<C: diesel::connection::LoadConnection>(
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
            self.geolocated_asset_model,
            conn,
        )
    }
    pub fn geolocated_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::read(
            self.geolocated_with_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`GeolocationProcedureTemplate`](crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::GeolocationProcedureTemplate;
/// use core_structures::tables::insertables::GeolocationProcedureTemplateSettable;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let geolocation_procedure_template = GeolocationProcedureTemplate::new()
///    // Set mandatory fields
///    .procedure_template_geolocated_asset_model(procedure_template_geolocated_asset_model)?
///    .procedure_template_geolocated_with_model(procedure_template_geolocated_with_model)?
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableGeolocationProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) geolocated_with_model: Option<i32>,
    pub(crate) procedure_template_geolocated_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) geolocated_asset_model: Option<i32>,
    pub(crate) procedure_template_geolocated_asset_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> diesel::associations::HasTable
    for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
{
    type Table = crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::table
    }
}
impl From<InsertableGeolocationProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableGeolocationProcedureTemplateBuilder>
{
    fn from(builder: InsertableGeolocationProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    ProcedureTemplate: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete()
            && (self.geolocated_with_model.is_some()
                || self.procedure_template_geolocated_with_model.is_complete())
            && self.procedure_template_geolocated_with_model.is_complete()
            && (self.geolocated_asset_model.is_some()
                || self.procedure_template_geolocated_asset_model.is_complete())
            && self.procedure_template_geolocated_asset_model.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `GeolocationProcedureTemplate` or descendant tables.
pub trait GeolocationProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.geolocation_procedure_templates.geolocated_with_model` column.
    ///
    /// # Arguments
    /// * `geolocated_with_model`: The value to set for the
    ///   `public.geolocation_procedure_templates.geolocated_with_model` column.
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
    fn geolocated_with_model<GWM>(self, geolocated_with_model: GWM) -> Result<Self, Self::Error>
    where
        GWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.geolocation_procedure_templates.
    /// procedure_template_geolocated_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_geolocated_with_model`: The value to set for the
    ///   `public.geolocation_procedure_templates.
    ///   procedure_template_geolocated_with_model` column.
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
    fn procedure_template_geolocated_with_model<PTGWM>(
        self,
        procedure_template_geolocated_with_model: PTGWM,
    ) -> Result<Self, Self::Error>
    where
        PTGWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.geolocation_procedure_templates.geolocated_asset_model` column.
    ///
    /// # Arguments
    /// * `geolocated_asset_model`: The value to set for the
    ///   `public.geolocation_procedure_templates.geolocated_asset_model`
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
    fn geolocated_asset_model<GAM>(self, geolocated_asset_model: GAM) -> Result<Self, Self::Error>
    where
        GAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.geolocation_procedure_templates.
    /// procedure_template_geolocated_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_geolocated_asset_model`: The value to set for the
    ///   `public.geolocation_procedure_templates.
    ///   procedure_template_geolocated_asset_model` column.
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
    fn procedure_template_geolocated_asset_model<PTGAM>(
        self,
        procedure_template_geolocated_asset_model: PTGAM,
    ) -> Result<Self, Self::Error>
    where
        PTGAM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> GeolocationProcedureTemplateSettable
for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.geolocation_procedure_templates.geolocated_with_model` column.
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
    ///subgraph v4 ["`geolocation_procedure_templates`"]
    ///    v0@{shape: rounded, label: "geolocated_with_model"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_with_model"}
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
    fn geolocated_with_model<GWM>(
        mut self,
        geolocated_with_model: GWM,
    ) -> Result<Self, Self::Error>
    where
        GWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let geolocated_with_model = <GWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &geolocated_with_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_geolocated_with_model,
        ) = self.procedure_template_geolocated_with_model
        {
            self.procedure_template_geolocated_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_geolocated_with_model,
                    geolocated_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedWithModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.geolocated_with_model = Some(geolocated_with_model);
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedure_templates.procedure_template_geolocated_with_model` column.
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
    ///subgraph v4 ["`geolocation_procedure_templates`"]
    ///    v0@{shape: rounded, label: "geolocated_with_model"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_with_model"}
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
    fn procedure_template_geolocated_with_model<PTGWM>(
        mut self,
        procedure_template_geolocated_with_model: PTGWM,
    ) -> Result<Self, Self::Error>
    where
        PTGWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_geolocated_with_model = procedure_template_geolocated_with_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_geolocated_with_model {
            procedure_template_geolocated_with_model = if let (
                Some(geolocated_with_model),
                Some(asset_model),
            ) = (self.geolocated_with_model, builder.asset_model) {
                if geolocated_with_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::GeolocatedWithModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.geolocated_with_model = Some(asset_model);
                builder.into()
            } else if let Some(geolocated_with_model) = self.geolocated_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        geolocated_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedWithModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_geolocated_with_model = procedure_template_geolocated_with_model;
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedure_templates.geolocated_asset_model` column.
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
    ///subgraph v4 ["`geolocation_procedure_templates`"]
    ///    v0@{shape: rounded, label: "geolocated_asset_model"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
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
    fn geolocated_asset_model<GAM>(
        mut self,
        geolocated_asset_model: GAM,
    ) -> Result<Self, Self::Error>
    where
        GAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let geolocated_asset_model = <GAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &geolocated_asset_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_geolocated_asset_model,
        ) = self.procedure_template_geolocated_asset_model
        {
            self.procedure_template_geolocated_asset_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_geolocated_asset_model,
                    geolocated_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedAssetModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.geolocated_asset_model = Some(geolocated_asset_model);
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedure_templates.procedure_template_geolocated_asset_model` column.
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
    ///subgraph v4 ["`geolocation_procedure_templates`"]
    ///    v0@{shape: rounded, label: "geolocated_asset_model"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
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
    fn procedure_template_geolocated_asset_model<PTGAM>(
        mut self,
        procedure_template_geolocated_asset_model: PTGAM,
    ) -> Result<Self, Self::Error>
    where
        PTGAM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_geolocated_asset_model = procedure_template_geolocated_asset_model
            .into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_template_geolocated_asset_model {
            procedure_template_geolocated_asset_model = if let (
                Some(geolocated_asset_model),
                Some(asset_model),
            ) = (self.geolocated_asset_model, builder.asset_model) {
                if geolocated_asset_model != asset_model {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::GeolocatedAssetModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.geolocated_asset_model = Some(asset_model);
                builder.into()
            } else if let Some(geolocated_asset_model) = self.geolocated_asset_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        geolocated_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedAssetModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_geolocated_asset_model = procedure_template_geolocated_asset_model;
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
for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
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
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                self.procedure_template,
                name,
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
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                self.procedure_template,
                deprecated,
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
impl<ProcedureTemplate> web_common_traits::database::MostConcreteTable
    for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableGeolocationProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        C,
        Row = crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            GeolocationProcedureTemplateAttribute,
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
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<GeolocationProcedureTemplateAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
