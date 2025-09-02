#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(procedure_template))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates
)]
pub struct PhotographProcedureTemplate {
    pub procedure_template: i32,
    pub photographed_with_model: i32,
    pub procedure_template_photographed_with_model: i32,
    pub photographed_asset_model: i32,
    pub foreign_procedure_template: i32,
    pub procedure_template_photographed_asset_model: i32,
}
impl web_common_traits::prelude::TableName for PhotographProcedureTemplate {
    const TABLE_NAME: &'static str = "photograph_procedure_templates";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for PhotographProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
> for PhotographProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for PhotographProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl PhotographProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn photographed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::camera_models::CameraModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::camera_models::CameraModel::table(),
                self.photographed_with_model,
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
    pub fn photographed_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::table(),
                self.photographed_asset_model,
            ),
            conn,
        )
    }
    pub fn foreign_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.foreign_procedure_template,
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
    #[cfg(feature = "postgres")]
    pub fn from_photographed_with_model(
        photographed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::photographed_with_model.eq(photographed_with_model),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_with_model(
        procedure_template_photographed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_with_model
                    .eq(procedure_template_photographed_with_model),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photographed_asset_model(
        photographed_asset_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::photographed_asset_model
                    .eq(photographed_asset_model),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_foreign_procedure_template(
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::foreign_procedure_template
                    .eq(foreign_procedure_template),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_asset_model(
        procedure_template_photographed_asset_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_asset_model
                    .eq(procedure_template_photographed_asset_model),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_foreign_procedure_template(
        procedure_template: &i32,
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template.eq(procedure_template).and(
                    photograph_procedure_templates::foreign_procedure_template
                        .eq(foreign_procedure_template),
                ),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_with_model_and_photographed_with_model(
        procedure_template_photographed_with_model: &i32,
        photographed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_with_model
                    .eq(procedure_template_photographed_with_model)
                    .and(
                        photograph_procedure_templates::photographed_with_model
                            .eq(photographed_with_model),
                    ),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_with_model_and_procedure_template(
        procedure_template_photographed_with_model: &i32,
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_with_model
                    .eq(procedure_template_photographed_with_model)
                    .and(photograph_procedure_templates::procedure_template.eq(procedure_template)),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_asset_model_and_foreign_procedure_template(
        procedure_template_photographed_asset_model: &i32,
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_asset_model
                    .eq(procedure_template_photographed_asset_model)
                    .and(
                        photograph_procedure_templates::foreign_procedure_template
                            .eq(foreign_procedure_template),
                    ),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_photographed_asset_model_and_photographed_asset_model(
        procedure_template_photographed_asset_model: &i32,
        photographed_asset_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
        Self::table()
            .filter(
                photograph_procedure_templates::procedure_template_photographed_asset_model
                    .eq(procedure_template_photographed_asset_model)
                    .and(
                        photograph_procedure_templates::photographed_asset_model
                            .eq(photographed_asset_model),
                    ),
            )
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::most_concrete_table.eq(most_concrete_table))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecated(
        deprecated: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::icon.eq(icon))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::created_at.eq(created_at))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            photograph_procedure_templates::photograph_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(photograph_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template)),
            )
            .filter(procedure_templates::updated_at.eq(updated_at))
            .order_by(photograph_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PhotographProcedureTemplate> for PhotographProcedureTemplate {
    fn as_ref(&self) -> &PhotographProcedureTemplate {
        self
    }
}
