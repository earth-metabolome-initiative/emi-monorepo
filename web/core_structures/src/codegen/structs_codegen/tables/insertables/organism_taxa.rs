#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismTaxonAttributes {
    CreatedBy,
    CreatedAt,
    OrganismId,
    TaxonId,
}
impl core::str::FromStr for InsertableOrganismTaxonAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "OrganismId" => Ok(Self::OrganismId),
            "TaxonId" => Ok(Self::TaxonId),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "organism_id" => Ok(Self::OrganismId),
            "taxon_id" => Ok(Self::TaxonId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableOrganismTaxonAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::OrganismId => write!(f, "organism_id"),
            Self::TaxonId => write!(f, "taxon_id"),
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
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) organism_id: ::rosetta_uuid::Uuid,
    pub(crate) taxon_id: i32,
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
/// Trait defining setters for attributes of an instance of `OrganismTaxon` or
/// descendant tables.
pub trait OrganismTaxonBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.organism_taxa.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.organism_taxa.created_by` column.
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
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.organism_taxa.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.organism_taxa.created_at` column.
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.organism_taxa.organism_id` column.
    ///
    /// # Arguments
    /// * `organism_id`: The value to set for the
    ///   `public.organism_taxa.organism_id` column.
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
    fn organism(
        self,
        organism_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.organism_taxa.taxon_id` column.
    ///
    /// # Arguments
    /// * `taxon_id`: The value to set for the `public.organism_taxa.taxon_id`
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
    fn taxon(
        self,
        taxon_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl OrganismTaxonBuildable for InsertableOrganismTaxonBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonAttributes;
    /// Sets the value of the `public.organism_taxa.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.organism_taxa.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableOrganismTaxonAttributes::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.organism_taxa.organism_id` column.
    fn organism(
        mut self,
        organism_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.organism_id = Some(organism_id);
        Ok(self)
    }
    /// Sets the value of the `public.organism_taxa.taxon_id` column.
    fn taxon(
        mut self,
        taxon_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.taxon_id = Some(taxon_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableOrganismTaxonBuilder {
    type PrimaryKey = (::rosetta_uuid::Uuid, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableOrganismTaxonBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
            Error = web_common_traits::database::InsertError<InsertableOrganismTaxonAttributes>,
        >,
{
    type Attributes = InsertableOrganismTaxonAttributes;
    fn is_complete(&self) -> bool {
        self.created_by.is_some()
            && self.created_at.is_some()
            && self.organism_id.is_some()
            && self.taxon_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
