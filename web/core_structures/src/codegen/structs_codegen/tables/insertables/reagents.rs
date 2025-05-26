#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentAttributes {
    Id,
    Purity,
    CasCode,
    MolecularFormulas,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableReagentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableReagentAttributes::Id => write!(f, "id"),
            InsertableReagentAttributes::Purity => write!(f, "purity"),
            InsertableReagentAttributes::CasCode => write!(f, "cas_code"),
            InsertableReagentAttributes::MolecularFormulas => {
                write!(f, "molecular_formulas")
            }
            InsertableReagentAttributes::CreatedBy => write!(f, "created_by"),
            InsertableReagentAttributes::CreatedAt => write!(f, "created_at"),
            InsertableReagentAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableReagentAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::reagents::reagents)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableReagent {
    id: i32,
    purity: f32,
    cas_code: ::cas_codes::CAS,
    molecular_formulas: ::molecular_formulas::MolecularFormula,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableReagent {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackable_categories::trackable_categories::dsl::id
                    .eq(&self.id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory,
            >(conn)
            .await
    }
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
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
pub struct InsertableReagentBuilder {
    id: Option<i32>,
    purity: Option<f32>,
    cas_code: Option<::cas_codes::CAS>,
    molecular_formulas: Option<::molecular_formulas::MolecularFormula>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableReagentBuilder {
    fn default() -> Self {
        Self {
            id: None,
            purity: None,
            cas_code: None,
            molecular_formulas: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableReagentBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn purity<P>(
        mut self,
        purity: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let purity = purity.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::Purity)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(purity)
            .map_err(|e| e.rename_field(InsertableReagentAttributes::Purity))
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(purity, 100f32)
                    .map_err(|e| e.rename_field(InsertableReagentAttributes::Purity))
            })?;
        self.purity = Some(purity);
        Ok(self)
    }
    pub fn cas_code<P>(
        mut self,
        cas_code: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::cas_codes::CAS>,
        <P as TryInto<::cas_codes::CAS>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let cas_code =
            cas_code.try_into().map_err(|err: <P as TryInto<::cas_codes::CAS>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::CasCode)
            })?;
        self.cas_code = Some(cas_code);
        Ok(self)
    }
    pub fn molecular_formulas<P>(
        mut self,
        molecular_formulas: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::molecular_formulas::MolecularFormula>,
        <P as TryInto<::molecular_formulas::MolecularFormula>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let molecular_formulas = molecular_formulas.try_into().map_err(
            |err: <P as TryInto<::molecular_formulas::MolecularFormula>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::MolecularFormulas)
            },
        )?;
        self.molecular_formulas = Some(molecular_formulas);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableReagentBuilder {
    type Error = web_common_traits::database::InsertError<InsertableReagentAttributes>;
    type Object = InsertableReagent;
    type Attribute = InsertableReagentAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableReagentAttributes::Id,
            ))?,
            purity: self.purity.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableReagentAttributes::Purity,
            ))?,
            cas_code: self.cas_code.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::CasCode,
                ),
            )?,
            molecular_formulas: self.molecular_formulas.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::MolecularFormulas,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableReagentAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableReagent> for InsertableReagentBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableReagent) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .purity(insertable_variant.purity)?
            .cas_code(insertable_variant.cas_code)?
            .molecular_formulas(insertable_variant.molecular_formulas)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
