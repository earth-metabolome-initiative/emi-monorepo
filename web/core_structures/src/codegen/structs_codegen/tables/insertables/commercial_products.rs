#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductExtensionAttributes {
    Trackable(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
}
impl core::fmt::Display for InsertableCommercialProductExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Trackable(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductAttributes {
    Extension(InsertableCommercialProductExtensionAttributes),
    Id,
    DeprecationDate,
    BrandId,
}
impl core::fmt::Display for InsertableCommercialProductAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::DeprecationDate => write!(f, "deprecation_date"),
            Self::BrandId => write!(f, "brand_id"),
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
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) brand_id: i32,
}
impl InsertableCommercialProduct {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn brand<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::brands::Brand,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::brands::Brand: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::brands::Brand,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::brands::Brand::table(),
                self.brand_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProductBuilder<
    Trackable = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
> {
    pub(crate) deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) brand_id: Option<i32>,
    pub(crate) id: Trackable,
}
impl<Trackable> web_common_traits::database::ExtendableBuilder
for InsertableCommercialProductBuilder<Trackable>
where
    Trackable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    >,
{
    type Attributes = InsertableCommercialProductAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                ))
            })?;
        if let Some(deprecation_date) = other.deprecation_date {
            self = self.deprecation_date(Some(deprecation_date))?;
        }
        if let Some(brand_id) = other.brand_id {
            self = self.brand(brand_id)?;
        }
        Ok(self)
    }
}
impl<Trackable> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialProductBuilder<Trackable>
where
    Trackable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        Trackable,
    >
{
    /// Sets the value of the `commercial_products.brand_id` column from table
    /// `commercial_products`.
    pub fn brand(
        mut self,
        brand_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    {
        self.brand_id = Some(brand_id);
        Ok(self)
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        Trackable,
    >
{
    /// Sets the value of the `commercial_products.deprecation_date` column from
    /// table `commercial_products`.
    pub fn deprecation_date<P>(
        mut self,
        deprecation_date: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let deprecation_date = deprecation_date.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error| {
                Into::into(err).rename_field(InsertableCommercialProductAttributes::DeprecationDate)
            },
        )?;
        self.deprecation_date = deprecation_date;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `commercial_products`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `commercial_products`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    {
        self.id = self.id.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `commercial_products`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.id` column from table
    /// `commercial_products`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `commercial_products`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `commercial_products`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    {
        self.id = self.id.parent(parent_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `commercial_products`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    {
        self.id = self.id.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `commercial_products`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `commercial_products`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    {
        self.id = self.id.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialProductAttributes::Extension(
                    InsertableCommercialProductExtensionAttributes::Trackable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<Trackable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCommercialProductBuilder<Trackable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            Error = web_common_traits::database::InsertError<InsertableCommercialProductAttributes>,
        >,
    Trackable: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableCommercialProductAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.brand_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
