//! Submodule providing helpers methods to work with the `FieldDAtum` objects

use std::str::FromStr;

use chrono::format;
use core_structures::{
    Organism, SampleModel, SampleSource, SampleSourceModel, Soil, User,
    tables::insertables::AssetSettable,
};
use diesel::{OptionalExtension, PgConnection};
use init_migration::asset_models::{
    organisms::{organism_model, organism_sample_model},
    soils::{soil_model, soil_sample_model},
};
use web_common_traits::{
    database::{DispatchableInsertableVariant, PrimaryKeyLike, Read},
    prelude::Insertable,
};

use crate::{
    errors::SampleIDError, sample_source_kind::SampleSourceKind, structs::FieldDatumWrapper,
};

mod field_data_author;
mod should_skip;
mod template_dispatch;

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
        let pk = match self.sample_source_kind() {
            SampleSourceKind::Organism => {
                let organism = Organism::new()
                    .id(uuid)?
                    .model(sample_source_model)?
                    .created_by(user)?
                    .insert(user.id, portal)?;
                organism.primary_key()
            }
            SampleSourceKind::Soil => {
                let soil = Soil::new()
                    .id(uuid)?
                    .model(sample_source_model)?
                    .created_by(user)?
                    .insert(user.id, portal)?;
                soil.primary_key()
            }
        };
        Ok(Some(SampleSource::read(pk, portal)?))
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
        Ok(SampleSourceModel::read(
            match self.sample_source_kind() {
                SampleSourceKind::Organism => organism_model(user, portal)?.primary_key(),
                SampleSourceKind::Soil => soil_model(user, portal)?.primary_key(),
            },
            portal,
        )?)
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
            SampleSourceKind::Soil => soil_sample_model(user, portal)?,
        })
    }

    /// Returns the sample id after validation.
    pub fn sample_id(&self) -> Result<String, SampleIDError> {
        let sample_id = self.as_ref().sample_id.trim();
        if sample_id.is_empty() {
            return Err(SampleIDError::Empty);
        }

        Ok(sample_id.to_owned())
    }

    /// Returns the geolocation of the observation.
    pub fn geolocation(&self) -> postgis_diesel::types::Point {
        self.as_ref().geometry.expect(&format!("FieldDatum {self:#?} has no geometry"))
    }
}
