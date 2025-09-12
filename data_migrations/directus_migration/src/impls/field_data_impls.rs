//! Submodule providing helpers methods to work with the FieldDAtum objects

use std::str::FromStr;

use core_structures::{
    Organism, SampleModel, SampleSource, SampleSourceModel, User,
    tables::insertables::AssetSettable,
};
use diesel::{OptionalExtension, PgConnection};
use init_migration::asset_models::organisms::{organism_model, organism_sample_model};
use web_common_traits::{
    database::{InsertableVariant, Read},
    prelude::Insertable,
};

use crate::sample_source_kind::SampleSourceKind;
use crate::structs::FieldDatumWrapper;

mod field_data_author;
mod should_skip;

impl FieldDatumWrapper {
    /// Returns the sample source UUID if the field datum has a sample source.
    /// The sample source is stored in the `comment_eco` field as a string
    /// representing a UUID.
    fn sample_source_uuid(&self) -> Option<rosetta_uuid::Uuid> {
        rosetta_uuid::Uuid::from_str(self.as_ref().comment_eco.as_ref()?).ok()
    }
    /// Returns a bool indicating whether the field datum has a sample source.
    /// If the fiels `comment_eco` is Some and has vor value a UUID, it is
    /// considered to have a sample source.
    pub fn has_sample_source(&self) -> bool {
        self.sample_source_uuid().is_some()
    }
    ///
    pub fn sample_source(
        &self,
        user: &User,
        portal: &mut PgConnection,
    ) -> Result<Option<SampleSource>, anyhow::Error> {
        let Some(uuid) = self.sample_source_uuid() else {
            return Ok(None);
        };
        if let Some(sample_source) = SampleSource::read(uuid, portal).optional()? {
            return Ok(Some(sample_source));
        }
        let sample_source_model = self.sample_source_model(user, portal)?;
        match self.sample_source_kind() {
            SampleSourceKind::Organism => {
                let organism = Organism::new()
                    .id(uuid)?
                    .model(&sample_source_model)?
                    .insert(user.id, portal)?;
                Ok(Some(organism.id(portal)?))
            }
            SampleSourceKind::Soil => todo!("implement soil sample source model"),
        }
    }
    /// Return the sample source kind of the sample.
    ///
    /// # Implementation details
    ///
    /// * If the `sample_name` field contains the string "Mixte", the sample
    ///   source kind is `Soil`.
    /// * Otherwise, the sample source kind is `Organism`.
    pub fn sample_source_kind(&self) -> SampleSourceKind {
        let Some(ref sample_name) = self.as_ref().sample_name else {
            return SampleSourceKind::Organism;
        };

        if sample_name.contains("Mixte") {
            SampleSourceKind::Soil
        } else {
            SampleSourceKind::Organism
        }
    }

    /// Returns, or insert in the portal database, the sample source model of
    /// the sample-
    ///
    /// # Arguments
    ///
    /// * `user` - The user who is creating the sample source model.
    /// * `portal` - The portal database connection.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn sample_source_model(
        &self,
        user: &User,
        portal: &mut PgConnection,
    ) -> Result<SampleSourceModel, anyhow::Error> {
        Ok(match self.sample_source_kind() {
            SampleSourceKind::Organism => organism_model(user, portal)?.id(portal)?,
            SampleSourceKind::Soil => todo!("implement soil sample source model"),
        })
    }

    /// Returns the sample model of the sample.
    ///
    /// # Arguments
    ///
    /// * `user` - The user who is creating the sample model.
    /// * `portal` - The portal database connection.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn sample_model(
        &self,
        user: &User,
        portal: &mut PgConnection,
    ) -> Result<SampleModel, anyhow::Error> {
        Ok(match self.sample_source_kind() {
            SampleSourceKind::Organism => organism_sample_model(user, portal)?,
            SampleSourceKind::Soil => todo!("implement soil sample source model"),
        })
    }

    /// Returns the sample id after validation.
    /// 
    pub fn sample_id(&self) -> Result<String, anyhow::Error> {





        Ok(self.as_ref()
            .sample_id
            .clone())
    }
}
