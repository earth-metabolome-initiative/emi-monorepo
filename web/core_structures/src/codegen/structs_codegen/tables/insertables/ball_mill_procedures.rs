#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BallMillProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for BallMillProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for BallMillProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BallMillProcedureAttribute {
    Extension(BallMillProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    BeadModel,
    ProcedureTemplateBeadModel,
    ProcedureBead(crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute),
    MilledWithModel,
    ProcedureTemplateMilledWithModel,
    ProcedureMilledWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    MilledWith,
    MilledContainer,
    MilledContainerModel,
    ProcedureTemplateMilledContainerModel,
    ProcedureMilledContainer(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for BallMillProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "BeadModel" => Ok(Self::BeadModel),
            "ProcedureTemplateBeadModel" => Ok(Self::ProcedureTemplateBeadModel),
            "ProcedureBead" => Ok(Self::ProcedureBead(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "MilledWithModel" => Ok(Self::MilledWithModel),
            "ProcedureTemplateMilledWithModel" => Ok(Self::ProcedureTemplateMilledWithModel),
            "ProcedureMilledWith" => Ok(Self::ProcedureMilledWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "MilledWith" => Ok(Self::MilledWith),
            "MilledContainer" => Ok(Self::MilledContainer),
            "MilledContainerModel" => Ok(Self::MilledContainerModel),
            "ProcedureTemplateMilledContainerModel" => {
                Ok(Self::ProcedureTemplateMilledContainerModel)
            }
            "ProcedureMilledContainer" => Ok(Self::ProcedureMilledContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "bead_model" => Ok(Self::BeadModel),
            "procedure_template_bead_model" => Ok(Self::ProcedureTemplateBeadModel),
            "procedure_bead" => Ok(Self::ProcedureBead(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "milled_with_model" => Ok(Self::MilledWithModel),
            "procedure_template_milled_with_model" => Ok(Self::ProcedureTemplateMilledWithModel),
            "procedure_milled_with" => Ok(Self::ProcedureMilledWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "milled_with" => Ok(Self::MilledWith),
            "milled_container" => Ok(Self::MilledContainer),
            "milled_container_model" => Ok(Self::MilledContainerModel),
            "procedure_template_milled_container_model" => {
                Ok(Self::ProcedureTemplateMilledContainerModel)
            }
            "procedure_milled_container" => Ok(Self::ProcedureMilledContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for BallMillProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::BeadModel => write!(f, "bead_model"),
            Self::ProcedureTemplateBeadModel => {
                write!(f, "procedure_template_bead_model")
            }
            Self::ProcedureBead(e) => write!(f, "{e}"),
            Self::MilledWithModel => write!(f, "milled_with_model"),
            Self::ProcedureTemplateMilledWithModel => {
                write!(f, "procedure_template_milled_with_model")
            }
            Self::ProcedureMilledWith(e) => write!(f, "{e}"),
            Self::MilledWith => write!(f, "milled_with"),
            Self::MilledContainer => write!(f, "milled_container"),
            Self::MilledContainerModel => write!(f, "milled_container_model"),
            Self::ProcedureTemplateMilledContainerModel => {
                write!(f, "procedure_template_milled_container_model")
            }
            Self::ProcedureMilledContainer(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) bead_model: i32,
    pub(crate) procedure_template_bead_model: i32,
    pub(crate) procedure_bead: ::rosetta_uuid::Uuid,
    pub(crate) milled_with_model: i32,
    pub(crate) procedure_template_milled_with_model: i32,
    pub(crate) procedure_milled_with: ::rosetta_uuid::Uuid,
    pub(crate) milled_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) milled_container: ::rosetta_uuid::Uuid,
    pub(crate) milled_container_model: i32,
    pub(crate) procedure_template_milled_container_model: i32,
    pub(crate) procedure_milled_container: ::rosetta_uuid::Uuid,
}
impl InsertableBallMillProcedure {
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
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error>
    where
        crate::BallMillProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BeadModel, diesel::result::Error>
    where
        crate::BeadModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BeadModel::read(self.bead_model, conn)
    }
    pub fn procedure_template_bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_bead_model, conn)
    }
    pub fn procedure_bead<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_bead, conn)
    }
    pub fn milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillMachineModel, diesel::result::Error>
    where
        crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillMachineModel::read(self.milled_with_model, conn)
    }
    pub fn procedure_template_milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_milled_with_model, conn)
    }
    pub fn procedure_milled_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_milled_with, conn)
    }
    pub fn milled_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::BallMillMachine>, diesel::result::Error>
    where
        crate::BallMillMachine: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(milled_with) = self.milled_with else {
            return Ok(None);
        };
        crate::BallMillMachine::read(milled_with, conn).map(Some)
    }
    pub fn milled_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainer, diesel::result::Error>
    where
        crate::VolumetricContainer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainer::read(self.milled_container, conn)
    }
    pub fn milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.milled_container_model, conn)
    }
    pub fn procedure_template_milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_milled_container_model,
            conn,
        )
    }
    pub fn procedure_milled_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_milled_container, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_procedure_template_fkey(
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
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_bead_model
                            .eq(&self.procedure_template_bead_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_templat_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_milled_with_model
                            .eq(&self.procedure_template_milled_with_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_templat_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_milled_container_model
                            .eq(&self.procedure_template_milled_container_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_bead_procedure_template_bea_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_bead)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_bead_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_procedure_templ_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_milled_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_procedure_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_milled_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_milled_con_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.milled_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_milled_with_mod_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.milled_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_milled_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureAsset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(milled_with) = self.milled_with else {
            return Ok(None);
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(milled_with),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_bead_bead_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_bead)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.bead_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_milled_co_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.milled_container),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn ball_mill_procedures_milled_with_model_milled_container_mo_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read(
            (self.milled_with_model, self.milled_container_model),
            conn,
        )
    }
    pub fn ball_mill_procedures_milled_with_model_bead_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.milled_with_model, self.bead_model), conn)
    }
    pub fn ball_mill_procedures_bead_model_milled_container_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.bead_model, self.milled_container_model), conn)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) bead_model: Option<i32>,
    pub(crate) procedure_template_bead_model: Option<i32>,
    pub(crate) procedure_bead: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) milled_with_model: Option<i32>,
    pub(crate) procedure_template_milled_with_model: Option<i32>,
    pub(crate) procedure_milled_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) milled_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) milled_container: Option<::rosetta_uuid::Uuid>,
    pub(crate) milled_container_model: Option<i32>,
    pub(crate) procedure_template_milled_container_model: Option<i32>,
    pub(crate) procedure_milled_container: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableBallMillProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableBallMillProcedureBuilder,
    >
{
    fn from(builder: InsertableBallMillProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `BallMillProcedure`
/// or descendant tables.
pub trait BallMillProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.ball_mill_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_template` column.
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
    /// Sets the value of the `public.ball_mill_procedures.bead_model` column.
    ///
    /// # Arguments
    /// * `bead_model`: The value to set for the
    ///   `public.ball_mill_procedures.bead_model` column.
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
    fn bead_model(
        self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_bead_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_bead_model`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_template_bead_model` column.
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
    fn procedure_template_bead_model(
        self,
        procedure_template_bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.ball_mill_procedures.procedure_bead`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_bead`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_bead` column.
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
    fn procedure_bead<PB>(
        self,
        procedure_bead: PB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PB: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.ball_mill_procedures.milled_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedures.milled_with_model` column.
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
    fn milled_with_model(
        self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_milled_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_template_milled_with_model`
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
    fn procedure_template_milled_with_model(
        self,
        procedure_template_milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_milled_with` column.
    ///
    /// # Arguments
    /// * `procedure_milled_with`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_milled_with` column.
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
    fn procedure_milled_with<PMW>(
        self,
        procedure_milled_with: PMW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.ball_mill_procedures.milled_with` column.
    ///
    /// # Arguments
    /// * `milled_with`: The value to set for the
    ///   `public.ball_mill_procedures.milled_with` column.
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
    fn milled_with(
        self,
        milled_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.ball_mill_procedures.milled_container`
    /// column.
    ///
    /// # Arguments
    /// * `milled_container`: The value to set for the
    ///   `public.ball_mill_procedures.milled_container` column.
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
    fn milled_container(
        self,
        milled_container: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.milled_container_model` column.
    ///
    /// # Arguments
    /// * `milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedures.milled_container_model` column.
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
    fn milled_container_model(
        self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_milled_container_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedures.
    ///   procedure_template_milled_container_model` column.
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
    fn procedure_template_milled_container_model(
        self,
        procedure_template_milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_milled_container` column.
    ///
    /// # Arguments
    /// * `procedure_milled_container`: The value to set for the
    ///   `public.ball_mill_procedures.procedure_milled_container` column.
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
    fn procedure_milled_container<PMC>(
        self,
        procedure_milled_container: PMC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMC: Into<
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
> BallMillProcedureSettable for InsertableBallMillProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute;
    /// Sets the value of the `public.ball_mill_procedures.procedure_template`
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
    /// subgraph v6 ["`ball_mill_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_milled_with_model"}
    /// class v3 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_milled_container_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v5 undirectly-involved-column
    /// end
    /// subgraph v8 ["`procedures`"]
    ///    v4@{shape: rounded, label: "procedure_template"}
    /// class v4 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v5
    /// v0 --->|"`ancestral same as`"| v4
    /// v0 -.->|"`foreign defines`"| v3
    /// v0 -.->|"`foreign defines`"| v2
    /// v0 -.->|"`foreign defines`"| v1
    /// v3 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v6 --->|"`extends`"| v8
    /// v6 ---o|"`associated with`"| v7
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
    /// Sets the value of the `public.ball_mill_procedures.bead_model` column.
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
    /// subgraph v6 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 column-of-interest
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_bead"}
    /// class v3 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v0
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn bead_model(
        mut self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_bead) =
            self.procedure_bead
        {
            self.procedure_bead = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_bead,
                    bead_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureBead(attribute)
                    })
                })?
                .into();
        }
        self.bead_model = Some(bead_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_bead_model` column.
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
    /// subgraph v4 ["`ball_mill_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "procedure_bead"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_bead_model(
        mut self,
        procedure_template_bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_bead) =
            self.procedure_bead
        {
            self.procedure_bead = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_bead,
                    procedure_template_bead_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureBead(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_bead_model = Some(procedure_template_bead_model);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedures.procedure_bead`
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
    /// subgraph v6 ["`ball_mill_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_bead"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_bead<PB>(
        mut self,
        procedure_bead: PB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PB: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_bead = procedure_bead.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_bead {
            procedure_bead = if let (
                Some(procedure_template_bead_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_bead_model, builder.procedure_template_asset_model)
            {
                if procedure_template_bead_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateBeadModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_bead_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_bead_model) = self.procedure_template_bead_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_bead_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureBead(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_bead {
            procedure_bead = if let (Some(bead_model), Some(asset_model)) =
                (self.bead_model, builder.asset_model)
            {
                if bead_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::BeadModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.bead_model = Some(asset_model);
                builder.into()
            } else if let Some(bead_model) = self.bead_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        bead_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureBead(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_bead = procedure_bead;
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedures.milled_with_model`
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
    /// subgraph v6 ["`ball_mill_procedures`"]
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_milled_with"}
    /// class v3 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v2
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn milled_with_model(
        mut self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_with) =
            self.procedure_milled_with
        {
            self.procedure_milled_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_milled_with,
                    milled_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledWith(attribute)
                    })
                })?
                .into();
        }
        self.milled_with_model = Some(milled_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_milled_with_model`
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
    /// subgraph v4 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_milled_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_milled_with_model"}
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
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_milled_with_model(
        mut self,
        procedure_template_milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_with) =
            self.procedure_milled_with
        {
            self.procedure_milled_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_milled_with,
                    procedure_template_milled_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_milled_with_model = Some(procedure_template_milled_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_milled_with` column.
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
    /// subgraph v8 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "milled_with"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_milled_with_model"}
    /// class v3 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_with_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_milled_with"}
    /// class v2 column-of-interest
    /// end
    /// subgraph v9 ["`procedure_assets`"]
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    ///    v5@{shape: rounded, label: "asset_model"}
    /// class v5 directly-involved-column
    ///    v4@{shape: rounded, label: "asset"}
    /// class v4 directly-involved-column
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v4
    /// v3 --->|"`associated same as`"| v6
    /// v1 --->|"`associated same as`"| v5
    /// v4 -.->|"`foreign defines`"| v5
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v2 -.->|"`foreign defines`"| v3
    /// v8 ---o|"`associated with`"| v9
    /// ```
    fn procedure_milled_with<PMW>(
        mut self,
        procedure_milled_with: PMW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_milled_with = procedure_milled_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_milled_with {
            procedure_milled_with = if let (
                Some(procedure_template_milled_with_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_milled_with_model, builder.procedure_template_asset_model)
            {
                if procedure_template_milled_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateMilledWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_milled_with_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_milled_with_model) =
                self.procedure_template_milled_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_milled_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_milled_with {
            procedure_milled_with = if let (Some(milled_with_model), Some(asset_model)) =
                (self.milled_with_model, builder.asset_model)
            {
                if milled_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.milled_with_model = Some(asset_model);
                builder.into()
            } else if let Some(milled_with_model) = self.milled_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        milled_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_milled_with {
            procedure_milled_with = if let (Some(milled_with), Some(asset)) =
                (self.milled_with, builder.asset)
            {
                if milled_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.milled_with = Some(asset);
                builder.into()
            } else if let Some(milled_with) = self.milled_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(milled_with),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_milled_with = procedure_milled_with;
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedures.milled_with` column.
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
    /// subgraph v4 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "milled_with"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_milled_with"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn milled_with(
        mut self,
        milled_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_with) =
            self.procedure_milled_with
        {
            self.procedure_milled_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_milled_with,
                    milled_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledWith(attribute)
                    })
                })?
                .into();
        }
        self.milled_with = milled_with;
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedures.milled_container`
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
    /// subgraph v4 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "milled_container"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_milled_container"}
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
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn milled_container(
        mut self,
        milled_container: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_container) =
            self.procedure_milled_container
        {
            self.procedure_milled_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_milled_container,
                    Some(milled_container),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledContainer(attribute)
                    })
                })?
                .into();
        }
        self.milled_container = Some(milled_container);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.milled_container_model` column.
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
    /// subgraph v6 ["`ball_mill_procedures`"]
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_milled_container"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v1
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn milled_container_model(
        mut self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_container) =
            self.procedure_milled_container
        {
            self.procedure_milled_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_milled_container,
                    milled_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledContainer(attribute)
                    })
                })?
                .into();
        }
        self.milled_container_model = Some(milled_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_template_milled_container_model`
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
    /// subgraph v4 ["`ball_mill_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_milled_container"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_milled_container_model"}
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
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_milled_container_model(
        mut self,
        procedure_template_milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_milled_container) =
            self.procedure_milled_container
        {
            self.procedure_milled_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_milled_container,
                    procedure_template_milled_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureMilledContainer(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_milled_container_model =
            Some(procedure_template_milled_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedures.procedure_milled_container` column.
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
    /// subgraph v8 ["`ball_mill_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_milled_container"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_milled_container_model"}
    /// class v3 directly-involved-column
    ///    v0@{shape: rounded, label: "milled_container"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v9 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "asset_model"}
    /// class v5 directly-involved-column
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 directly-involved-column
    ///    v4@{shape: rounded, label: "asset"}
    /// class v4 directly-involved-column
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v2 -.->|"`foreign defines`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v6
    /// v0 --->|"`associated same as`"| v4
    /// v4 -.->|"`foreign defines`"| v5
    /// v8 ---o|"`associated with`"| v9
    /// ```
    fn procedure_milled_container<PMC>(
        mut self,
        procedure_milled_container: PMC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PMC: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_milled_container = procedure_milled_container.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_milled_container
        {
            procedure_milled_container = if let (
                Some(procedure_template_milled_container_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_milled_container_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_milled_container_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateMilledContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_milled_container_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_milled_container_model) =
                self.procedure_template_milled_container_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_milled_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_milled_container
        {
            procedure_milled_container = if let (Some(milled_container_model), Some(asset_model)) =
                (self.milled_container_model, builder.asset_model)
            {
                if milled_container_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.milled_container_model = Some(asset_model);
                builder.into()
            } else if let Some(milled_container_model) = self.milled_container_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        milled_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_milled_container
        {
            procedure_milled_container = if let (Some(milled_container), Some(asset)) =
                (self.milled_container, builder.asset)
            {
                if milled_container != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledContainer,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.milled_container = Some(asset);
                builder.into()
            } else if let Some(milled_container) = self.milled_container {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(milled_container),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureMilledContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_milled_container = procedure_milled_container;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableBallMillProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute;
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
    ///subgraph v2 ["`ball_mill_procedures`"]
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
        <Self as BallMillProcedureSettable>::procedure_template(self, procedure_template)
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
    for InsertableBallMillProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableBallMillProcedureBuilder<Procedure>
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
    for InsertableBallMillProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::BallMillProcedure,
            Error = web_common_traits::database::InsertError<BallMillProcedureAttribute>,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        web_common_traits::database::TryInsertGeneric<C>,
{
    type Attributes = BallMillProcedureAttribute;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && self.bead_model.is_some()
            && self.procedure_template_bead_model.is_some()
            && self.procedure_bead.is_complete()
            && self.milled_with_model.is_some()
            && self.procedure_template_milled_with_model.is_some()
            && self.procedure_milled_with.is_complete()
            && self.milled_container.is_some()
            && self.milled_container_model.is_some()
            && self.procedure_template_milled_container_model.is_some()
            && self.procedure_milled_container.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::BallMillProcedure = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
