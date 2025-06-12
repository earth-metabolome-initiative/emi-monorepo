impl<C> web_common_traits::prelude::ReadDispatch<C> for super::Row
where
    crate::codegen::structs_codegen::tables::addresses::Address: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::brands::Brand: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::cities::City: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::colors::Color: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_models::ContainerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::containers::Container: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::countries::Country: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::documents::Document: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::email_providers::EmailProvider: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::instruments::Instrument: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::login_providers::LoginProvider: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::materials::Material: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organisms::Organism: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organizations::Organization: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::processables::Processable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::project_states::ProjectState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ranks::Rank: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::reagents::Reagent: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::roles::Role: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::rooms::Room: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_states::SampleState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra::Spectrum: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::taxa::Taxon: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_members::TeamMember: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_projects::TeamProject: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_states::TeamState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::trackables::Trackable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::units::Unit: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_emails::UserEmail: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure: web_common_traits::database::Read<
        C,
    >,
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn read(
        primary_key: Self::PrimaryKey,
        conn: &mut C,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use web_common_traits::database::Read;
        Ok(
            match primary_key {
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Address(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::addresses::Address::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingInstrumentModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::brands::Brand::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::City(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::cities::City::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::colors::Color::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialReagent(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::containers::Container::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::countries::Country::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Document(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::documents::Document::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::EmailProvider(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::email_providers::EmailProvider::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Instrument(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::instruments::Instrument::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::login_providers::LoginProvider::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Material(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::materials::Material::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::MixCountableProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::MixSolidProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::MountTipProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::NextProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismObservation(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismTaxon(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organisms::Organism::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organization(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organizations::Organization::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModelTrackable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTrackable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::processables::Processable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::project_states::ProjectState::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::projects::Project::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Rank(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ranks::Rank::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Reagent(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::reagents::Reagent::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Role(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::roles::Role::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::rooms::Room::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sample_states::SampleState::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SamplingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ShakingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SharedProcedureModelTrackable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpatialRefSy(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Spectrum(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spectra::Spectrum::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::taxa::Taxon::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_members::TeamMember::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_projects::TeamProject::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_states::TeamState::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::teams::Team::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TemporaryUser(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TrackableLocation(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::trackables::Trackable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::units::Unit::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::user_emails::UserEmail::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::users::User::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricProcessable(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingInstrumentModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedureModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .map(super::Row::from)
                }
            },
        )
    }
}
