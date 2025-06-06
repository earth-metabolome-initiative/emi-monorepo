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
    pub fn spectra_collection<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::table(),
                self.spectra_collection_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableSpectrumBuilder {
    id: Option<i32>,
    spectra_collection_id: Option<i32>,
}
impl InsertableSpectrumBuilder {
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectrumAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableSpectrumAttributes>>
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
impl TryFrom<InsertableSpectrumBuilder> for InsertableSpectrum {
    type Error = common_traits::prelude::BuilderError<InsertableSpectrumAttributes>;
    fn try_from(builder: InsertableSpectrumBuilder) -> Result<InsertableSpectrum, Self::Error> {
        Ok(Self {
            id: builder.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableSpectrumAttributes::Id,
            ))?,
            spectra_collection_id: builder.spectra_collection_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSpectrumAttributes::SpectraCollectionId,
                ),
            )?,
        })
    }
}
