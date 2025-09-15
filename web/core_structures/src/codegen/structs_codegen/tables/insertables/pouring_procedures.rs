#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PouringProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for PouringProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for PouringProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for PouringProcedureExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PouringProcedureAttribute {
    Extension(PouringProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    PouredFrom,
    ProcedureTemplatePouredFromModel,
    ProcedurePouredFrom(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    MeasuredWith,
    ProcedureTemplateMeasuredWithModel,
    ProcedureMeasuredWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    PouredInto,
    ProcedureTemplatePouredIntoModel,
    ProcedurePouredInto(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for PouringProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "PouredFrom" => Ok(Self::PouredFrom),
            "ProcedureTemplatePouredFromModel" => Ok(Self::ProcedureTemplatePouredFromModel),
            "ProcedurePouredFrom" => Ok(Self::ProcedurePouredFrom(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "MeasuredWith" => Ok(Self::MeasuredWith),
            "ProcedureTemplateMeasuredWithModel" => Ok(Self::ProcedureTemplateMeasuredWithModel),
            "ProcedureMeasuredWith" => Ok(Self::ProcedureMeasuredWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "PouredInto" => Ok(Self::PouredInto),
            "ProcedureTemplatePouredIntoModel" => Ok(Self::ProcedureTemplatePouredIntoModel),
            "ProcedurePouredInto" => Ok(Self::ProcedurePouredInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "poured_from" => Ok(Self::PouredFrom),
            "procedure_template_poured_from_model" => Ok(Self::ProcedureTemplatePouredFromModel),
            "procedure_poured_from" => Ok(Self::ProcedurePouredFrom(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "measured_with" => Ok(Self::MeasuredWith),
            "procedure_template_measured_with_model" => {
                Ok(Self::ProcedureTemplateMeasuredWithModel)
            }
            "procedure_measured_with" => Ok(Self::ProcedureMeasuredWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "poured_into" => Ok(Self::PouredInto),
            "procedure_template_poured_into_model" => Ok(Self::ProcedureTemplatePouredIntoModel),
            "procedure_poured_into" => Ok(Self::ProcedurePouredInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<T1>
{
    type Attribute = PouringProcedureAttribute;
}
impl core::fmt::Display for PouringProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "pouring_procedures.procedure"),
            Self::ProcedureTemplate => write!(f, "pouring_procedures.procedure_template"),
            Self::PouredFrom => write!(f, "pouring_procedures.poured_from"),
            Self::ProcedureTemplatePouredFromModel => {
                write!(f, "pouring_procedures.procedure_template_poured_from_model")
            }
            Self::ProcedurePouredFrom(e) => write!(f, "pouring_procedures.{e}"),
            Self::MeasuredWith => write!(f, "pouring_procedures.measured_with"),
            Self::ProcedureTemplateMeasuredWithModel => {
                write!(f, "pouring_procedures.procedure_template_measured_with_model")
            }
            Self::ProcedureMeasuredWith(e) => write!(f, "pouring_procedures.{e}"),
            Self::PouredInto => write!(f, "pouring_procedures.poured_into"),
            Self::ProcedureTemplatePouredIntoModel => {
                write!(f, "pouring_procedures.procedure_template_poured_into_model")
            }
            Self::ProcedurePouredInto(e) => write!(f, "pouring_procedures.{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::pouring_procedures::pouring_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePouringProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) poured_from: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_poured_from_model: i32,
    pub(crate) procedure_poured_from: ::rosetta_uuid::Uuid,
    pub(crate) measured_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_measured_with_model: i32,
    pub(crate) procedure_measured_with: ::rosetta_uuid::Uuid,
    pub(crate) poured_into: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_poured_into_model: i32,
    pub(crate) procedure_poured_into: ::rosetta_uuid::Uuid,
}
impl InsertablePouringProcedure {
    pub fn measured_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(measured_with) = self.measured_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice::read(
                measured_with,
                conn,
            )
            .optional()
    }
    pub fn poured_from<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.poured_from,
            conn,
        )
    }
    pub fn poured_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.poured_into,
            conn,
        )
    }
    pub fn procedure_measured_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_measured_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_measured_with_measured_with_fkey(
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
        let Some(measured_with) = self.measured_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_measured_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(measured_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_measured_with_procedure_templ_fkey(
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
                    .eq(&self.procedure_measured_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_measured_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_poured_from<C: diesel::connection::LoadConnection>(
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
            self.procedure_poured_from,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_poured_from_poured_from_fkey(
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
                    .eq(&self.procedure_poured_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.poured_from),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_poured_from_procedure_templat_fkey(
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
                    .eq(&self.procedure_poured_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_poured_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_poured_into<C: diesel::connection::LoadConnection>(
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
            self.procedure_poured_into,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_poured_into_poured_into_fkey(
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
                    .eq(&self.procedure_poured_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.poured_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_poured_into_procedure_templat_fkey(
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
                    .eq(&self.procedure_poured_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_poured_into_model),
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
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_measured_with_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_measured_with_model,
            conn,
        )
    }
    pub fn procedure_template_poured_from_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_poured_from_model,
            conn,
        )
    }
    pub fn procedure_template_poured_into_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_poured_into_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template_poured_into_model
                            .eq(&self.procedure_template_poured_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_template_procedure_template_m_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template_measured_with_model
                            .eq(&self.procedure_template_measured_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedures_procedure_template_procedure_template_p_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::dsl::procedure_template_poured_from_model
                            .eq(&self.procedure_template_poured_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
            >(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`PouringProcedure`](crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::PouringProcedure;
/// use core_structures::tables::insertables::PouringProcedureSettable;
/// use core_structures::tables::insertables::ProcedureSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let pouring_procedure = PouringProcedure::new()
///    // Set mandatory fields
///    .procedure_measured_with(procedure_measured_with)?
///    .procedure_poured_from(procedure_poured_from)?
///    .procedure_poured_into(procedure_poured_into)?
///    .procedure_template(procedure_template)?
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
pub struct InsertablePouringProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) poured_from: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_poured_from_model: Option<i32>,
    pub(crate) procedure_poured_from: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) measured_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_measured_with_model: Option<i32>,
    pub(crate) procedure_measured_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) poured_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_poured_into_model: Option<i32>,
    pub(crate) procedure_poured_into: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertablePouringProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertablePouringProcedureBuilder,
    >
{
    fn from(builder: InsertablePouringProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
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
            && (self.poured_from.is_some() || self.procedure_poured_from.is_complete())
            && (self.procedure_template_poured_from_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_poured_from.is_complete())
            && self.procedure_poured_from.is_complete()
            && (self.procedure_template_measured_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_measured_with.is_complete())
            && self.procedure_measured_with.is_complete()
            && (self.poured_into.is_some() || self.procedure_poured_into.is_complete())
            && (self.procedure_template_poured_into_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_poured_into.is_complete())
            && self.procedure_poured_into.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `PouringProcedure`
/// or descendant tables.
pub trait PouringProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.pouring_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.pouring_procedures.procedure_template` column.
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
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.pouring_procedures.poured_from` column.
    ///
    /// # Arguments
    /// * `poured_from`: The value to set for the
    ///   `public.pouring_procedures.poured_from` column.
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
    fn poured_from<PF>(
        self,
        poured_from: PF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PF: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_poured_from_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_poured_from_model`: The value to set for the
    ///   `public.pouring_procedures.procedure_template_poured_from_model`
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
    fn procedure_template_poured_from_model<PTPFM>(
        self,
        procedure_template_poured_from_model: PTPFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.pouring_procedures.procedure_poured_from`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_poured_from`: The value to set for the
    ///   `public.pouring_procedures.procedure_poured_from` column.
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
    fn procedure_poured_from<PPF>(
        self,
        procedure_poured_from: PPF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPF: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.pouring_procedures.measured_with` column.
    ///
    /// # Arguments
    /// * `measured_with`: The value to set for the
    ///   `public.pouring_procedures.measured_with` column.
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
    fn measured_with<MW>(
        self,
        measured_with: MW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_measured_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_measured_with_model`: The value to set for the
    ///   `public.pouring_procedures.procedure_template_measured_with_model`
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
    fn procedure_template_measured_with_model<PTMWM>(
        self,
        procedure_template_measured_with_model: PTMWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_measured_with` column.
    ///
    /// # Arguments
    /// * `procedure_measured_with`: The value to set for the
    ///   `public.pouring_procedures.procedure_measured_with` column.
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
    fn procedure_measured_with<PMW>(
        self,
        procedure_measured_with: PMW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.pouring_procedures.poured_into` column.
    ///
    /// # Arguments
    /// * `poured_into`: The value to set for the
    ///   `public.pouring_procedures.poured_into` column.
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
    fn poured_into<PI>(
        self,
        poured_into: PI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_poured_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_poured_into_model`: The value to set for the
    ///   `public.pouring_procedures.procedure_template_poured_into_model`
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
    fn procedure_template_poured_into_model<PTPIM>(
        self,
        procedure_template_poured_into_model: PTPIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.pouring_procedures.procedure_poured_into`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_poured_into`: The value to set for the
    ///   `public.pouring_procedures.procedure_poured_into` column.
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
    fn procedure_poured_into<PPI>(
        self,
        procedure_poured_into: PPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> PouringProcedureSettable for InsertablePouringProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute;
    /// Sets the value of the `public.pouring_procedures.procedure_template`
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
    /// subgraph v6 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_measured_with_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_poured_from_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_poured_into_model"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v5 undirectly-involved-column
    /// end
    /// subgraph v8 ["`procedures`"]
    ///    v4@{shape: rounded, label: "procedure_template"}
    /// class v4 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v0 -.->|"`foreign defines`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v6 --->|"`extends`"| v8
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template =
            <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure_template);
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
    /// Sets the value of the `public.pouring_procedures.poured_from` column.
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "poured_from"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_poured_from"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn poured_from<PF>(
        mut self,
        poured_from: PF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PF: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let poured_from =
            <PF as web_common_traits::database::PrimaryKeyLike>::primary_key(&poured_from);
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_poured_from) =
            self.procedure_poured_from
        {
            self.procedure_poured_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_poured_from,
                    poured_from,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePouredFrom(attribute)
                    })
                })?
                .into();
        }
        self.poured_from = Some(poured_from);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_poured_from_model` column.
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_poured_from"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_poured_from_model"}
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
    fn procedure_template_poured_from_model<PTPFM>(
        mut self,
        procedure_template_poured_from_model: PTPFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_poured_from_model =
            <PTPFM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_poured_from_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_poured_from) =
            self.procedure_poured_from
        {
            self.procedure_poured_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_poured_from,
                    procedure_template_poured_from_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePouredFrom(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_poured_from_model = Some(procedure_template_poured_from_model);
        Ok(self)
    }
    /// Sets the value of the `public.pouring_procedures.procedure_poured_from`
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
    /// subgraph v6 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "poured_from"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_poured_from"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_poured_from_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_poured_from<PPF>(
        mut self,
        procedure_poured_from: PPF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPF: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_poured_from = procedure_poured_from.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_poured_from {
            procedure_poured_from = if let (Some(poured_from), Some(asset)) =
                (self.poured_from, builder.asset)
            {
                if poured_from != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PouredFrom,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.poured_from = Some(asset);
                builder.into()
            } else if let Some(poured_from) = self.poured_from {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        poured_from,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePouredFrom(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_poured_from {
            procedure_poured_from = if let (
                Some(procedure_template_poured_from_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_poured_from_model, builder.procedure_template_asset_model)
            {
                if procedure_template_poured_from_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplatePouredFromModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_poured_from_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_poured_from_model) =
                self.procedure_template_poured_from_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_poured_from_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePouredFrom(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_poured_from = procedure_poured_from;
        Ok(self)
    }
    /// Sets the value of the `public.pouring_procedures.measured_with` column.
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "measured_with"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_measured_with"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn measured_with<MW>(
        mut self,
        measured_with: MW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let measured_with =
            <MW as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &measured_with,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_measured_with) =
            self.procedure_measured_with
        {
            self.procedure_measured_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_measured_with,
                    measured_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMeasuredWith(attribute)
                    })
                })?
                .into();
        }
        self.measured_with = measured_with;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_measured_with_model`
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_measured_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_measured_with_model"}
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
    fn procedure_template_measured_with_model<PTMWM>(
        mut self,
        procedure_template_measured_with_model: PTMWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_measured_with_model =
            <PTMWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_measured_with_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_measured_with) =
            self.procedure_measured_with
        {
            self.procedure_measured_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_measured_with,
                    procedure_template_measured_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMeasuredWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_measured_with_model = Some(procedure_template_measured_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_measured_with` column.
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
    /// subgraph v6 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "measured_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_measured_with"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_measured_with_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_measured_with<PMW>(
        mut self,
        procedure_measured_with: PMW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_measured_with = procedure_measured_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_measured_with
        {
            procedure_measured_with = if let (Some(measured_with), Some(asset)) =
                (self.measured_with, builder.asset)
            {
                if measured_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MeasuredWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.measured_with = Some(asset);
                builder.into()
            } else if let Some(measured_with) = self.measured_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        measured_with,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMeasuredWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_measured_with
        {
            procedure_measured_with = if let (
                Some(procedure_template_measured_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_measured_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_measured_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateMeasuredWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_measured_with_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_measured_with_model) =
                self.procedure_template_measured_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_measured_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMeasuredWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_measured_with = procedure_measured_with;
        Ok(self)
    }
    /// Sets the value of the `public.pouring_procedures.poured_into` column.
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "poured_into"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_poured_into"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn poured_into<PI>(
        mut self,
        poured_into: PI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let poured_into =
            <PI as web_common_traits::database::PrimaryKeyLike>::primary_key(&poured_into);
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_poured_into) =
            self.procedure_poured_into
        {
            self.procedure_poured_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_poured_into,
                    poured_into,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePouredInto(attribute)
                    })
                })?
                .into();
        }
        self.poured_into = Some(poured_into);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.pouring_procedures.procedure_template_poured_into_model` column.
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
    /// subgraph v4 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_poured_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_poured_into_model"}
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
    fn procedure_template_poured_into_model<PTPIM>(
        mut self,
        procedure_template_poured_into_model: PTPIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_poured_into_model =
            <PTPIM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_poured_into_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_poured_into) =
            self.procedure_poured_into
        {
            self.procedure_poured_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_poured_into,
                    procedure_template_poured_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePouredInto(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_poured_into_model = Some(procedure_template_poured_into_model);
        Ok(self)
    }
    /// Sets the value of the `public.pouring_procedures.procedure_poured_into`
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
    /// subgraph v6 ["`pouring_procedures`"]
    ///    v0@{shape: rounded, label: "poured_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_poured_into"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_poured_into_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_poured_into<PPI>(
        mut self,
        procedure_poured_into: PPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_poured_into = procedure_poured_into.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_poured_into {
            procedure_poured_into = if let (Some(poured_into), Some(asset)) =
                (self.poured_into, builder.asset)
            {
                if poured_into != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PouredInto,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.poured_into = Some(asset);
                builder.into()
            } else if let Some(poured_into) = self.poured_into {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        poured_into,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePouredInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_poured_into {
            procedure_poured_into = if let (
                Some(procedure_template_poured_into_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_poured_into_model, builder.procedure_template_asset_model)
            {
                if procedure_template_poured_into_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplatePouredIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_poured_into_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_poured_into_model) =
                self.procedure_template_poured_into_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_poured_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePouredInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_poured_into = procedure_poured_into;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertablePouringProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(
        mut self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    ///subgraph v2 ["`pouring_procedures`"]
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
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as PouringProcedureSettable>::procedure_template(self, procedure_template)
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure<PP>(
        mut self,
        parent_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
                    .into_field_name(|attribute| Self::Attributes::Extension(
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
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
    ///Sets the value of the `public.procedures.predecessor_procedure` column.
    fn predecessor_procedure<PP>(
        mut self,
        predecessor_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
                    .into_field_name(|attribute| Self::Attributes::Extension(
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                self.procedure,
                predecessor_procedure_template,
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    for InsertablePouringProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertablePouringProcedureBuilder<Procedure>
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
    for InsertablePouringProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
            Attribute = PouringProcedureAttribute,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        web_common_traits::database::TryInsertGeneric<C>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
