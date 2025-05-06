#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductAttributes {
    Name,
    Description,
    PhotographId,
    DeprecationDate,
    BrandId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableCommercialProductAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCommercialProductAttributes::Name => write!(f, "name"),
            InsertableCommercialProductAttributes::Description => {
                write!(f, "description")
            }
            InsertableCommercialProductAttributes::PhotographId => {
                write!(f, "photograph_id")
            }
            InsertableCommercialProductAttributes::DeprecationDate => {
                write!(f, "deprecation_date")
            }
            InsertableCommercialProductAttributes::BrandId => write!(f, "brand_id"),
            InsertableCommercialProductAttributes::CreatedBy => write!(f, "created_by"),
            InsertableCommercialProductAttributes::CreatedAt => write!(f, "created_at"),
            InsertableCommercialProductAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableCommercialProductAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_products::commercial_products
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProduct {
    name: String,
    description: String,
    photograph_id: rosetta_uuid::Uuid,
    deprecation_date: Option<rosetta_timestamp::TimestampUTC>,
    brand_id: i32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableCommercialProduct {
    #[cfg(feature = "postgres")]
    pub async fn photograph(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::photographs::Photograph::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photographs::photographs::dsl::id
                    .eq(&self.photograph_id),
            )
            .first::<crate::codegen::structs_codegen::tables::photographs::Photograph>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn brand(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::brands::Brand, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::brands::Brand::table()
            .filter(
                crate::codegen::diesel_codegen::tables::brands::brands::dsl::id.eq(&self.brand_id),
            )
            .first::<crate::codegen::structs_codegen::tables::brands::Brand>(conn)
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
pub struct InsertableCommercialProductBuilder {
    name: Option<String>,
    description: Option<String>,
    photograph_id: Option<rosetta_uuid::Uuid>,
    deprecation_date: Option<rosetta_timestamp::TimestampUTC>,
    brand_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableCommercialProductBuilder {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            photograph_id: None,
            deprecation_date: None,
            brand_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableCommercialProductBuilder {
    pub fn name<P: Into<String>>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let name = name.into();
        pgrx_validation::must_not_be_empty(name.as_ref())
            .map_err(|e| e.rename_field(InsertableCommercialProductAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P: Into<String>>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let description = description.into();
        pgrx_validation::must_not_be_empty(description.as_ref())
            .map_err(|e| e.rename_field(InsertableCommercialProductAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn photograph_id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let photograph_id = photograph_id.into();
        self.photograph_id = Some(photograph_id);
        Ok(self)
    }
    pub fn deprecation_date<P: Into<Option<rosetta_timestamp::TimestampUTC>>>(
        mut self,
        deprecation_date: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let deprecation_date = deprecation_date.into();
        self.deprecation_date = deprecation_date;
        Ok(self)
    }
    pub fn brand_id<P: Into<i32>>(
        mut self,
        brand_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let brand_id = brand_id.into();
        self.brand_id = Some(brand_id);
        Ok(self)
    }
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P: Into<i32>>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_by = updated_by.into();
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCommercialProductBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCommercialProductAttributes>;
    type Object = InsertableCommercialProduct;
    type Attribute = InsertableCommercialProductAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCommercialProductAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::Description,
                ),
            )?,
            photograph_id: self.photograph_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::PhotographId,
                ),
            )?,
            deprecation_date: self.deprecation_date,
            brand_id: self.brand_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::BrandId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableCommercialProduct> for InsertableCommercialProductBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCommercialProduct) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .photograph_id(insertable_variant.photograph_id)?
            .deprecation_date(insertable_variant.deprecation_date)?
            .brand_id(insertable_variant.brand_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
