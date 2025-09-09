#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GeolocationProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for GeolocationProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
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
}
impl core::str::FromStr for GeolocationProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for GeolocationProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::GeolocatedAsset => write!(f, "geolocated_asset"),
            Self::ProcedureTemplateGeolocatedAssetModel => {
                write!(f, "procedure_template_geolocated_asset_model")
            }
            Self::ProcedureGeolocatedAsset(e) => write!(f, "{e}"),
            Self::GeolocatedWith => write!(f, "geolocated_with"),
            Self::ProcedureGeolocatedWith(e) => write!(f, "{e}"),
            Self::ProcedureTemplateGeolocatedWithModel => {
                write!(f, "procedure_template_geolocated_with_model")
            }
        }
    }
}
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
}
impl InsertableGeolocationProcedure {
    pub fn geolocated_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAsset, diesel::result::Error>
    where
        crate::PhysicalAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAsset::read(self.geolocated_asset, conn)
    }
    pub fn geolocated_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::PositioningDevice>, diesel::result::Error>
    where
        crate::PositioningDevice: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(geolocated_with) = self.geolocated_with else {
            return Ok(None);
        };
        crate::PositioningDevice::read(geolocated_with, conn).map(Some)
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Procedure, diesel::result::Error>
    where
        crate::Procedure: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_geolocated_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_geolocated_asset, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_asset_geolocat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_geolocated_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.geolocated_asset),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_asset_procedur_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_geolocated_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_geolocated_asset_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_geolocated_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_geolocated_with, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_with_geolocate_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureAsset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(geolocated_with) = self.geolocated_with else {
            return Ok(None);
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_geolocated_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(geolocated_with),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_geolocated_with_procedure_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_geolocated_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_geolocated_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::Procedure, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<crate::Procedure>(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::GeolocationProcedureTemplate, diesel::result::Error>
    where
        crate::GeolocationProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::GeolocationProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn procedure_template_geolocated_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_geolocated_asset_model,
            conn,
        )
    }
    pub fn procedure_template_geolocated_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_geolocated_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_template_procedure_templ_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::GeolocationProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::GeolocationProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template_geolocated_asset_model
                            .eq(&self.procedure_template_geolocated_asset_model),
                    ),
            )
            .first::<crate::GeolocationProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn geolocation_procedures_procedure_template_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::GeolocationProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::GeolocationProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::geolocation_procedure_templates::geolocation_procedure_templates::dsl::procedure_template_geolocated_with_model
                            .eq(&self.procedure_template_geolocated_with_model),
                    ),
            )
            .first::<crate::GeolocationProcedureTemplate>(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub(crate) procedure: Procedure,
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
    }
}
/// Trait defining setters for attributes of an instance of
/// `GeolocationProcedure` or descendant tables.
pub trait GeolocationProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
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
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn geolocated_asset(
        self,
        geolocated_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure_template_geolocated_asset_model(
        self,
        procedure_template_geolocated_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn geolocated_with(
        self,
        geolocated_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn procedure_template_geolocated_with_model(
        self,
        procedure_template_geolocated_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> GeolocationProcedureSettable for InsertableGeolocationProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute;
    /// Sets the value of the `public.geolocation_procedures.procedure_template`
    /// column.
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
    /// subgraph v5 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_with_model"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 undirectly-involved-column
    /// end
    /// subgraph v7 ["`procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template"}
    /// class v3 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v5 --->|"`extends`"| v7
    /// v5 ---o|"`associated with`"| v6
    /// ```
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                self.procedure,
                procedure_template,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.geolocation_procedures.geolocated_asset`
    /// column.
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
    /// subgraph v4 ["`geolocation_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_geolocated_asset"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "geolocated_asset"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn geolocated_asset(
        mut self,
        geolocated_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_geolocated_asset) =
            self.procedure_geolocated_asset
        {
            self.procedure_geolocated_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_geolocated_asset,
                    Some(geolocated_asset),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureGeolocatedAsset(attribute)
                    })
                })?
                .into();
        }
        self.geolocated_asset = Some(geolocated_asset);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.geolocation_procedures.
    /// procedure_template_geolocated_asset_model` column.
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
    /// subgraph v4 ["`geolocation_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "procedure_geolocated_asset"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_geolocated_asset_model(
        mut self,
        procedure_template_geolocated_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_geolocated_asset) =
            self.procedure_geolocated_asset
        {
            self.procedure_geolocated_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_geolocated_asset,
                    procedure_template_geolocated_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureGeolocatedAsset(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_geolocated_asset_model =
            Some(procedure_template_geolocated_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_geolocated_asset` column.
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
    /// subgraph v6 ["`geolocation_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_asset_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "geolocated_asset"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_geolocated_asset"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_geolocated_asset<PGA>(
        mut self,
        procedure_geolocated_asset: PGA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PGA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_geolocated_asset = procedure_geolocated_asset.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_geolocated_asset
        {
            procedure_geolocated_asset = if let (Some(geolocated_asset), Some(asset)) =
                (self.geolocated_asset, builder.asset)
            {
                if geolocated_asset != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::GeolocatedAsset,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.geolocated_asset = Some(asset);
                builder.into()
            } else if let Some(geolocated_asset) = self.geolocated_asset {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(geolocated_asset),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureGeolocatedAsset(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_geolocated_asset
        {
            procedure_geolocated_asset = if let (
                Some(procedure_template_geolocated_asset_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_geolocated_asset_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_geolocated_asset_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateGeolocatedAssetModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_geolocated_asset_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_geolocated_asset_model) =
                self.procedure_template_geolocated_asset_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_geolocated_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureGeolocatedAsset(attribute)
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
    /// Sets the value of the `public.geolocation_procedures.geolocated_with`
    /// column.
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
    /// subgraph v4 ["`geolocation_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_geolocated_with"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "geolocated_with"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn geolocated_with(
        mut self,
        geolocated_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_geolocated_with) =
            self.procedure_geolocated_with
        {
            self.procedure_geolocated_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_geolocated_with,
                    geolocated_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureGeolocatedWith(attribute)
                    })
                })?
                .into();
        }
        self.geolocated_with = geolocated_with;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_geolocated_with` column.
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
    /// subgraph v6 ["`geolocation_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_geolocated_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "geolocated_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_geolocated_with"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_geolocated_with<PGW>(
        mut self,
        procedure_geolocated_with: PGW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PGW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_geolocated_with = procedure_geolocated_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_geolocated_with
        {
            procedure_geolocated_with = if let (Some(geolocated_with), Some(asset)) =
                (self.geolocated_with, builder.asset)
            {
                if geolocated_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::GeolocatedWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.geolocated_with = Some(asset);
                builder.into()
            } else if let Some(geolocated_with) = self.geolocated_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(geolocated_with),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureGeolocatedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_geolocated_with
        {
            procedure_geolocated_with = if let (
                Some(procedure_template_geolocated_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_geolocated_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_geolocated_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateGeolocatedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_geolocated_with_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_geolocated_with_model) =
                self.procedure_template_geolocated_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_geolocated_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureGeolocatedWith(attribute)
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
    /// Sets the value of the
    /// `public.geolocation_procedures.procedure_template_geolocated_with_model`
    /// column.
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
    /// subgraph v4 ["`geolocation_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_geolocated_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_geolocated_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_geolocated_with_model(
        mut self,
        procedure_template_geolocated_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_geolocated_with) =
            self.procedure_geolocated_with
        {
            self.procedure_geolocated_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_geolocated_with,
                    procedure_template_geolocated_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureGeolocatedWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_geolocated_with_model =
            Some(procedure_template_geolocated_with_model);
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableGeolocationProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure(
        mut self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                self.procedure,
                procedure,
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
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as GeolocationProcedureSettable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure(
        mut self,
        parent_procedure: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                self.procedure,
                parent_procedure,
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
    ///Sets the value of the `public.procedures.parent_procedure_template` column.
    fn parent_procedure_template(
        mut self,
        parent_procedure_template: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                self.procedure,
                parent_procedure_template,
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
    ///Sets the value of the `public.procedures.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                self.procedure,
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
    ///Sets the value of the `public.procedures.created_at` column.
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                self.procedure,
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
    ///Sets the value of the `public.procedures.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                self.procedure,
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
    ///Sets the value of the `public.procedures.updated_at` column.
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                self.procedure,
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
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::GeolocationProcedure,
            Error = web_common_traits::database::InsertError<GeolocationProcedureAttribute>,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        web_common_traits::database::TryInsertGeneric<C>,
{
    type Attributes = GeolocationProcedureAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::GeolocationProcedure = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
