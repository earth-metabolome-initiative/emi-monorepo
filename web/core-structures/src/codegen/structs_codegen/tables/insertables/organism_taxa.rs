#[derive(Clone, core::fmt::Debug)]
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
    created_at: rosetta_timestamp::TimestampUTC,
    organism_id: rosetta_uuid::Uuid,
    taxon_id: i32,
}
impl InsertableOrganismTaxon {
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::id
                    .eq(&self.organism_id),
            )
            .first::<crate::codegen::structs_codegen::tables::organisms::Organism>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn taxon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::taxa::Taxon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::taxa::Taxon::table()
            .filter(crate::codegen::diesel_codegen::tables::taxa::taxa::dsl::id.eq(&self.taxon_id))
            .first::<crate::codegen::structs_codegen::tables::taxa::Taxon>(conn)
            .await
    }
}
pub struct InsertableOrganismTaxonBuilder {
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    organism_id: Option<rosetta_uuid::Uuid>,
    taxon_id: Option<i32>,
}
impl Default for InsertableOrganismTaxonBuilder {
    fn default() -> Self {
        Self {
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            organism_id: None,
            taxon_id: None,
        }
    }
}
impl InsertableOrganismTaxonBuilder {
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableOrganismTaxonAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn organism_id<P>(
        mut self,
        organism_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organism_id =
            organism_id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableOrganismTaxonAttributes::OrganismId)
            })?;
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    pub fn taxon_id<P>(
        mut self,
        taxon_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
impl common_traits::prelude::Builder for InsertableOrganismTaxonBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>;
    type Object = InsertableOrganismTaxon;
    type Attribute = InsertableOrganismTaxonAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::CreatedAt,
                ),
            )?,
            organism_id: self.organism_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::OrganismId,
                ),
            )?,
            taxon_id: self.taxon_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismTaxonAttributes::TaxonId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableOrganismTaxon> for InsertableOrganismTaxonBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganismTaxon) -> Result<Self, Self::Error> {
        Self::default()
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .organism_id(insertable_variant.organism_id)?
            .taxon_id(insertable_variant.taxon_id)
    }
}
