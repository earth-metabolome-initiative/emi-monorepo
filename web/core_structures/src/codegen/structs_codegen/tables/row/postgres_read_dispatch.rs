#[cfg(feature = "postgres")]
impl web_common_traits::prelude::ReadDispatch<diesel::PgConnection> for super::Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn read(
        primary_key: Self::PrimaryKey,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        match primary_key {
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Address(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::addresses::Address as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingInstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::brands::Brand as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::City(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::cities::City as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::colors::Color as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialReagent(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::container_models::ContainerModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::containers::Container as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::countries::Country as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Document(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::documents::Document as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::EmailProvider(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instrument_states::InstrumentState as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Instrument(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::instruments::Instrument as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Material(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::materials::Material as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::MixCountableProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::MixSolidProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::NextProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismObservation(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismTaxon(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organisms::Organism as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organization(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::organizations::Organization as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::procedures::Procedure as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::processables::Processable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcessingProcedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::project_states::ProjectState as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::projects::Project as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Rank(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::ranks::Rank as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Reagent(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::reagents::Reagent as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Role(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::roles::Role as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::rooms::Room as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sample_states::SampleState as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingProcedure(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ShakingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SharedProcedureModelTrackable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpatialRefSy(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Spectrum(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spectra::Spectrum as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::taxa::Taxon as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_members::TeamMember as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_projects::TeamProject as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::team_states::TeamState as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::teams::Team as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TemporaryUser(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableLocation(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::trackables::Trackable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::units::Unit as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::user_emails::UserEmail as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::users::User as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricProcessable(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingInstrumentModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedureModel(
                primary_key,
            ) => {
                Ok(
                    <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as web_common_traits::database::Read<
                        diesel::PgConnection,
                    >>::read(primary_key, conn)?
                        .map(super::Row::from),
                )
            }
        }
    }
}
