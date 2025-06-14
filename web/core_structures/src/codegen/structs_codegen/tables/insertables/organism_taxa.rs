#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismTaxonAttributes {
    CreatedBy,
    CreatedAt,
    OrganismId,
    TaxonId,
}
impl core::fmt::Display for InsertableOrganismTaxonAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismTaxonAttributes::CreatedBy => write!(f, "created_by"),
            InsertableOrganismTaxonAttributes::CreatedAt => write!(f, "created_at"),
            InsertableOrganismTaxonAttributes::OrganismId => write!(f, "organism_id"),
            InsertableOrganismTaxonAttributes::TaxonId => write!(f, "taxon_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::organism_taxa::organism_taxa
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismTaxon {
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    organism_id: ::rosetta_uuid::Uuid,
    taxon_id: i32,
}
impl InsertableOrganismTaxon {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn organism<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organisms::Organism,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::organisms::Organism: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::organisms::Organism,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::organisms::Organism::table(),
                self.organism_id,
            ),
            conn,
        )
    }
    pub fn taxon<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::taxa::Taxon,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::taxa::Taxon: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::taxa::Taxon,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::taxa::Taxon::table(),
                self.taxon_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismTaxonBuilder {
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) organism_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) taxon_id: Option<i32>,
}
impl Default for InsertableOrganismTaxonBuilder {
    fn default() -> Self {
        Self {
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            organism_id: Default::default(),
            taxon_id: Default::default(),
        }
    }
}
impl InsertableOrganismTaxonBuilder {
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismTaxonAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismTaxonAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn organism_id<P>(
        mut self,
        organism_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organism_id = organism_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableOrganismTaxonAttributes::OrganismId)
            },
        )?;
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    pub fn taxon_id<P>(
        mut self,
        taxon_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let taxon_id = taxon_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismTaxonAttributes::TaxonId)
        })?;
        self.taxon_id = Some(taxon_id);
        Ok(self)
    }
}
impl TryFrom<InsertableOrganismTaxonBuilder> for InsertableOrganismTaxon {
    type Error = common_traits::prelude::BuilderError<InsertableOrganismTaxonAttributes>;
    fn try_from(
        builder: InsertableOrganismTaxonBuilder,
    ) -> Result<InsertableOrganismTaxon, Self::Error> {
        Ok(Self {
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::CreatedAt,
                ),
            )?,
            organism_id: builder.organism_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::OrganismId,
                ),
            )?,
            taxon_id: builder.taxon_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::TaxonId,
                ),
            )?,
        })
    }
}
