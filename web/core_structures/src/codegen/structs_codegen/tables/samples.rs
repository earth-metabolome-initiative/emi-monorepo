#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource,
        foreign_key = sample_source
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
        foreign_key = sample_source_model
    )
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::samples::samples)]
pub struct Sample {
    pub id: ::rosetta_uuid::Uuid,
    pub model: i32,
    pub sample_source: Option<::rosetta_uuid::Uuid>,
    pub sample_source_model: i32,
}
impl web_common_traits::prelude::TableName for Sample {
    const TABLE_NAME: &'static str = "samples";
}
impl<'a> From<&'a Sample>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder,
    >
{
    fn from(value: &'a Sample) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::assets::Asset,
    > for Sample
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    > for Sample
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::samples::Sample,
    > for Sample
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for Sample {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for Sample {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl Sample {
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_models::SampleModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_models::SampleModel::read(self.model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn samples_model_sample_source_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::sample_models::SampleModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::sample_models::sample_models::dsl::id
                    .eq(&self.model)
                    .and(
                        crate::codegen::diesel_codegen::tables::sample_models::sample_models::dsl::sample_source_model
                            .eq(&self.sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::sample_models::SampleModel,
            >(conn)
    }
    pub fn sample_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(sample_source) = self.sample_source else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(
            sample_source,
            conn,
        )
        .optional()
    }
    pub fn sample_source_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel::read(
            self.sample_source_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn samples_sample_source_sample_source_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::assets::Asset>, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(sample_source) = self.sample_source else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id
                    .eq(sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                            .eq(&self.sample_source_model),
                    ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
            .optional()
    }
    pub fn from_model<C>(
        model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::samples::samples::model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::samples::samples::model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::samples::samples::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::samples::samples::model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::samples::samples::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::samples::samples;
        Self::table()
            .filter(samples::model.eq(model))
            .order_by(samples::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_and_sample_source_model(
        model: i32,
        sample_source_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::samples::samples;
        Self::table()
            .filter(
                samples::model.eq(model).and(samples::sample_source_model.eq(sample_source_model)),
            )
            .order_by(samples::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sample_source_and_sample_source_model(
        sample_source: ::rosetta_uuid::Uuid,
        sample_source_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::samples::samples;
        Self::table()
            .filter(
                samples::sample_source
                    .eq(sample_source)
                    .and(samples::sample_source_model.eq(sample_source_model)),
            )
            .order_by(samples::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name_and_model(
        name: &str,
        model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
        Self::table()
            .inner_join(assets::table.on(samples::id.eq(assets::id)))
            .filter(assets::name.eq(name).and(assets::model.eq(model)))
            .order_by(samples::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
        Self::table()
            .inner_join(assets::table.on(samples::id.eq(assets::id)))
            .filter(assets::name.eq(name))
            .order_by(samples::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
        Self::table()
            .inner_join(assets::table.on(samples::id.eq(assets::id)))
            .filter(assets::description.eq(description))
            .order_by(samples::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
        Self::table()
            .inner_join(assets::table.on(samples::id.eq(assets::id)))
            .filter(assets::created_by.eq(created_by))
            .order_by(samples::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, samples::samples};
        Self::table()
            .inner_join(assets::table.on(samples::id.eq(assets::id)))
            .filter(assets::updated_by.eq(updated_by))
            .order_by(samples::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<Sample> for Sample {
    fn as_ref(&self) -> &Sample {
        self
    }
}
