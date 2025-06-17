mod addresses;
mod aliquoting_procedure_models;
mod ball_mill_container_models;
mod ball_mill_machine_models;
mod ball_mill_procedure_models;
mod brands;
mod capping_procedure_models;
mod capping_rules;
mod centrifugable_container_models;
mod centrifuge_models;
mod centrifuge_procedure_models;
mod cities;
mod colors;
mod commercial_product_lots;
mod commercial_products;
mod commercial_reagents;
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
mod from_row;
mod instrument_models;
mod instrument_states;
mod login_providers;
mod materials;
mod mix_countable_procedure_models;
mod mix_solid_procedure_models;
mod mount_tip_procedure_models;
mod next_procedure_models;
mod observation_subjects;
mod organism_observations;
mod organism_taxa;
mod organisms;
mod organizations;
mod packaging_models;
mod packaging_procedure_models;
mod parent_procedure_models;
mod permanence_categories;
mod pouring_procedure_models;
mod procedure_model_trackables;
mod procedure_models;
mod procedure_trackables;
mod procedures;
mod processables;
mod project_states;
mod projects;
mod ranks;
mod read_dispatch;
mod reagents;
mod roles;
mod rooms;
mod sample_states;
mod shared_procedure_model_trackables;
mod spatial_ref_sys;
mod spectra;
mod spectra_collections;
mod storage_procedure_models;
mod storage_rules;
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
pub enum Row {
    Address(crate::codegen::structs_codegen::tables::addresses::Address),
    AliquotingProcedureModel(
        crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    ),
    BallMillContainerModel(
        crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel,
    ),
    BallMillMachineModel(
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ),
    BallMillProcedureModel(
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    ),
    Brand(crate::codegen::structs_codegen::tables::brands::Brand),
    CappingProcedureModel(
        crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel,
    ),
    CappingRule(crate::codegen::structs_codegen::tables::capping_rules::CappingRule),
    CentrifugableContainerModel(
        crate::codegen::structs_codegen::tables::centrifugable_container_models::CentrifugableContainerModel,
    ),
    CentrifugeModel(
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    ),
    CentrifugeProcedureModel(
        crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel,
    ),
    City(crate::codegen::structs_codegen::tables::cities::City),
    Color(crate::codegen::structs_codegen::tables::colors::Color),
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ),
    CommercialReagent(
        crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
    ),
    ContainerModel(
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ),
    Container(crate::codegen::structs_codegen::tables::containers::Container),
    Country(crate::codegen::structs_codegen::tables::countries::Country),
    DisposalProcedureModel(
        crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel,
    ),
    Document(crate::codegen::structs_codegen::tables::documents::Document),
    EmailProvider(
        crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    ),
    FractioningProcedureModel(
        crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
    ),
    FreezeDrierModel(
        crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
    ),
    FreezeDryingProcedureModel(
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
    ),
    FreezerModel(crate::codegen::structs_codegen::tables::freezer_models::FreezerModel),
    FreezingProcedureModel(
        crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel,
    ),
    InstrumentModel(
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ),
    InstrumentState(
        crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ),
    LoginProvider(
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ),
    Material(crate::codegen::structs_codegen::tables::materials::Material),
    MixCountableProcedureModel(
        crate::codegen::structs_codegen::tables::mix_countable_procedure_models::MixCountableProcedureModel,
    ),
    MixSolidProcedureModel(
        crate::codegen::structs_codegen::tables::mix_solid_procedure_models::MixSolidProcedureModel,
    ),
    MountTipProcedureModel(
        crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    ),
    NextProcedureModel(
        crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel,
    ),
    ObservationSubject(
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    ),
    OrganismObservation(
        crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
    ),
    OrganismTaxon(crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon),
    Organism(crate::codegen::structs_codegen::tables::organisms::Organism),
    Organization(crate::codegen::structs_codegen::tables::organizations::Organization),
    PackagingModel(
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ),
    PackagingProcedureModel(
        crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
    ),
    ParentProcedureModel(
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
    ),
    PermanenceCategory(
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
    ),
    PouringProcedureModel(
        crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel,
    ),
    ProcedureModelTrackable(
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
    ),
    ProcedureModel(
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    ),
    ProcedureTrackable(
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    ),
    Procedure(crate::codegen::structs_codegen::tables::procedures::Procedure),
    Processable(crate::codegen::structs_codegen::tables::processables::Processable),
    ProjectState(crate::codegen::structs_codegen::tables::project_states::ProjectState),
    Project(crate::codegen::structs_codegen::tables::projects::Project),
    Rank(crate::codegen::structs_codegen::tables::ranks::Rank),
    Reagent(crate::codegen::structs_codegen::tables::reagents::Reagent),
    Role(crate::codegen::structs_codegen::tables::roles::Role),
    Room(crate::codegen::structs_codegen::tables::rooms::Room),
    SampleState(crate::codegen::structs_codegen::tables::sample_states::SampleState),
    SharedProcedureModelTrackable(
        crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
    ),
    SpatialRefSy(crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy),
    Spectrum(crate::codegen::structs_codegen::tables::spectra::Spectrum),
    SpectraCollection(
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ),
    StorageProcedureModel(
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    ),
    StorageRule(crate::codegen::structs_codegen::tables::storage_rules::StorageRule),
    SupernatantProcedureModel(
        crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel,
    ),
    Taxon(crate::codegen::structs_codegen::tables::taxa::Taxon),
    TeamMember(crate::codegen::structs_codegen::tables::team_members::TeamMember),
    TeamProject(crate::codegen::structs_codegen::tables::team_projects::TeamProject),
    TeamState(crate::codegen::structs_codegen::tables::team_states::TeamState),
    Team(crate::codegen::structs_codegen::tables::teams::Team),
    TemporaryUser(
        crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
    ),
    TrackableLocation(
        crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
    ),
    Trackable(crate::codegen::structs_codegen::tables::trackables::Trackable),
    Unit(crate::codegen::structs_codegen::tables::units::Unit),
    UserEmail(crate::codegen::structs_codegen::tables::user_emails::UserEmail),
    UserOrganization(
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    ),
    User(crate::codegen::structs_codegen::tables::users::User),
    VolumetricContainerModel(
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    ),
    VolumetricProcessable(
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    ),
    WeighingInstrumentModel(
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ),
    WeighingProcedureModel(
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
    ),
    WeighingProcedure(
        crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    ),
}
impl Row {
    #[cfg(feature = "sqlite")]
    /// Executes the upsert operation and returns the result.
    pub fn sqlite_upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Row>, diesel::result::Error> {
        use web_common_traits::database::Upsertable;
        Ok(match self {
            Row::Address(addresses) => addresses.upsert(conn)?.map(Row::from),
            Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::BallMillContainerModel(ball_mill_container_models) => {
                ball_mill_container_models.upsert(conn)?.map(Row::from)
            }
            Row::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.upsert(conn)?.map(Row::from)
            }
            Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::Brand(brands) => brands.upsert(conn)?.map(Row::from),
            Row::CappingProcedureModel(capping_procedure_models) => {
                capping_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::CappingRule(capping_rules) => capping_rules.upsert(conn)?.map(Row::from),
            Row::CentrifugableContainerModel(centrifugable_container_models) => {
                centrifugable_container_models.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeModel(centrifuge_models) => {
                centrifuge_models.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeProcedureModel(centrifuge_procedure_models) => {
                centrifuge_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::City(cities) => cities.upsert(conn)?.map(Row::from),
            Row::Color(colors) => colors.upsert(conn)?.map(Row::from),
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialProduct(commercial_products) => {
                commercial_products.upsert(conn)?.map(Row::from)
            }
            Row::CommercialReagent(commercial_reagents) => {
                commercial_reagents.upsert(conn)?.map(Row::from)
            }
            Row::ContainerModel(container_models) => container_models.upsert(conn)?.map(Row::from),
            Row::Container(containers) => containers.upsert(conn)?.map(Row::from),
            Row::Country(countries) => countries.upsert(conn)?.map(Row::from),
            Row::DisposalProcedureModel(disposal_procedure_models) => {
                disposal_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::Document(documents) => documents.upsert(conn)?.map(Row::from),
            Row::EmailProvider(email_providers) => email_providers.upsert(conn)?.map(Row::from),
            Row::FractioningProcedureModel(fractioning_procedure_models) => {
                fractioning_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDrierModel(freeze_drier_models) => {
                freeze_drier_models.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::FreezerModel(freezer_models) => freezer_models.upsert(conn)?.map(Row::from),
            Row::FreezingProcedureModel(freezing_procedure_models) => {
                freezing_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentModel(instrument_models) => {
                instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentState(instrument_states) => {
                instrument_states.upsert(conn)?.map(Row::from)
            }
            Row::LoginProvider(login_providers) => login_providers.upsert(conn)?.map(Row::from),
            Row::Material(materials) => materials.upsert(conn)?.map(Row::from),
            Row::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::MountTipProcedureModel(mount_tip_procedure_models) => {
                mount_tip_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::NextProcedureModel(next_procedure_models) => {
                next_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::ObservationSubject(observation_subjects) => {
                observation_subjects.upsert(conn)?.map(Row::from)
            }
            Row::OrganismObservation(organism_observations) => {
                organism_observations.upsert(conn)?.map(Row::from)
            }
            Row::OrganismTaxon(organism_taxa) => organism_taxa.upsert(conn)?.map(Row::from),
            Row::Organism(organisms) => organisms.upsert(conn)?.map(Row::from),
            Row::Organization(organizations) => organizations.upsert(conn)?.map(Row::from),
            Row::PackagingModel(packaging_models) => packaging_models.upsert(conn)?.map(Row::from),
            Row::PackagingProcedureModel(packaging_procedure_models) => {
                packaging_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::ParentProcedureModel(parent_procedure_models) => {
                parent_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::PermanenceCategory(permanence_categories) => {
                permanence_categories.upsert(conn)?.map(Row::from)
            }
            Row::PouringProcedureModel(pouring_procedure_models) => {
                pouring_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModelTrackable(procedure_model_trackables) => {
                procedure_model_trackables.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModel(procedure_models) => procedure_models.upsert(conn)?.map(Row::from),
            Row::ProcedureTrackable(procedure_trackables) => {
                procedure_trackables.upsert(conn)?.map(Row::from)
            }
            Row::Procedure(procedures) => procedures.upsert(conn)?.map(Row::from),
            Row::Processable(processables) => processables.upsert(conn)?.map(Row::from),
            Row::ProjectState(project_states) => project_states.upsert(conn)?.map(Row::from),
            Row::Project(projects) => projects.upsert(conn)?.map(Row::from),
            Row::Rank(ranks) => ranks.upsert(conn)?.map(Row::from),
            Row::Reagent(reagents) => reagents.upsert(conn)?.map(Row::from),
            Row::Role(roles) => roles.upsert(conn)?.map(Row::from),
            Row::Room(rooms) => rooms.upsert(conn)?.map(Row::from),
            Row::SampleState(sample_states) => sample_states.upsert(conn)?.map(Row::from),
            Row::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables.upsert(conn)?.map(Row::from)
            }
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.upsert(conn)?.map(Row::from),
            Row::Spectrum(spectra) => spectra.upsert(conn)?.map(Row::from),
            Row::SpectraCollection(spectra_collections) => {
                spectra_collections.upsert(conn)?.map(Row::from)
            }
            Row::StorageProcedureModel(storage_procedure_models) => {
                storage_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::StorageRule(storage_rules) => storage_rules.upsert(conn)?.map(Row::from),
            Row::SupernatantProcedureModel(supernatant_procedure_models) => {
                supernatant_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::Taxon(taxa) => taxa.upsert(conn)?.map(Row::from),
            Row::TeamMember(team_members) => team_members.upsert(conn)?.map(Row::from),
            Row::TeamProject(team_projects) => team_projects.upsert(conn)?.map(Row::from),
            Row::TeamState(team_states) => team_states.upsert(conn)?.map(Row::from),
            Row::Team(teams) => teams.upsert(conn)?.map(Row::from),
            Row::TemporaryUser(temporary_user) => temporary_user.upsert(conn)?.map(Row::from),
            Row::TrackableLocation(trackable_locations) => {
                trackable_locations.upsert(conn)?.map(Row::from)
            }
            Row::Trackable(trackables) => trackables.upsert(conn)?.map(Row::from),
            Row::Unit(units) => units.upsert(conn)?.map(Row::from),
            Row::UserEmail(user_emails) => user_emails.upsert(conn)?.map(Row::from),
            Row::UserOrganization(user_organizations) => {
                user_organizations.upsert(conn)?.map(Row::from)
            }
            Row::User(users) => users.upsert(conn)?.map(Row::from),
            Row::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.upsert(conn)?.map(Row::from)
            }
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.upsert(conn)?.map(Row::from)
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingProcedure(weighing_procedures) => {
                weighing_procedures.upsert(conn)?.map(Row::from)
            }
        })
    }
}
impl web_common_traits::prelude::Row for Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            Row::Address(addresses) => addresses.primary_key(),
            Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models.primary_key()
            }
            Row::BallMillContainerModel(ball_mill_container_models) => {
                ball_mill_container_models.primary_key()
            }
            Row::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.primary_key()
            }
            Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models.primary_key()
            }
            Row::Brand(brands) => brands.primary_key(),
            Row::CappingProcedureModel(capping_procedure_models) => {
                capping_procedure_models.primary_key()
            }
            Row::CappingRule(capping_rules) => capping_rules.primary_key(),
            Row::CentrifugableContainerModel(centrifugable_container_models) => {
                centrifugable_container_models.primary_key()
            }
            Row::CentrifugeModel(centrifuge_models) => centrifuge_models.primary_key(),
            Row::CentrifugeProcedureModel(centrifuge_procedure_models) => {
                centrifuge_procedure_models.primary_key()
            }
            Row::City(cities) => cities.primary_key(),
            Row::Color(colors) => colors.primary_key(),
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_key()
            }
            Row::CommercialProduct(commercial_products) => commercial_products.primary_key(),
            Row::CommercialReagent(commercial_reagents) => commercial_reagents.primary_key(),
            Row::ContainerModel(container_models) => container_models.primary_key(),
            Row::Container(containers) => containers.primary_key(),
            Row::Country(countries) => countries.primary_key(),
            Row::DisposalProcedureModel(disposal_procedure_models) => {
                disposal_procedure_models.primary_key()
            }
            Row::Document(documents) => documents.primary_key(),
            Row::EmailProvider(email_providers) => email_providers.primary_key(),
            Row::FractioningProcedureModel(fractioning_procedure_models) => {
                fractioning_procedure_models.primary_key()
            }
            Row::FreezeDrierModel(freeze_drier_models) => freeze_drier_models.primary_key(),
            Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models.primary_key()
            }
            Row::FreezerModel(freezer_models) => freezer_models.primary_key(),
            Row::FreezingProcedureModel(freezing_procedure_models) => {
                freezing_procedure_models.primary_key()
            }
            Row::InstrumentModel(instrument_models) => instrument_models.primary_key(),
            Row::InstrumentState(instrument_states) => instrument_states.primary_key(),
            Row::LoginProvider(login_providers) => login_providers.primary_key(),
            Row::Material(materials) => materials.primary_key(),
            Row::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models.primary_key()
            }
            Row::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models.primary_key()
            }
            Row::MountTipProcedureModel(mount_tip_procedure_models) => {
                mount_tip_procedure_models.primary_key()
            }
            Row::NextProcedureModel(next_procedure_models) => next_procedure_models.primary_key(),
            Row::ObservationSubject(observation_subjects) => observation_subjects.primary_key(),
            Row::OrganismObservation(organism_observations) => organism_observations.primary_key(),
            Row::OrganismTaxon(organism_taxa) => organism_taxa.primary_key(),
            Row::Organism(organisms) => organisms.primary_key(),
            Row::Organization(organizations) => organizations.primary_key(),
            Row::PackagingModel(packaging_models) => packaging_models.primary_key(),
            Row::PackagingProcedureModel(packaging_procedure_models) => {
                packaging_procedure_models.primary_key()
            }
            Row::ParentProcedureModel(parent_procedure_models) => {
                parent_procedure_models.primary_key()
            }
            Row::PermanenceCategory(permanence_categories) => permanence_categories.primary_key(),
            Row::PouringProcedureModel(pouring_procedure_models) => {
                pouring_procedure_models.primary_key()
            }
            Row::ProcedureModelTrackable(procedure_model_trackables) => {
                procedure_model_trackables.primary_key()
            }
            Row::ProcedureModel(procedure_models) => procedure_models.primary_key(),
            Row::ProcedureTrackable(procedure_trackables) => procedure_trackables.primary_key(),
            Row::Procedure(procedures) => procedures.primary_key(),
            Row::Processable(processables) => processables.primary_key(),
            Row::ProjectState(project_states) => project_states.primary_key(),
            Row::Project(projects) => projects.primary_key(),
            Row::Rank(ranks) => ranks.primary_key(),
            Row::Reagent(reagents) => reagents.primary_key(),
            Row::Role(roles) => roles.primary_key(),
            Row::Room(rooms) => rooms.primary_key(),
            Row::SampleState(sample_states) => sample_states.primary_key(),
            Row::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables.primary_key()
            }
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_key(),
            Row::Spectrum(spectra) => spectra.primary_key(),
            Row::SpectraCollection(spectra_collections) => spectra_collections.primary_key(),
            Row::StorageProcedureModel(storage_procedure_models) => {
                storage_procedure_models.primary_key()
            }
            Row::StorageRule(storage_rules) => storage_rules.primary_key(),
            Row::SupernatantProcedureModel(supernatant_procedure_models) => {
                supernatant_procedure_models.primary_key()
            }
            Row::Taxon(taxa) => taxa.primary_key(),
            Row::TeamMember(team_members) => team_members.primary_key(),
            Row::TeamProject(team_projects) => team_projects.primary_key(),
            Row::TeamState(team_states) => team_states.primary_key(),
            Row::Team(teams) => teams.primary_key(),
            Row::TemporaryUser(temporary_user) => temporary_user.primary_key(),
            Row::TrackableLocation(trackable_locations) => trackable_locations.primary_key(),
            Row::Trackable(trackables) => trackables.primary_key(),
            Row::Unit(units) => units.primary_key(),
            Row::UserEmail(user_emails) => user_emails.primary_key(),
            Row::UserOrganization(user_organizations) => user_organizations.primary_key(),
            Row::User(users) => users.primary_key(),
            Row::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.primary_key()
            }
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_key()
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_key()
            }
            Row::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models.primary_key()
            }
            Row::WeighingProcedure(weighing_procedures) => weighing_procedures.primary_key(),
        }
    }
}
