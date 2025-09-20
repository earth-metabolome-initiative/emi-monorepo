#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhotographProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for PhotographProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "photograph_procedures({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for PhotographProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for PhotographProcedureExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhotographProcedureAttribute {
    Extension(PhotographProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    PhotographedAsset,
    ProcedureTemplatePhotographedAssetModel,
    ProcedurePhotographedAsset(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    PhotographedWith,
    ProcedureTemplatePhotographedWithModel,
    ProcedurePhotographedWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    Photograph,
    ProcedureTemplatePhotographModel,
    ProcedurePhotograph(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for PhotographProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Procedure" => Ok(Self::Procedure),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "PhotographedAsset" => Ok(Self::PhotographedAsset),
            "ProcedureTemplatePhotographedAssetModel" => {
                Ok(Self::ProcedureTemplatePhotographedAssetModel)
            }
            "ProcedurePhotographedAsset" => Ok(Self::ProcedurePhotographedAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "PhotographedWith" => Ok(Self::PhotographedWith),
            "ProcedureTemplatePhotographedWithModel" => {
                Ok(Self::ProcedureTemplatePhotographedWithModel)
            }
            "ProcedurePhotographedWith" => Ok(Self::ProcedurePhotographedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "Photograph" => Ok(Self::Photograph),
            "ProcedureTemplatePhotographModel" => Ok(Self::ProcedureTemplatePhotographModel),
            "ProcedurePhotograph" => Ok(Self::ProcedurePhotograph(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure" => Ok(Self::Procedure),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "photographed_asset" => Ok(Self::PhotographedAsset),
            "procedure_template_photographed_asset_model" => {
                Ok(Self::ProcedureTemplatePhotographedAssetModel)
            }
            "procedure_photographed_asset" => Ok(Self::ProcedurePhotographedAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "photographed_with" => Ok(Self::PhotographedWith),
            "procedure_template_photographed_with_model" => {
                Ok(Self::ProcedureTemplatePhotographedWithModel)
            }
            "procedure_photographed_with" => Ok(Self::ProcedurePhotographedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "photograph" => Ok(Self::Photograph),
            "procedure_template_photograph_model" => Ok(Self::ProcedureTemplatePhotographModel),
            "procedure_photograph" => Ok(Self::ProcedurePhotograph(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
        T1,
    >
{
    type Attribute = PhotographProcedureAttribute;
}
impl web_common_traits::database::TableField for PhotographProcedureAttribute {}
impl web_common_traits::database::HasTableType for PhotographProcedureAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    > for PhotographProcedureAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        PhotographProcedureAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for PhotographProcedureAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        PhotographProcedureAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for PhotographProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "photograph_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "photograph_procedures.procedure_template")
            }
            Self::PhotographedAsset => {
                write!(f, "photograph_procedures.photographed_asset")
            }
            Self::ProcedureTemplatePhotographedAssetModel => {
                write!(f, "photograph_procedures.procedure_template_photographed_asset_model")
            }
            Self::ProcedurePhotographedAsset(e) => write!(f, "photograph_procedures.{e}"),
            Self::PhotographedWith => {
                write!(f, "photograph_procedures.photographed_with")
            }
            Self::ProcedureTemplatePhotographedWithModel => {
                write!(f, "photograph_procedures.procedure_template_photographed_with_model")
            }
            Self::ProcedurePhotographedWith(e) => write!(f, "photograph_procedures.{e}"),
            Self::Photograph => write!(f, "photograph_procedures.photograph"),
            Self::ProcedureTemplatePhotographModel => {
                write!(f, "photograph_procedures.procedure_template_photograph_model")
            }
            Self::ProcedurePhotograph(e) => write!(f, "photograph_procedures.{e}"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhotographProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) photographed_asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photographed_asset_model: i32,
    pub(crate) procedure_photographed_asset: ::rosetta_uuid::Uuid,
    pub(crate) photographed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photographed_with_model: i32,
    pub(crate) procedure_photographed_with: ::rosetta_uuid::Uuid,
    pub(crate) photograph: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_photograph_model: i32,
    pub(crate) procedure_photograph: ::rosetta_uuid::Uuid,
}
impl InsertablePhotographProcedure {
    pub fn photograph<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::photographs::Photograph:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::photographs::Photograph::read(
            self.photograph,
            conn,
        )
    }
    pub fn photographed_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(photographed_asset) = self.photographed_asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(
            photographed_asset,
            conn,
        )
        .optional()
    }
    pub fn photographed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::cameras::Camera>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cameras::Camera:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::cameras::Camera::read(photographed_with, conn)
            .optional()
    }
    pub fn procedure_photograph<C: diesel::connection::LoadConnection>(
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
            self.procedure_photograph,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photograph_photograph_fkey(
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
                    .eq(&self.procedure_photograph)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.photograph),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photograph_procedure_templ_fkey(
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
                    .eq(&self.procedure_photograph)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photograph_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_photographed_asset<C: diesel::connection::LoadConnection>(
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
            self.procedure_photographed_asset,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_asset_photogr_fkey(
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
        let Some(photographed_asset) = self.photographed_asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_photographed_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(photographed_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_asset_procedu_fkey(
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
                    .eq(&self.procedure_photographed_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photographed_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_photographed_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_photographed_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_with_photogra_fkey(
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
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_photographed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(photographed_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_with_procedur_fkey(
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
                    .eq(&self.procedure_photographed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photographed_with_model),
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
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_photograph_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_photograph_model,
            conn,
        )
    }
    pub fn procedure_template_photographed_asset_model<
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
            self.procedure_template_photographed_asset_model,
            conn,
        )
    }
    pub fn procedure_template_photographed_with_model<
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
            self.procedure_template_photographed_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photographed_asset_model
                            .eq(&self.procedure_template_photographed_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templa_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photograph_model
                            .eq(&self.procedure_template_photograph_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photographed_with_model
                            .eq(&self.procedure_template_photographed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`PhotographProcedure`](crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::PhotographProcedure;
/// use core_structures::tables::insertables::PhotographProcedureSettable;
/// use core_structures::tables::insertables::ProcedureSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let photograph_procedure = PhotographProcedure::new()
///    // Set mandatory fields
///    .procedure_photograph(procedure_photograph)?
///    .procedure_photographed_asset(procedure_photographed_asset)?
///    .procedure_photographed_with(procedure_photographed_with)?
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
pub struct InsertablePhotographProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) photographed_asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photographed_asset_model: Option<i32>,
    pub(crate) procedure_photographed_asset: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) photographed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photographed_with_model: Option<i32>,
    pub(crate) procedure_photographed_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) photograph: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photograph_model: Option<i32>,
    pub(crate) procedure_photograph: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl<Procedure> diesel::associations::HasTable for InsertablePhotographProcedureBuilder<Procedure> {
    type Table =
        crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::table
    }
}
impl From<InsertablePhotographProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertablePhotographProcedureBuilder,
    >
{
    fn from(builder: InsertablePhotographProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
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
            && (self.procedure_template_photographed_asset_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_photographed_asset.is_complete())
            && self.procedure_photographed_asset.is_complete()
            && (self.procedure_template_photographed_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_photographed_with.is_complete())
            && self.procedure_photographed_with.is_complete()
            && (self.photograph.is_some() || self.procedure_photograph.is_complete())
            && (self.procedure_template_photograph_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_photograph.is_complete())
            && self.procedure_photograph.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `PhotographProcedure` or descendant tables.
pub trait PhotographProcedureSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.photograph_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.photograph_procedures.procedure_template` column.
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
    /// Sets the value of the `public.photograph_procedures.photographed_asset`
    /// column.
    ///
    /// # Arguments
    /// * `photographed_asset`: The value to set for the
    ///   `public.photograph_procedures.photographed_asset` column.
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
    fn photographed_asset<PA>(self, photographed_asset: PA) -> Result<Self, Self::Error>
    where
        PA: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.photograph_procedures.
    /// procedure_template_photographed_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_photographed_asset_model`: The value to set for
    ///   the `public.photograph_procedures.
    ///   procedure_template_photographed_asset_model` column.
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
    fn procedure_template_photographed_asset_model<PTPAM>(
        self,
        procedure_template_photographed_asset_model: PTPAM,
    ) -> Result<Self, Self::Error>
    where
        PTPAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_photographed_asset` column.
    ///
    /// # Arguments
    /// * `procedure_photographed_asset`: The value to set for the
    ///   `public.photograph_procedures.procedure_photographed_asset` column.
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
    fn procedure_photographed_asset<PPA>(
        self,
        procedure_photographed_asset: PPA,
    ) -> Result<Self, Self::Error>
    where
        PPA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.photograph_procedures.photographed_with`
    /// column.
    ///
    /// # Arguments
    /// * `photographed_with`: The value to set for the
    ///   `public.photograph_procedures.photographed_with` column.
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
    fn photographed_with<PW>(self, photographed_with: PW) -> Result<Self, Self::Error>
    where
        PW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.photograph_procedures.
    /// procedure_template_photographed_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_photographed_with_model`: The value to set for the
    ///   `public.photograph_procedures.
    ///   procedure_template_photographed_with_model` column.
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
    fn procedure_template_photographed_with_model<PTPWM>(
        self,
        procedure_template_photographed_with_model: PTPWM,
    ) -> Result<Self, Self::Error>
    where
        PTPWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_photographed_with` column.
    ///
    /// # Arguments
    /// * `procedure_photographed_with`: The value to set for the
    ///   `public.photograph_procedures.procedure_photographed_with` column.
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
    fn procedure_photographed_with<PPW>(
        self,
        procedure_photographed_with: PPW,
    ) -> Result<Self, Self::Error>
    where
        PPW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.photograph_procedures.photograph` column.
    ///
    /// # Arguments
    /// * `photograph`: The value to set for the
    ///   `public.photograph_procedures.photograph` column.
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
    fn photograph<P>(self, photograph: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_template_photograph_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_photograph_model`: The value to set for the
    ///   `public.photograph_procedures.procedure_template_photograph_model`
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
    fn procedure_template_photograph_model<PTPM>(
        self,
        procedure_template_photograph_model: PTPM,
    ) -> Result<Self, Self::Error>
    where
        PTPM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_photograph` column.
    ///
    /// # Arguments
    /// * `procedure_photograph`: The value to set for the
    ///   `public.photograph_procedures.procedure_photograph` column.
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
    fn procedure_photograph<PP>(
        self,
        procedure_photograph: PP,
    ) -> Result<Self, Self::Error>
    where
        PP: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
            >,
        >,
> PhotographProcedureSettable for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.photograph_procedures.procedure_template` column.
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
    ///subgraph v6 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_photograph_model"}
    ///class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_photographed_asset_model"}
    ///class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_photographed_with_model"}
    ///class v3 directly-involved-column
    ///end
    ///subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "procedure_template_asset_model"}
    ///class v5 undirectly-involved-column
    ///end
    ///subgraph v8 ["`procedures`"]
    ///    v4@{shape: rounded, label: "procedure_template"}
    ///class v4 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v4
    ///v0 -.->|"`foreign defines`"| v1
    ///v0 -.->|"`foreign defines`"| v2
    ///v0 -.->|"`foreign defines`"| v3
    ///v1 --->|"`associated same as`"| v5
    ///v2 --->|"`associated same as`"| v5
    ///v3 --->|"`associated same as`"| v5
    ///v6 --->|"`extends`"| v8
    ///v6 ---o|"`associated with`"| v7
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
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photographed_asset` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_asset"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_photographed_asset"}
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
    fn photographed_asset<PA>(
        mut self,
        photographed_asset: PA,
    ) -> Result<Self, Self::Error>
    where
        PA: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let photographed_asset = <PA as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
            &photographed_asset,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_photographed_asset,
        ) = self.procedure_photographed_asset
        {
            self.procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_photographed_asset,
                    photographed_asset,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedAsset(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.photographed_asset = photographed_asset;
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_template_photographed_asset_model` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_photographed_asset"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_photographed_asset_model"}
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
    fn procedure_template_photographed_asset_model<PTPAM>(
        mut self,
        procedure_template_photographed_asset_model: PTPAM,
    ) -> Result<Self, Self::Error>
    where
        PTPAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_photographed_asset_model = <PTPAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_photographed_asset_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_photographed_asset,
        ) = self.procedure_photographed_asset
        {
            self.procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_photographed_asset,
                    procedure_template_photographed_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedAsset(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_photographed_asset_model = Some(
            procedure_template_photographed_asset_model,
        );
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_photographed_asset` column.
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
    ///subgraph v6 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_asset"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_photographed_asset"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_photographed_asset_model"}
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
    fn procedure_photographed_asset<PPA>(
        mut self,
        procedure_photographed_asset: PPA,
    ) -> Result<Self, Self::Error>
    where
        PPA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_photographed_asset = procedure_photographed_asset.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photographed_asset {
            procedure_photographed_asset = if let (
                Some(photographed_asset),
                Some(asset),
            ) = (self.photographed_asset, builder.asset) {
                if photographed_asset != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::PhotographedAsset,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.photographed_asset = Some(asset);
                builder.into()
            } else if let Some(photographed_asset) = self.photographed_asset {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        photographed_asset,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedAsset(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photographed_asset {
            procedure_photographed_asset = if let (
                Some(procedure_template_photographed_asset_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_photographed_asset_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_photographed_asset_model
                    != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplatePhotographedAssetModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_photographed_asset_model = Some(
                    procedure_template_asset_model,
                );
                builder.into()
            } else if let Some(procedure_template_photographed_asset_model) = self
                .procedure_template_photographed_asset_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_photographed_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedAsset(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_photographed_asset = procedure_photographed_asset;
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photographed_with` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_with"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_photographed_with"}
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
    fn photographed_with<PW>(
        mut self,
        photographed_with: PW,
    ) -> Result<Self, Self::Error>
    where
        PW: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let photographed_with = <PW as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
            &photographed_with,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_photographed_with,
        ) = self.procedure_photographed_with
        {
            self.procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_photographed_with,
                    photographed_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedWith(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.photographed_with = photographed_with;
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_template_photographed_with_model` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_photographed_with"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_photographed_with_model"}
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
    fn procedure_template_photographed_with_model<PTPWM>(
        mut self,
        procedure_template_photographed_with_model: PTPWM,
    ) -> Result<Self, Self::Error>
    where
        PTPWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_photographed_with_model = <PTPWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_photographed_with_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_photographed_with,
        ) = self.procedure_photographed_with
        {
            self.procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_photographed_with,
                    procedure_template_photographed_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedWith(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_photographed_with_model = Some(
            procedure_template_photographed_with_model,
        );
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_photographed_with` column.
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
    ///subgraph v6 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_with"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_photographed_with"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_photographed_with_model"}
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
    fn procedure_photographed_with<PPW>(
        mut self,
        procedure_photographed_with: PPW,
    ) -> Result<Self, Self::Error>
    where
        PPW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_photographed_with = procedure_photographed_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photographed_with {
            procedure_photographed_with = if let (
                Some(photographed_with),
                Some(asset),
            ) = (self.photographed_with, builder.asset) {
                if photographed_with != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::PhotographedWith,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.photographed_with = Some(asset);
                builder.into()
            } else if let Some(photographed_with) = self.photographed_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        photographed_with,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedWith(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photographed_with {
            procedure_photographed_with = if let (
                Some(procedure_template_photographed_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_photographed_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_photographed_with_model
                    != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplatePhotographedWithModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_photographed_with_model = Some(
                    procedure_template_asset_model,
                );
                builder.into()
            } else if let Some(procedure_template_photographed_with_model) = self
                .procedure_template_photographed_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_photographed_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotographedWith(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_photographed_with = procedure_photographed_with;
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photograph` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photograph"}
    ///class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_photograph"}
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
    fn photograph<P>(mut self, photograph: P) -> Result<Self, Self::Error>
    where
        P: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        let photograph = <P as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &photograph,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photograph) = self
            .procedure_photograph
        {
            self.procedure_photograph = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_photograph,
                    photograph,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotograph(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.photograph = Some(photograph);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_template_photograph_model` column.
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
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_photograph"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_photograph_model"}
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
    fn procedure_template_photograph_model<PTPM>(
        mut self,
        procedure_template_photograph_model: PTPM,
    ) -> Result<Self, Self::Error>
    where
        PTPM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_photograph_model = <PTPM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template_photograph_model,
        );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photograph) = self
            .procedure_photograph
        {
            self.procedure_photograph = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_photograph,
                    procedure_template_photograph_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotograph(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.procedure_template_photograph_model = Some(
            procedure_template_photograph_model,
        );
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.procedure_photograph` column.
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
    ///subgraph v6 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photograph"}
    ///class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_photograph"}
    ///class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_photograph_model"}
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
    fn procedure_photograph<PP>(
        mut self,
        procedure_photograph: PP,
    ) -> Result<Self, Self::Error>
    where
        PP: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_photograph = procedure_photograph.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photograph {
            procedure_photograph = if let (Some(photograph), Some(asset)) = (
                self.photograph,
                builder.asset,
            ) {
                if photograph != asset {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::Photograph,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.photograph = Some(asset);
                builder.into()
            } else if let Some(photograph) = self.photograph {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        photograph,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotograph(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_photograph {
            procedure_photograph = if let (
                Some(procedure_template_photograph_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_photograph_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_photograph_model != procedure_template_asset_model
                {
                    return Err(
                        web_common_traits::database::InsertError::BuilderError(
                            web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                                <Self as common_traits::builder::Attributed>::Attribute::ProcedureTemplatePhotographModel,
                            ),
                        ),
                    );
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) = builder
                .procedure_template_asset_model
            {
                self.procedure_template_photograph_model = Some(
                    procedure_template_asset_model,
                );
                builder.into()
            } else if let Some(procedure_template_photograph_model) = self
                .procedure_template_photograph_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_photograph_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            <Self as common_traits::builder::Attributed>::Attribute::ProcedurePhotograph(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_photograph = procedure_photograph;
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
for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
    ///subgraph v2 ["`photograph_procedures`"]
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
        <Self as PhotographProcedureSettable>::procedure_template(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertablePhotographProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhotographProcedureBuilder<Procedure>
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
for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
            Error = web_common_traits::database::InsertError<
                PhotographProcedureAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<PhotographProcedureAttribute>;
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
