#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSpectrumAttributes {
    Id,
    SpectraCollectionId,
}
impl core::fmt::Display for InsertableSpectrumAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSpectrumAttributes::Id => write!(f, "id"),
            InsertableSpectrumAttributes::SpectraCollectionId => {
                write!(f, "spectra_collection_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::spectra::spectra)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectrum {
    id: i32,
    spectra_collection_id: i32,
}
impl InsertableSpectrum {
    #[cfg(feature = "postgres")]
    pub async fn spectra_collection(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::table()
            .filter(
                crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections::dsl::id
                    .eq(&self.spectra_collection_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
            >(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableSpectrumBuilder {
    id: Option<i32>,
    spectra_collection_id: Option<i32>,
}
impl InsertableSpectrumBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableSpectrumAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn spectra_collection_id<P>(
        mut self,
        spectra_collection_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let spectra_collection_id =
            spectra_collection_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(InsertableSpectrumAttributes::SpectraCollectionId)
            })?;
        self.spectra_collection_id = Some(spectra_collection_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableSpectrumBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSpectrumAttributes>;
    type Object = InsertableSpectrum;
    type Attribute = InsertableSpectrumAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableSpectrumAttributes::Id,
            ))?,
            spectra_collection_id: self.spectra_collection_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectrumAttributes::SpectraCollectionId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableSpectrum> for InsertableSpectrumBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSpectrum) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .spectra_collection_id(insertable_variant.spectra_collection_id)
    }
}
