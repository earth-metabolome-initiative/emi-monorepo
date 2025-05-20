#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialReagentAttributes {
    Id,
    CommercialProductLotId,
}
impl core::fmt::Display for InsertableCommercialReagentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCommercialReagentAttributes::Id => write!(f, "id"),
            InsertableCommercialReagentAttributes::CommercialProductLotId => {
                write!(f, "commercial_product_lot_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialReagent {
    id: rosetta_uuid::Uuid,
    commercial_product_lot_id: i32,
}
impl InsertableCommercialReagent {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::processables::Processable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::processables::processables::dsl::id
                    .eq(&self.id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn commercial_product_lot(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::dsl::id
                    .eq(&self.commercial_product_lot_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
            >(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableCommercialReagentBuilder {
    id: Option<rosetta_uuid::Uuid>,
    commercial_product_lot_id: Option<i32>,
}
impl InsertableCommercialReagentBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableCommercialReagentAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn commercial_product_lot_id<P>(
        mut self,
        commercial_product_lot_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let commercial_product_lot_id =
            commercial_product_lot_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCommercialReagentAttributes::CommercialProductLotId)
            })?;
        self.commercial_product_lot_id = Some(commercial_product_lot_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCommercialReagentBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>;
    type Object = InsertableCommercialReagent;
    type Attribute = InsertableCommercialReagentAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCommercialReagentAttributes::Id,
            ))?,
            commercial_product_lot_id: self.commercial_product_lot_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCommercialReagentAttributes::CommercialProductLotId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableCommercialReagent> for InsertableCommercialReagentBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCommercialReagent) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .commercial_product_lot_id(insertable_variant.commercial_product_lot_id)
    }
}
