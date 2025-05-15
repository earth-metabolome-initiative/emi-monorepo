#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentModelCategoryAttributes {
    InstrumentModelId,
    InstrumentCategory,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableInstrumentModelCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentModelCategoryAttributes::InstrumentModelId => {
                write!(f, "instrument_model_id")
            }
            InsertableInstrumentModelCategoryAttributes::InstrumentCategory => {
                write!(f, "instrument_category")
            }
            InsertableInstrumentModelCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableInstrumentModelCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableInstrumentModelCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableInstrumentModelCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::instrument_model_categories::instrument_model_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrumentModelCategory {
    instrument_model_id: i32,
    instrument_category: instrument_categories::InstrumentCategory,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableInstrumentModelCategory {
    #[cfg(feature = "postgres")]
    pub async fn instrument_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_models::instrument_models::dsl::id
                    .eq(&self.instrument_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
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
pub struct InsertableInstrumentModelCategoryBuilder {
    instrument_model_id: Option<i32>,
    instrument_category: Option<instrument_categories::InstrumentCategory>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableInstrumentModelCategoryBuilder {
    fn default() -> Self {
        Self {
            instrument_model_id: None,
            instrument_category: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableInstrumentModelCategoryBuilder {
    pub fn instrument_model_id<P>(
        mut self,
        instrument_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let instrument_model_id =
            instrument_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableInstrumentModelCategoryAttributes::InstrumentModelId)
            })?;
        self.instrument_model_id = Some(instrument_model_id);
        Ok(self)
    }
    pub fn instrument_category<P>(
        mut self,
        instrument_category: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<instrument_categories::InstrumentCategory>,
        <P as TryInto<instrument_categories::InstrumentCategory>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let instrument_category = instrument_category.try_into().map_err(
            |err: <P as TryInto<instrument_categories::InstrumentCategory>>::Error| {
                Into::into(err)
                    .rename_field(InsertableInstrumentModelCategoryAttributes::InstrumentCategory)
            },
        )?;
        self.instrument_category = Some(instrument_category);
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
            Into::into(err).rename_field(InsertableInstrumentModelCategoryAttributes::CreatedBy)
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
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentModelCategoryAttributes::CreatedAt)
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
            Into::into(err).rename_field(InsertableInstrumentModelCategoryAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentModelCategoryAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableInstrumentModelCategoryBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableInstrumentModelCategoryAttributes>;
    type Object = InsertableInstrumentModelCategory;
    type Attribute = InsertableInstrumentModelCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            instrument_model_id: self.instrument_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::InstrumentModelId,
                ),
            )?,
            instrument_category: self.instrument_category.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::InstrumentCategory,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentModelCategoryAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableInstrumentModelCategory> for InsertableInstrumentModelCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableInstrumentModelCategory,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .instrument_model_id(insertable_variant.instrument_model_id)?
            .instrument_category(insertable_variant.instrument_category)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
