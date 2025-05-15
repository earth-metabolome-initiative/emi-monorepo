#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductLotAttributes {
    Lot,
    ProductModelId,
}
impl core::fmt::Display for InsertableCommercialProductLotAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCommercialProductLotAttributes::Lot => write!(f, "lot"),
            InsertableCommercialProductLotAttributes::ProductModelId => {
                write!(f, "product_model_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProductLot {
    lot: String,
    product_model_id: i32,
}
impl InsertableCommercialProductLot {
    #[cfg(feature = "postgres")]
    pub async fn product_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::id
                    .eq(&self.product_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            >(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableCommercialProductLotBuilder {
    lot: Option<String>,
    product_model_id: Option<i32>,
}
impl InsertableCommercialProductLotBuilder {
    pub fn lot<P>(
        mut self,
        lot: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let lot = lot.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCommercialProductLotAttributes::Lot)
        })?;
        self.lot = Some(lot);
        Ok(self)
    }
    pub fn product_model_id<P>(
        mut self,
        product_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let product_model_id =
            product_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCommercialProductLotAttributes::ProductModelId)
            })?;
        self.product_model_id = Some(product_model_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCommercialProductLotBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCommercialProductLotAttributes>;
    type Object = InsertableCommercialProductLot;
    type Attribute = InsertableCommercialProductLotAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            lot: self.lot.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCommercialProductLotAttributes::Lot,
            ))?,
            product_model_id: self.product_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialProductLotAttributes::ProductModelId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableCommercialProductLot> for InsertableCommercialProductLotBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCommercialProductLot) -> Result<Self, Self::Error> {
        Self::default()
            .lot(insertable_variant.lot)?
            .product_model_id(insertable_variant.product_model_id)
    }
}
