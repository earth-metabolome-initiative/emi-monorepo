#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableContainerModelAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes),
    Liters,
    ContainerCategory,
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    > for InsertableContainerModelAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableContainerModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableContainerModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableContainerModelAttributes::Liters => write!(f, "liters"),
            InsertableContainerModelAttributes::ContainerCategory => {
                write!(f, "container_category")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::container_models::container_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerModel {
    id: ::rosetta_uuid::Uuid,
    liters: f32,
    container_category: ::container_categories::ContainerCategory,
}
impl InsertableContainerModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableContainerModelBuilder {
    pub(crate) id:
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder,
    pub(crate) liters: Option<f32>,
    pub(crate) container_category: Option<::container_categories::ContainerCategory>,
}
impl InsertableContainerModelBuilder {
    pub fn liters<P>(
        mut self,
        liters: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let liters = liters.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableContainerModelAttributes::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| e.rename_field(InsertableContainerModelAttributes::Liters))?;
        self.liters = Some(liters);
        Ok(self)
    }
    pub fn container_category<P>(
        mut self,
        container_category: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<::container_categories::ContainerCategory>,
        <P as TryInto<::container_categories::ContainerCategory>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let container_category = container_category.try_into().map_err(
            |err: <P as TryInto<::container_categories::ContainerCategory>>::Error| {
                Into::into(err).rename_field(InsertableContainerModelAttributes::ContainerCategory)
            },
        )?;
        self.container_category = Some(container_category);
        Ok(self)
    }
    pub fn deprecation_date<P>(
        mut self,
        deprecation_date: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id =
            self.id.deprecation_date(deprecation_date).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn brand_id<P>(
        mut self,
        brand_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.brand_id(brand_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.parent_id(parent_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableContainerModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableContainerModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableContainerModel,
        web_common_traits::database::InsertError<InsertableContainerModelAttributes>,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let liters = self.liters.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableContainerModelAttributes::Liters,
        ))?;
        let container_category = self.container_category.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableContainerModelAttributes::ContainerCategory,
            ),
        )?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableContainerModel { id, liters, container_category })
    }
}
