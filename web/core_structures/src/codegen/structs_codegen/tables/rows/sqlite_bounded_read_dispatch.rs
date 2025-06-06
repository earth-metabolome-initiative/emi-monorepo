#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::BoundedReadDispatch<diesel::SqliteConnection> for super::Rows {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn bounded_read(
        table_name: Self::TableName,
        offset: u16,
        limit: u16,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Self, diesel::result::Error> {
        match table_name {
            crate::codegen::tables::table_names::TableName::Address => {
                <crate::codegen::structs_codegen::tables::addresses::Address as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingInstrumentModel => {
                <crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedureModel => {
                <crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedure => {
                <crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedureModel => {
                <crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Brand => {
                <crate::codegen::structs_codegen::tables::brands::Brand as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedureModel => {
                <crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::City => {
                <crate::codegen::structs_codegen::tables::cities::City as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Color => {
                <crate::codegen::structs_codegen::tables::colors::Color as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProductLot => {
                <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProduct => {
                <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialReagent => {
                <crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerModel => {
                <crate::codegen::structs_codegen::tables::container_models::ContainerModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Container => {
                <crate::codegen::structs_codegen::tables::containers::Container as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Country => {
                <crate::codegen::structs_codegen::tables::countries::Country as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedureModel => {
                <crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Document => {
                <crate::codegen::structs_codegen::tables::documents::Document as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::EmailProvider => {
                <crate::codegen::structs_codegen::tables::email_providers::EmailProvider as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedureModel => {
                <crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedure => {
                <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedureModel => {
                <crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentModel => {
                <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentState => {
                <crate::codegen::structs_codegen::tables::instrument_states::InstrumentState as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Instrument => {
                <crate::codegen::structs_codegen::tables::instruments::Instrument as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::LoginProvider => {
                <crate::codegen::structs_codegen::tables::login_providers::LoginProvider as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Material => {
                <crate::codegen::structs_codegen::tables::materials::Material as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::MixCountableProcedureModel => {
                <crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::MixSolidProcedureModel => {
                <crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::NextProcedureModel => {
                <crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ObservationSubject => {
                <crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismObservation => {
                <crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismTaxon => {
                <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organism => {
                <crate::codegen::structs_codegen::tables::organisms::Organism as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organization => {
                <crate::codegen::structs_codegen::tables::organizations::Organization as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingModel => {
                <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedureModel => {
                <crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ParentProcedureModel => {
                <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PermanenceCategory => {
                <crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedureModel => {
                <crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelTrackable => {
                <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModel => {
                <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTrackable => {
                <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Procedure => {
                <crate::codegen::structs_codegen::tables::procedures::Procedure as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Processable => {
                <crate::codegen::structs_codegen::tables::processables::Processable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcessingProcedure => {
                <crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectState => {
                <crate::codegen::structs_codegen::tables::project_states::ProjectState as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Project => {
                <crate::codegen::structs_codegen::tables::projects::Project as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Rank => {
                <crate::codegen::structs_codegen::tables::ranks::Rank as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Reagent => {
                <crate::codegen::structs_codegen::tables::reagents::Reagent as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Role => {
                <crate::codegen::structs_codegen::tables::roles::Role as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Room => {
                <crate::codegen::structs_codegen::tables::rooms::Room as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleState => {
                <crate::codegen::structs_codegen::tables::sample_states::SampleState as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SamplingProcedureModel => {
                <crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SamplingProcedure => {
                <crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ShakingProcedureModel => {
                <crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable => {
                <crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpatialRefSy => {
                <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Spectrum => {
                <crate::codegen::structs_codegen::tables::spectra::Spectrum as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpectraCollection => {
                <crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Taxon => {
                <crate::codegen::structs_codegen::tables::taxa::Taxon as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamMember => {
                <crate::codegen::structs_codegen::tables::team_members::TeamMember as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamProject => {
                <crate::codegen::structs_codegen::tables::team_projects::TeamProject as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamState => {
                <crate::codegen::structs_codegen::tables::team_states::TeamState as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Team => {
                <crate::codegen::structs_codegen::tables::teams::Team as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TemporaryUser => {
                <crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TrackableLocation => {
                <crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Trackable => {
                <crate::codegen::structs_codegen::tables::trackables::Trackable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Unit => {
                <crate::codegen::structs_codegen::tables::units::Unit as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserEmail => {
                <crate::codegen::structs_codegen::tables::user_emails::UserEmail as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserOrganization => {
                <crate::codegen::structs_codegen::tables::user_organizations::UserOrganization as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::User => {
                <crate::codegen::structs_codegen::tables::users::User as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricProcessable => {
                <crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingInstrumentModel => {
                <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedureModel => {
                <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as web_common_traits::database::BoundedRead<
                    diesel::SqliteConnection,
                >>::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
        }
    }
}
