mod addresses;
mod aliquoting_procedure_models;
mod ball_mill_machine_models;
mod ball_mill_procedure_models;
mod binary_question_procedure_models;
mod bounded_read_dispatch;
mod brands;
mod camera_models;
mod capping_procedure_models;
mod centrifuge_models;
mod centrifuge_procedure_models;
mod cities;
mod colors;
mod commercial_product_lots;
mod commercial_products;
mod commercial_reagents;
mod compatibility_rules;
mod container_models;
mod containers;
mod countries;
mod disposal_procedure_models;
mod documents;
mod email_providers;
mod fractioning_procedure_models;
mod freeze_drier_models;
mod freeze_drying_procedure_models;
mod freezer_models;
mod freezing_procedure_models;
mod geolocation_procedure_models;
mod instrument_models;
mod instrument_states;
mod into_iter;
mod len;
mod login_providers;
mod materials;
mod mix_countable_procedure_models;
mod mix_solid_procedure_models;
mod mount_tip_procedure_models;
mod next_procedure_models;
mod observation_subjects;
mod organism_taxa;
mod organisms;
mod organizations;
mod packaging_procedure_models;
mod parent_procedure_models;
mod permanence_categories;
mod photograph_procedure_models;
mod pipette_models;
mod pipette_tip_models;
mod positioning_device_models;
mod pouring_procedure_models;
mod procedure_model_trackables;
mod procedure_models;
mod procedure_trackables;
mod procedures;
mod processables;
mod project_states;
mod projects;
mod ranks;
mod reagents;
mod roles;
mod rooms;
mod sample_states;
mod shared_procedure_model_trackables;
mod spatial_ref_sys;
mod spectra;
mod spectra_collections;
mod storage_procedure_models;
mod supernatant_procedure_models;
mod tabular;
mod taxa;
mod team_members;
mod team_projects;
mod team_states;
mod teams;
mod temporary_user;
mod trackable_locations;
mod trackables;
mod units;
mod user_emails;
mod user_organizations;
mod users;
mod volumetric_container_models;
mod volumetric_processables;
mod weighing_instrument_models;
mod weighing_procedure_models;
mod weighing_procedures;
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rows {
    Address(Vec<crate::codegen::structs_codegen::tables::addresses::Address>),
    AliquotingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
        >,
    ),
    BallMillMachineModel(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    ),
    BallMillProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
        >,
    ),
    BinaryQuestionProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel,
        >,
    ),
    Brand(Vec<crate::codegen::structs_codegen::tables::brands::Brand>),
    CameraModel(
        Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>,
    ),
    CappingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
        >,
    ),
    CentrifugeModel(
        Vec<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>,
    ),
    CentrifugeProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
        >,
    ),
    City(Vec<crate::codegen::structs_codegen::tables::cities::City>),
    Color(Vec<crate::codegen::structs_codegen::tables::colors::Color>),
    CommercialProductLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        >,
    ),
    CommercialProduct(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    ),
    CommercialReagent(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
        >,
    ),
    CompatibilityRule(
        Vec<
            crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
        >,
    ),
    ContainerModel(
        Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ),
    Container(Vec<crate::codegen::structs_codegen::tables::containers::Container>),
    Country(Vec<crate::codegen::structs_codegen::tables::countries::Country>),
    DisposalProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
        >,
    ),
    Document(Vec<crate::codegen::structs_codegen::tables::documents::Document>),
    EmailProvider(
        Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ),
    FractioningProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
        >,
    ),
    FreezeDrierModel(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
        >,
    ),
    FreezeDryingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
        >,
    ),
    FreezerModel(
        Vec<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>,
    ),
    FreezingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
        >,
    ),
    GeolocationProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel,
        >,
    ),
    InstrumentModel(
        Vec<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>,
    ),
    InstrumentState(
        Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ),
    LoginProvider(
        Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ),
    Material(Vec<crate::codegen::structs_codegen::tables::materials::Material>),
    MixCountableProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
        >,
    ),
    MixSolidProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
        >,
    ),
    MountTipProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
        >,
    ),
    NextProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel,
        >,
    ),
    ObservationSubject(
        Vec<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ),
    OrganismTaxon(
        Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ),
    Organism(Vec<crate::codegen::structs_codegen::tables::organisms::Organism>),
    Organization(
        Vec<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ),
    PackagingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
        >,
    ),
    ParentProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        >,
    ),
    PermanenceCategory(
        Vec<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ),
    PhotographProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
        >,
    ),
    PipetteModel(
        Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
    ),
    PipetteTipModel(
        Vec<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>,
    ),
    PositioningDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        >,
    ),
    PouringProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
        >,
    ),
    ProcedureModelTrackable(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    ),
    ProcedureModel(
        Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    ),
    ProcedureTrackable(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
        >,
    ),
    Procedure(Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>),
    Processable(Vec<crate::codegen::structs_codegen::tables::processables::Processable>),
    ProjectState(
        Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ),
    Project(Vec<crate::codegen::structs_codegen::tables::projects::Project>),
    Rank(Vec<crate::codegen::structs_codegen::tables::ranks::Rank>),
    Reagent(Vec<crate::codegen::structs_codegen::tables::reagents::Reagent>),
    Role(Vec<crate::codegen::structs_codegen::tables::roles::Role>),
    Room(Vec<crate::codegen::structs_codegen::tables::rooms::Room>),
    SampleState(
        Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ),
    SharedProcedureModelTrackable(
        Vec<
            crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
        >,
    ),
    SpatialRefSy(
        Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ),
    Spectrum(Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>),
    SpectraCollection(
        Vec<
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    ),
    StorageProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        >,
    ),
    SupernatantProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
        >,
    ),
    Taxon(Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>),
    TeamMember(Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>),
    TeamProject(
        Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ),
    TeamState(Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>),
    Team(Vec<crate::codegen::structs_codegen::tables::teams::Team>),
    TemporaryUser(
        Vec<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>,
    ),
    TrackableLocation(
        Vec<
            crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
        >,
    ),
    Trackable(Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>),
    Unit(Vec<crate::codegen::structs_codegen::tables::units::Unit>),
    UserEmail(Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>),
    UserOrganization(
        Vec<
            crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        >,
    ),
    User(Vec<crate::codegen::structs_codegen::tables::users::User>),
    VolumetricContainerModel(
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    ),
    VolumetricProcessable(
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
        >,
    ),
    WeighingInstrumentModel(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ),
    WeighingProcedureModel(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
        >,
    ),
    WeighingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
        >,
    ),
}
impl Rows {
    #[cfg(feature = "sqlite")]
    /// Executes the upsert operation and returns the result.
    pub fn sqlite_upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Rows, diesel::result::Error> {
        use web_common_traits::database::Upsertable;
        Ok(match self {
            Rows::Address(addresses) => {
                addresses
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BinaryQuestionProcedureModel(binary_question_procedure_models) => {
                binary_question_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Brand(brands) => {
                brands
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CameraModel(camera_models) => {
                camera_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CappingProcedureModel(capping_procedure_models) => {
                capping_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeModel(centrifuge_models) => {
                centrifuge_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeProcedureModel(centrifuge_procedure_models) => {
                centrifuge_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::City(cities) => {
                cities
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Color(colors) => {
                colors
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialProduct(commercial_products) => {
                commercial_products
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialReagent(commercial_reagents) => {
                commercial_reagents
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CompatibilityRule(compatibility_rules) => {
                compatibility_rules
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ContainerModel(container_models) => {
                container_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Container(containers) => {
                containers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Country(countries) => {
                countries
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DisposalProcedureModel(disposal_procedure_models) => {
                disposal_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Document(documents) => {
                documents
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::EmailProvider(email_providers) => {
                email_providers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FractioningProcedureModel(fractioning_procedure_models) => {
                fractioning_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDrierModel(freeze_drier_models) => {
                freeze_drier_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezerModel(freezer_models) => {
                freezer_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezingProcedureModel(freezing_procedure_models) => {
                freezing_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::GeolocationProcedureModel(geolocation_procedure_models) => {
                geolocation_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentModel(instrument_models) => {
                instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentState(instrument_states) => {
                instrument_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::LoginProvider(login_providers) => {
                login_providers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Material(materials) => {
                materials
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::MountTipProcedureModel(mount_tip_procedure_models) => {
                mount_tip_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::NextProcedureModel(next_procedure_models) => {
                next_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ObservationSubject(observation_subjects) => {
                observation_subjects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::OrganismTaxon(organism_taxa) => {
                organism_taxa
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Organism(organisms) => {
                organisms
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Organization(organizations) => {
                organizations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PackagingProcedureModel(packaging_procedure_models) => {
                packaging_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ParentProcedureModel(parent_procedure_models) => {
                parent_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PermanenceCategory(permanence_categories) => {
                permanence_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PhotographProcedureModel(photograph_procedure_models) => {
                photograph_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PipetteModel(pipette_models) => {
                pipette_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PipetteTipModel(pipette_tip_models) => {
                pipette_tip_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PouringProcedureModel(pouring_procedure_models) => {
                pouring_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModelTrackable(procedure_model_trackables) => {
                procedure_model_trackables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModel(procedure_models) => {
                procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureTrackable(procedure_trackables) => {
                procedure_trackables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Procedure(procedures) => {
                procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Processable(processables) => {
                processables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProjectState(project_states) => {
                project_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Project(projects) => {
                projects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Rank(ranks) => {
                ranks
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Reagent(reagents) => {
                reagents
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Role(roles) => {
                roles
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Room(rooms) => {
                rooms
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SampleState(sample_states) => {
                sample_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SpatialRefSy(spatial_ref_sys) => {
                spatial_ref_sys
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Spectrum(spectra) => {
                spectra
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SpectraCollection(spectra_collections) => {
                spectra_collections
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StorageProcedureModel(storage_procedure_models) => {
                storage_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SupernatantProcedureModel(supernatant_procedure_models) => {
                supernatant_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Taxon(taxa) => {
                taxa.iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamMember(team_members) => {
                team_members
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamProject(team_projects) => {
                team_projects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamState(team_states) => {
                team_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Team(teams) => {
                teams
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TemporaryUser(temporary_user) => {
                temporary_user
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TrackableLocation(trackable_locations) => {
                trackable_locations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Trackable(trackables) => {
                trackables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Unit(units) => {
                units
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::UserEmail(user_emails) => {
                user_emails
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::UserOrganization(user_organizations) => {
                user_organizations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::User(users) => {
                users
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingProcedure(weighing_procedures) => {
                weighing_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
        })
    }
}
impl web_common_traits::prelude::Rows for Rows {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_keys(&self) -> Vec<Self::PrimaryKey> {
        match self {
            Rows::Address(addresses) => addresses.primary_keys(),
            Rows::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models.primary_keys()
            }
            Rows::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.primary_keys()
            }
            Rows::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models.primary_keys()
            }
            Rows::BinaryQuestionProcedureModel(binary_question_procedure_models) => {
                binary_question_procedure_models.primary_keys()
            }
            Rows::Brand(brands) => brands.primary_keys(),
            Rows::CameraModel(camera_models) => camera_models.primary_keys(),
            Rows::CappingProcedureModel(capping_procedure_models) => {
                capping_procedure_models.primary_keys()
            }
            Rows::CentrifugeModel(centrifuge_models) => centrifuge_models.primary_keys(),
            Rows::CentrifugeProcedureModel(centrifuge_procedure_models) => {
                centrifuge_procedure_models.primary_keys()
            }
            Rows::City(cities) => cities.primary_keys(),
            Rows::Color(colors) => colors.primary_keys(),
            Rows::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_keys()
            }
            Rows::CommercialProduct(commercial_products) => commercial_products.primary_keys(),
            Rows::CommercialReagent(commercial_reagents) => commercial_reagents.primary_keys(),
            Rows::CompatibilityRule(compatibility_rules) => compatibility_rules.primary_keys(),
            Rows::ContainerModel(container_models) => container_models.primary_keys(),
            Rows::Container(containers) => containers.primary_keys(),
            Rows::Country(countries) => countries.primary_keys(),
            Rows::DisposalProcedureModel(disposal_procedure_models) => {
                disposal_procedure_models.primary_keys()
            }
            Rows::Document(documents) => documents.primary_keys(),
            Rows::EmailProvider(email_providers) => email_providers.primary_keys(),
            Rows::FractioningProcedureModel(fractioning_procedure_models) => {
                fractioning_procedure_models.primary_keys()
            }
            Rows::FreezeDrierModel(freeze_drier_models) => freeze_drier_models.primary_keys(),
            Rows::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models.primary_keys()
            }
            Rows::FreezerModel(freezer_models) => freezer_models.primary_keys(),
            Rows::FreezingProcedureModel(freezing_procedure_models) => {
                freezing_procedure_models.primary_keys()
            }
            Rows::GeolocationProcedureModel(geolocation_procedure_models) => {
                geolocation_procedure_models.primary_keys()
            }
            Rows::InstrumentModel(instrument_models) => instrument_models.primary_keys(),
            Rows::InstrumentState(instrument_states) => instrument_states.primary_keys(),
            Rows::LoginProvider(login_providers) => login_providers.primary_keys(),
            Rows::Material(materials) => materials.primary_keys(),
            Rows::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models.primary_keys()
            }
            Rows::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models.primary_keys()
            }
            Rows::MountTipProcedureModel(mount_tip_procedure_models) => {
                mount_tip_procedure_models.primary_keys()
            }
            Rows::NextProcedureModel(next_procedure_models) => next_procedure_models.primary_keys(),
            Rows::ObservationSubject(observation_subjects) => observation_subjects.primary_keys(),
            Rows::OrganismTaxon(organism_taxa) => organism_taxa.primary_keys(),
            Rows::Organism(organisms) => organisms.primary_keys(),
            Rows::Organization(organizations) => organizations.primary_keys(),
            Rows::PackagingProcedureModel(packaging_procedure_models) => {
                packaging_procedure_models.primary_keys()
            }
            Rows::ParentProcedureModel(parent_procedure_models) => {
                parent_procedure_models.primary_keys()
            }
            Rows::PermanenceCategory(permanence_categories) => permanence_categories.primary_keys(),
            Rows::PhotographProcedureModel(photograph_procedure_models) => {
                photograph_procedure_models.primary_keys()
            }
            Rows::PipetteModel(pipette_models) => pipette_models.primary_keys(),
            Rows::PipetteTipModel(pipette_tip_models) => pipette_tip_models.primary_keys(),
            Rows::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models.primary_keys()
            }
            Rows::PouringProcedureModel(pouring_procedure_models) => {
                pouring_procedure_models.primary_keys()
            }
            Rows::ProcedureModelTrackable(procedure_model_trackables) => {
                procedure_model_trackables.primary_keys()
            }
            Rows::ProcedureModel(procedure_models) => procedure_models.primary_keys(),
            Rows::ProcedureTrackable(procedure_trackables) => procedure_trackables.primary_keys(),
            Rows::Procedure(procedures) => procedures.primary_keys(),
            Rows::Processable(processables) => processables.primary_keys(),
            Rows::ProjectState(project_states) => project_states.primary_keys(),
            Rows::Project(projects) => projects.primary_keys(),
            Rows::Rank(ranks) => ranks.primary_keys(),
            Rows::Reagent(reagents) => reagents.primary_keys(),
            Rows::Role(roles) => roles.primary_keys(),
            Rows::Room(rooms) => rooms.primary_keys(),
            Rows::SampleState(sample_states) => sample_states.primary_keys(),
            Rows::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables.primary_keys()
            }
            Rows::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_keys(),
            Rows::Spectrum(spectra) => spectra.primary_keys(),
            Rows::SpectraCollection(spectra_collections) => spectra_collections.primary_keys(),
            Rows::StorageProcedureModel(storage_procedure_models) => {
                storage_procedure_models.primary_keys()
            }
            Rows::SupernatantProcedureModel(supernatant_procedure_models) => {
                supernatant_procedure_models.primary_keys()
            }
            Rows::Taxon(taxa) => taxa.primary_keys(),
            Rows::TeamMember(team_members) => team_members.primary_keys(),
            Rows::TeamProject(team_projects) => team_projects.primary_keys(),
            Rows::TeamState(team_states) => team_states.primary_keys(),
            Rows::Team(teams) => teams.primary_keys(),
            Rows::TemporaryUser(temporary_user) => temporary_user.primary_keys(),
            Rows::TrackableLocation(trackable_locations) => trackable_locations.primary_keys(),
            Rows::Trackable(trackables) => trackables.primary_keys(),
            Rows::Unit(units) => units.primary_keys(),
            Rows::UserEmail(user_emails) => user_emails.primary_keys(),
            Rows::UserOrganization(user_organizations) => user_organizations.primary_keys(),
            Rows::User(users) => users.primary_keys(),
            Rows::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.primary_keys()
            }
            Rows::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_keys()
            }
            Rows::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_keys()
            }
            Rows::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models.primary_keys()
            }
            Rows::WeighingProcedure(weighing_procedures) => weighing_procedures.primary_keys(),
        }
    }
}
