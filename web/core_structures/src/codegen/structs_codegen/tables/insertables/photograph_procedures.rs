#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhotographProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for PhotographProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
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
}
impl core::str::FromStr for PhotographProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for PhotographProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::PhotographedAsset => write!(f, "photographed_asset"),
            Self::ProcedureTemplatePhotographedAssetModel => {
                write!(f, "procedure_template_photographed_asset_model")
            }
            Self::ProcedurePhotographedAsset(e) => write!(f, "{e}"),
            Self::PhotographedWith => write!(f, "photographed_with"),
            Self::ProcedureTemplatePhotographedWithModel => {
                write!(f, "procedure_template_photographed_with_model")
            }
            Self::ProcedurePhotographedWith(e) => write!(f, "{e}"),
        }
    }
}
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
    pub(crate) photographed_asset: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_photographed_asset_model: i32,
    pub(crate) procedure_photographed_asset: ::rosetta_uuid::Uuid,
    pub(crate) photographed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_photographed_with_model: i32,
    pub(crate) procedure_photographed_with: ::rosetta_uuid::Uuid,
}
impl InsertablePhotographProcedure {
    pub fn photographed_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::table(),
                self.photographed_asset,
            ),
            conn,
        )
    }
    pub fn photographed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::cameras::Camera>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cameras::Camera: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cameras::Camera,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cameras::Camera::table(),
                photographed_with,
            ),
            conn,
        )
        .map(Some)
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.procedure,
            ),
            conn,
        )
    }
    pub fn procedure_photographed_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table(),
                self.procedure_photographed_asset,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_asset_photogr_fkey(
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
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.photographed_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
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
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table(),
                self.procedure_photographed_with,
            ),
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
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
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
            .map(Some)
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
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
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
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table(),
                self.procedure_template,
            ),
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
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_photographed_asset_model,
            ),
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
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_photographed_with_model,
            ),
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub(crate) procedure: Procedure,
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
/// Trait defining setters for attributes of an instance of
/// `PhotographProcedure` or descendant tables.
pub trait PhotographProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
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
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn photographed_asset(
        self,
        photographed_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure_template_photographed_asset_model(
        self,
        procedure_template_photographed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn photographed_with(
        self,
        photographed_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure_template_photographed_with_model(
        self,
        procedure_template_photographed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPW: Into<
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
> PhotographProcedureSettable for InsertablePhotographProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute;
    /// Sets the value of the `public.photograph_procedures.procedure_template`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v5 ["`photograph_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_photographed_asset_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_photographed_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v6 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 undirectly-involved-column
    /// end
    /// subgraph v7 ["`procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template"}
    /// class v3 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v4
    /// v2 --->|"`associated same as`"| v4
    /// v0 --->|"`ancestral same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
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
    /// Sets the value of the `public.photograph_procedures.photographed_asset`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_asset"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_photographed_asset"}
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
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn photographed_asset(
        mut self,
        photographed_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photographed_asset) =
            self.procedure_photographed_asset
        {
            self.procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_photographed_asset,
                    Some(photographed_asset),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePhotographedAsset(attribute)
                    })
                })?
                .into();
        }
        self.photographed_asset = Some(photographed_asset);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.photograph_procedures.
    /// procedure_template_photographed_asset_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`photograph_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_photographed_asset_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "procedure_photographed_asset"}
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
    fn procedure_template_photographed_asset_model(
        mut self,
        procedure_template_photographed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photographed_asset) =
            self.procedure_photographed_asset
        {
            self.procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_photographed_asset,
                    procedure_template_photographed_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePhotographedAsset(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_photographed_asset_model =
            Some(procedure_template_photographed_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_photographed_asset` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v6 ["`photograph_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_photographed_asset_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "photographed_asset"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_photographed_asset"}
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
    fn procedure_photographed_asset<PPA>(
        mut self,
        procedure_photographed_asset: PPA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_photographed_asset = procedure_photographed_asset.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_photographed_asset
        {
            procedure_photographed_asset = if let (Some(photographed_asset), Some(asset)) =
                (self.photographed_asset, builder.asset)
            {
                if photographed_asset != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PhotographedAsset,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.photographed_asset = Some(asset);
                builder.into()
            } else if let Some(photographed_asset) = self.photographed_asset {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(photographed_asset),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePhotographedAsset(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_photographed_asset
        {
            procedure_photographed_asset = if let (
                Some(procedure_template_photographed_asset_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_photographed_asset_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_photographed_asset_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplatePhotographedAssetModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_photographed_asset_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_photographed_asset_model) =
                self.procedure_template_photographed_asset_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_photographed_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePhotographedAsset(attribute)
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
    /// Sets the value of the `public.photograph_procedures.photographed_with`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "photographed_with"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_photographed_with"}
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
    fn photographed_with(
        mut self,
        photographed_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photographed_with) =
            self.procedure_photographed_with
        {
            self.procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_photographed_with,
                    photographed_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePhotographedWith(attribute)
                    })
                })?
                .into();
        }
        self.photographed_with = photographed_with;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.photograph_procedures.
    /// procedure_template_photographed_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_photographed_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_photographed_with_model"}
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
    fn procedure_template_photographed_with_model(
        mut self,
        procedure_template_photographed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_photographed_with) =
            self.procedure_photographed_with
        {
            self.procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_photographed_with,
                    procedure_template_photographed_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePhotographedWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_photographed_with_model =
            Some(procedure_template_photographed_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.photograph_procedures.procedure_photographed_with` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v6 ["`photograph_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_photographed_with"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "photographed_with"}
    /// class v0 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_photographed_with_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_photographed_with<PPW>(
        mut self,
        procedure_photographed_with: PPW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_photographed_with = procedure_photographed_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_photographed_with
        {
            procedure_photographed_with = if let (Some(photographed_with), Some(asset)) =
                (self.photographed_with, builder.asset)
            {
                if photographed_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PhotographedWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.photographed_with = Some(asset);
                builder.into()
            } else if let Some(photographed_with) = self.photographed_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(photographed_with),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePhotographedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_photographed_with
        {
            procedure_photographed_with = if let (
                Some(procedure_template_photographed_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_photographed_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_photographed_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplatePhotographedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_photographed_with_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_photographed_with_model) =
                self.procedure_template_photographed_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_photographed_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePhotographedWith(attribute)
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
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute;
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
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`photograph_procedures`"]
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
        <Self as PhotographProcedureSettable>::procedure_template(
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
        Error = web_common_traits::database::InsertError<PhotographProcedureAttribute>,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = PhotographProcedureAttribute;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete() && self.procedure_template.is_some()
            && self.photographed_asset.is_some()
            && self.procedure_template_photographed_asset_model.is_some()
            && self.procedure_photographed_asset.is_complete()
            && self.procedure_template_photographed_with_model.is_some()
            && self.procedure_photographed_with.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
