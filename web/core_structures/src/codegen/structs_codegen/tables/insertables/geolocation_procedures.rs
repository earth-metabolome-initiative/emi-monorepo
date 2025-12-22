#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GeolocationProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for GeolocationProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "geolocation_procedures({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for GeolocationProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for GeolocationProcedureExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GeolocationProcedureAttribute {
    Extension(GeolocationProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    GeolocatedAsset,
    ProcedureTemplateGeolocatedAssetModel,
    ProcedureGeolocatedAsset(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    GeolocatedWith,
    ProcedureGeolocatedWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    ProcedureTemplateGeolocatedWithModel,
    Location,
}
impl core::str::FromStr for GeolocationProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Procedure" => Ok(Self::Procedure),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "GeolocatedAsset" => Ok(Self::GeolocatedAsset),
            "ProcedureTemplateGeolocatedAssetModel" => {
                Ok(Self::ProcedureTemplateGeolocatedAssetModel)
            }
            "ProcedureGeolocatedAsset" => Ok(Self::ProcedureGeolocatedAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "GeolocatedWith" => Ok(Self::GeolocatedWith),
            "ProcedureGeolocatedWith" => Ok(Self::ProcedureGeolocatedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "ProcedureTemplateGeolocatedWithModel" => {
                Ok(Self::ProcedureTemplateGeolocatedWithModel)
            }
            "Location" => Ok(Self::Location),
            "procedure" => Ok(Self::Procedure),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "geolocated_asset" => Ok(Self::GeolocatedAsset),
            "procedure_template_geolocated_asset_model" => {
                Ok(Self::ProcedureTemplateGeolocatedAssetModel)
            }
            "procedure_geolocated_asset" => Ok(Self::ProcedureGeolocatedAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "geolocated_with" => Ok(Self::GeolocatedWith),
            "procedure_geolocated_with" => Ok(Self::ProcedureGeolocatedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template_geolocated_with_model" => {
                Ok(Self::ProcedureTemplateGeolocatedWithModel)
            }
            "location" => Ok(Self::Location),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
        T1,
    >
{
    type Attribute = GeolocationProcedureAttribute;
}
impl web_common_traits::database::TableField for GeolocationProcedureAttribute {}
impl web_common_traits::database::HasTableType for GeolocationProcedureAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    > for GeolocationProcedureAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        GeolocationProcedureAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for GeolocationProcedureAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        GeolocationProcedureAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for GeolocationProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "geolocation_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "geolocation_procedures.procedure_template")
            }
            Self::GeolocatedAsset => write!(f, "geolocation_procedures.geolocated_asset"),
            Self::ProcedureTemplateGeolocatedAssetModel => {
                write!(f, "geolocation_procedures.procedure_template_geolocated_asset_model")
            }
            Self::ProcedureGeolocatedAsset(e) => write!(f, "geolocation_procedures.{e}"),
            Self::GeolocatedWith => write!(f, "geolocation_procedures.geolocated_with"),
            Self::ProcedureGeolocatedWith(e) => write!(f, "geolocation_procedures.{e}"),
            Self::ProcedureTemplateGeolocatedWithModel => {
                write!(f, "geolocation_procedures.procedure_template_geolocated_with_model")
            }
            Self::Location => write!(f, "geolocation_procedures.location"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::geolocation_procedures::geolocation_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableGeolocationProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) geolocated_asset: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_geolocated_asset_model: i32,
    pub(crate) procedure_geolocated_asset: ::rosetta_uuid::Uuid,
    pub(crate) geolocated_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_geolocated_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_geolocated_with_model: i32,
    pub(crate) location: postgis_diesel::types::Point,
}
impl InsertableGeolocationProcedure {
    pub fn geolocated_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(
            self.geolocated_asset,
            conn,
        )
    }
    pub fn geolocated_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(geolocated_with) = self.geolocated_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice::read(
            geolocated_with,
            conn,
        )
        .optional()
    }
    pub fn procedure_geolocated_asset<C: diesel::connection::LoadConnection>(
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
            self.procedure_geolocated_asset,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_asset_geolocat_fkey(
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
                    .eq(&self.procedure_geolocated_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.geolocated_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_asset_procedur_fkey(
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
                    .eq(&self.procedure_geolocated_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_geolocated_asset_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_geolocated_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_geolocated_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_with_geolocate_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(geolocated_with) = self.geolocated_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_geolocated_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(geolocated_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_with_procedure_fkey(
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
                    .eq(&self.procedure_geolocated_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_geolocated_with_model),
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
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
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
            self.procedure_template_geolocated_asset_model_id,
            conn,
        )
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
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_template_procedure_templ_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template_geolocated_asset_model
                            .eq(&self.procedure_template_geolocated_asset_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_template_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template_geolocated_with_model
                            .eq(&self.procedure_template_geolocated_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
            >(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`GeolocationProcedure`](crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::GeolocationProcedure;
/// use core_structures::tables::insertables::GeolocationProcedureSettable;
/// use core_structures::tables::insertables::ProcedureSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let geolocation_procedure = GeolocationProcedure::new()
///    // Set mandatory fields
///    .location(location)?
///    .procedure_geolocated_asset(procedure_geolocated_asset)?
///    .procedure_geolocated_with(procedure_geolocated_with)?
///    .procedure_template(procedure_template_id)?
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
pub struct InsertableGeolocationProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) geolocated_asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_geolocated_asset_model: Option<i32>,
    pub(crate) procedure_geolocated_asset: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) geolocated_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_geolocated_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure_template_geolocated_with_model: Option<i32>,
    pub(crate) location: Option<postgis_diesel::types::Point>,
    pub(crate) procedure: Procedure,
}
impl<Procedure> diesel::associations::HasTable
    for InsertableGeolocationProcedureBuilder<Procedure>
{
    type Table = crate::codegen::diesel_codegen::tables::geolocation_procedures::geolocation_procedures::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::geolocation_procedures::geolocation_procedures::table
    }
}
impl From<InsertableGeolocationProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableGeolocationProcedureBuilder,
    >
{
    fn from(builder: InsertableGeolocationProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
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
            && (self.geolocated_asset.is_some() || self.procedure_geolocated_asset.is_complete())
            && (self.procedure_template_geolocated_asset_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_geolocated_asset.is_complete())
            && self.procedure_geolocated_asset.is_complete()
            && self.procedure_geolocated_with.is_complete()
            && (self.procedure_template_geolocated_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_geolocated_with.is_complete())
            && self.location.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `GeolocationProcedure` or descendant tables.
pub trait GeolocationProcedureSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.geolocation_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.geolocation_procedures.procedure_template` column.
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
    /// Sets the value of the `public.geolocation_procedures.geolocated_asset`
    /// column.
    ///
    /// # Arguments
    /// * `geolocated_asset`: The value to set for the
    ///   `public.geolocation_procedures.geolocated_asset` column.
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
    fn geolocated_asset<GA>(self, geolocated_asset: GA) -> Result<Self, Self::Error>
    where
        GA: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.geolocation_procedures.
    /// procedure_template_geolocated_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_geolocated_asset_model`: The value to set for the
    ///   `public.geolocation_procedures.
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
        PTGAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_geolocated_asset` column.
    ///
    /// # Arguments
    /// * `procedure_geolocated_asset`: The value to set for the
    ///   `public.geolocation_procedures.procedure_geolocated_asset` column.
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
    fn procedure_geolocated_asset<PGA>(
        self,
        procedure_geolocated_asset: PGA,
    ) -> Result<Self, Self::Error>
    where
        PGA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.geolocation_procedures.geolocated_with`
    /// column.
    ///
    /// # Arguments
    /// * `geolocated_with`: The value to set for the
    ///   `public.geolocation_procedures.geolocated_with` column.
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
    fn geolocated_with<GW>(self, geolocated_with: GW) -> Result<Self, Self::Error>
    where
        GW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_geolocated_with` column.
    ///
    /// # Arguments
    /// * `procedure_geolocated_with`: The value to set for the
    ///   `public.geolocation_procedures.procedure_geolocated_with` column.
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
    fn procedure_geolocated_with<PGW>(
        self,
        procedure_geolocated_with: PGW,
    ) -> Result<Self, Self::Error>
    where
        PGW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_template_geolocated_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_geolocated_with_model`: The value to set for the
    ///   `public.geolocation_procedures.
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
        PTGWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.geolocation_procedures.location` column.
    ///
    /// # Arguments
    /// * `location`: The value to set for the
    ///   `public.geolocation_procedures.location` column.
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
    ///   `postgis_diesel::types::Point`.
    /// * If the provided value does not pass schema-defined validation.
    fn location<L>(self, location: L) -> Result<Self, Self::Error>
    where
        L: TryInto<postgis_diesel::types::Point>,
        validation_errors::prelude::SingleFieldError:
            From<<L as TryInto<postgis_diesel::types::Point>>::Error>;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
            >,
        >,
> GeolocationProcedureSettable for InsertableGeolocationProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.geolocation_procedures.procedure_template` column.
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
    ///subgraph v5 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_with_model"}
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
                err.replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template_id);
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.geolocated_asset` column.
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
    ///subgraph v4 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "geolocated_asset"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_geolocated_asset"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn geolocated_asset<GA>(mut self, geolocated_asset: GA) -> Result<Self, Self::Error>
    where
        GA: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let geolocated_asset = <GA as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &geolocated_asset,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_geolocated_asset,
        ) = self.procedure_geolocated_asset
        {
            self.procedure_geolocated_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_geolocated_asset,
                    geolocated_asset,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedAsset(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.geolocated_asset = Some(geolocated_asset);
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.procedure_template_geolocated_asset_model` column.
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
    ///subgraph v4 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_geolocated_asset"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
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
    fn procedure_template_geolocated_asset_model<PTGAM>(
        mut self,
        procedure_template_geolocated_asset_model: PTGAM,
    ) -> Result<Self, Self::Error>
    where
        PTGAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_geolocated_asset_model_id = <PTGAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_geolocated_asset_model_id,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_geolocated_asset,
        ) = self.procedure_geolocated_asset
        {
            self.procedure_geolocated_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_geolocated_asset,
                    procedure_template_geolocated_asset_model_id,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedAsset(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_geolocated_asset_model_id = Some(
            procedure_template_geolocated_asset_model_id,
        );
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.procedure_geolocated_asset` column.
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
    ///subgraph v6 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "geolocated_asset"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_geolocated_asset"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
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
    ///v0 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v5
    ///v1 --->|"`associated same as`"| v5
    ///v1 --->|"`associated same as`"| v5
    ///v1 -.->|"`foreign defines`"| v0
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v4
    ///v6 ---o|"`associated with`"| v7
    ///```
    fn procedure_geolocated_asset<PGA>(
        mut self,
        procedure_geolocated_asset: PGA,
    ) -> Result<Self, Self::Error>
    where
        PGA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_geolocated_asset = procedure_geolocated_asset.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_geolocated_asset {
            procedure_geolocated_asset = if let (Some(geolocated_asset), Some(asset)) = (
                self.geolocated_asset,
                builder.asset,
            ) {
                if geolocated_asset != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::GeolocatedAsset,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.geolocated_asset = Some(asset);
                builder.into()
            } else if let Some(geolocated_asset) = self.geolocated_asset {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        geolocated_asset,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedAsset(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_geolocated_asset {
            procedure_geolocated_asset = if let (
                Some(procedure_template_geolocated_asset_model_id),
                Some(procedure_template_asset_model_id),
            ) = (
                self.procedure_template_geolocated_asset_model_id,
                builder.procedure_template_asset_model_id,
            ) {
                if procedure_template_geolocated_asset_model
                    != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedAssetModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model_id) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_geolocated_asset_model_id = Some(
                    procedure_template_asset_model_id,
                );
                builder.into()
            } else if let Some(procedure_template_geolocated_asset_model_id) = self
                .procedure_template_geolocated_asset_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_geolocated_asset_model_id,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedAsset(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_geolocated_asset = procedure_geolocated_asset;
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.geolocated_with` column.
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
    ///subgraph v4 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "geolocated_with"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_geolocated_with"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    ///class v3 undirectly-involved-column
    ///end
    ///v0 --->|"`associated same as`"| v2
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v3
    ///v1 -.->|"`foreign defines`"| v0
    ///v4 ---o|"`associated with`"| v5
    ///```
    fn geolocated_with<GW>(mut self, geolocated_with: GW) -> Result<Self, Self::Error>
    where
        GW: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let geolocated_with = <GW as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
            &geolocated_with,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_geolocated_with,
        ) = self.procedure_geolocated_with
        {
            self.procedure_geolocated_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_geolocated_with,
                    geolocated_with,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedWith(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.geolocated_with = geolocated_with;
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.procedure_geolocated_with` column.
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
    ///subgraph v6 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "geolocated_with"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_geolocated_with"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_with_model"}
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
    ///v0 --->|"`associated same as`"| v3
    ///v1 --->|"`associated same as`"| v5
    ///v1 --->|"`associated same as`"| v5
    ///v1 --->|"`associated same as`"| v5
    ///v1 -.->|"`foreign defines`"| v0
    ///v1 -.->|"`foreign defines`"| v2
    ///v2 --->|"`associated same as`"| v4
    ///v6 ---o|"`associated with`"| v7
    ///```
    fn procedure_geolocated_with<PGW>(
        mut self,
        procedure_geolocated_with: PGW,
    ) -> Result<Self, Self::Error>
    where
        PGW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_geolocated_with = procedure_geolocated_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_geolocated_with {
            procedure_geolocated_with = if let (Some(geolocated_with), Some(asset)) = (
                self.geolocated_with,
                builder.asset,
            ) {
                if geolocated_with != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::GeolocatedWith,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.geolocated_with = Some(asset);
                builder.into()
            } else if let Some(geolocated_with) = self.geolocated_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        geolocated_with,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedWith(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_geolocated_with {
            procedure_geolocated_with = if let (
                Some(procedure_template_geolocated_with_model),
                Some(procedure_template_asset_model_id),
            ) = (
                self.procedure_template_geolocated_with_model,
                builder.procedure_template_asset_model_id,
            ) {
                if procedure_template_geolocated_with_model
                    != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplateGeolocatedWithModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model_id) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_geolocated_with_model = Some(
                    procedure_template_asset_model_id,
                );
                builder.into()
            } else if let Some(procedure_template_geolocated_with_model) = self
                .procedure_template_geolocated_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_geolocated_with_model,
                    )
                    .map_err(|e| {
                        e.replace_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedWith(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_geolocated_with = procedure_geolocated_with;
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.procedure_template_geolocated_with_model` column.
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
    ///subgraph v4 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_geolocated_with"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_with_model"}
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
    fn procedure_template_geolocated_with_model<PTGWM>(
        mut self,
        procedure_template_geolocated_with_model: PTGWM,
    ) -> Result<Self, Self::Error>
    where
        PTGWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_geolocated_with_model = <PTGWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_geolocated_with_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_geolocated_with,
        ) = self.procedure_geolocated_with
        {
            self.procedure_geolocated_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_geolocated_with,
                    procedure_template_geolocated_with_model,
                )
                .map_err(|e| {
                    e.replace_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedureGeolocatedWith(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_geolocated_with_model = Some(
            procedure_template_geolocated_with_model,
        );
        Ok(self)
    }
    ///Sets the value of the `public.geolocation_procedures.location` column.
    fn location<L>(mut self, location: L) -> Result<Self, Self::Error>
    where
        L: TryInto<postgis_diesel::types::Point>,
        validation_errors::prelude::SingleFieldError: From<
            <L as TryInto<postgis_diesel::types::Point>>::Error,
        >,
    {
        let location = location
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(GeolocationProcedureAttribute::Location)
            })?;
        self.location = Some(location);
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
for InsertableGeolocationProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
    ///subgraph v2 ["`geolocation_procedures`"]
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
        <Self as GeolocationProcedureSettable>::procedure_template(
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                self.procedure,
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
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                self.procedure,
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
}
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertableGeolocationProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableGeolocationProcedureBuilder<Procedure>
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
for InsertableGeolocationProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
            Error = web_common_traits::database::InsertError<
                GeolocationProcedureAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<GeolocationProcedureAttribute>;
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
