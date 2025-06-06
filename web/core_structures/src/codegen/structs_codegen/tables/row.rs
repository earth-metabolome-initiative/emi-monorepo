mod addresses;
mod aliquoting_instrument_models;
mod aliquoting_procedure_models;
mod aliquoting_procedures;
mod ball_mill_procedure_models;
mod brands;
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
mod fractioning_procedures;
mod freeze_drying_procedure_models;
mod from_row;
mod instrument_models;
mod instrument_states;
mod instruments;
mod login_providers;
mod materials;
mod mix_countable_procedure_models;
mod mix_solid_procedure_models;
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
mod postgres_read_dispatch;
mod pouring_procedure_models;
mod procedure_model_trackables;
mod procedure_models;
mod procedure_trackables;
mod procedures;
mod processables;
mod processing_procedures;
mod project_states;
mod projects;
mod ranks;
mod reagents;
mod roles;
mod rooms;
mod sample_states;
mod sampling_procedure_models;
mod sampling_procedures;
mod shaking_procedure_models;
mod shared_procedure_model_trackables;
mod spatial_ref_sys;
mod spectra;
mod spectra_collections;
mod sqlite_read_dispatch;
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
mod volumetric_processables;
mod weighing_instrument_models;
mod weighing_procedure_models;
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Row {
    Address(crate::codegen::structs_codegen::tables::addresses::Address),
    AliquotingInstrumentModel(
        crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    ),
    AliquotingProcedureModel(
        crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    ),
    AliquotingProcedure(
        crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    ),
    BallMillProcedureModel(
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    ),
    Brand(crate::codegen::structs_codegen::tables::brands::Brand),
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
    FractioningProcedure(
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    ),
    FreezeDryingProcedureModel(
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel,
    ),
    InstrumentModel(
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ),
    InstrumentState(
        crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ),
    Instrument(crate::codegen::structs_codegen::tables::instruments::Instrument),
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
    ProcessingProcedure(
        crate::codegen::structs_codegen::tables::processing_procedures::ProcessingProcedure,
    ),
    ProjectState(crate::codegen::structs_codegen::tables::project_states::ProjectState),
    Project(crate::codegen::structs_codegen::tables::projects::Project),
    Rank(crate::codegen::structs_codegen::tables::ranks::Rank),
    Reagent(crate::codegen::structs_codegen::tables::reagents::Reagent),
    Role(crate::codegen::structs_codegen::tables::roles::Role),
    Room(crate::codegen::structs_codegen::tables::rooms::Room),
    SampleState(crate::codegen::structs_codegen::tables::sample_states::SampleState),
    SamplingProcedureModel(
        crate::codegen::structs_codegen::tables::sampling_procedure_models::SamplingProcedureModel,
    ),
    SamplingProcedure(
        crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure,
    ),
    ShakingProcedureModel(
        crate::codegen::structs_codegen::tables::shaking_procedure_models::ShakingProcedureModel,
    ),
    SharedProcedureModelTrackable(
        crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
    ),
    SpatialRefSy(crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy),
    Spectrum(crate::codegen::structs_codegen::tables::spectra::Spectrum),
    SpectraCollection(
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
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
    VolumetricProcessable(
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    ),
    WeighingInstrumentModel(
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ),
    WeighingProcedureModel(
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
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
            Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::AliquotingProcedure(aliquoting_procedures) => {
                aliquoting_procedures.upsert(conn)?.map(Row::from)
            }
            Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::Brand(brands) => brands.upsert(conn)?.map(Row::from),
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
            Row::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentModel(instrument_models) => {
                instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentState(instrument_states) => {
                instrument_states.upsert(conn)?.map(Row::from)
            }
            Row::Instrument(instruments) => instruments.upsert(conn)?.map(Row::from),
            Row::LoginProvider(login_providers) => login_providers.upsert(conn)?.map(Row::from),
            Row::Material(materials) => materials.upsert(conn)?.map(Row::from),
            Row::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models.upsert(conn)?.map(Row::from)
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
            Row::ProcessingProcedure(processing_procedures) => {
                processing_procedures.upsert(conn)?.map(Row::from)
            }
            Row::ProjectState(project_states) => project_states.upsert(conn)?.map(Row::from),
            Row::Project(projects) => projects.upsert(conn)?.map(Row::from),
            Row::Rank(ranks) => ranks.upsert(conn)?.map(Row::from),
            Row::Reagent(reagents) => reagents.upsert(conn)?.map(Row::from),
            Row::Role(roles) => roles.upsert(conn)?.map(Row::from),
            Row::Room(rooms) => rooms.upsert(conn)?.map(Row::from),
            Row::SampleState(sample_states) => sample_states.upsert(conn)?.map(Row::from),
            Row::SamplingProcedureModel(sampling_procedure_models) => {
                sampling_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::SamplingProcedure(sampling_procedures) => {
                sampling_procedures.upsert(conn)?.map(Row::from)
            }
            Row::ShakingProcedureModel(shaking_procedure_models) => {
                shaking_procedure_models.upsert(conn)?.map(Row::from)
            }
            Row::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables.upsert(conn)?.map(Row::from)
            }
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.upsert(conn)?.map(Row::from),
            Row::Spectrum(spectra) => spectra.upsert(conn)?.map(Row::from),
            Row::SpectraCollection(spectra_collections) => {
                spectra_collections.upsert(conn)?.map(Row::from)
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
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.upsert(conn)?.map(Row::from)
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models.upsert(conn)?.map(Row::from)
            }
        })
    }
}
impl web_common_traits::prelude::Row for Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            Row::Address(addresses) => addresses.primary_key(),
            Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.primary_key()
            }
            Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                aliquoting_procedure_models.primary_key()
            }
            Row::AliquotingProcedure(aliquoting_procedures) => aliquoting_procedures.primary_key(),
            Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                ball_mill_procedure_models.primary_key()
            }
            Row::Brand(brands) => brands.primary_key(),
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
            Row::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures.primary_key()
            }
            Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                freeze_drying_procedure_models.primary_key()
            }
            Row::InstrumentModel(instrument_models) => instrument_models.primary_key(),
            Row::InstrumentState(instrument_states) => instrument_states.primary_key(),
            Row::Instrument(instruments) => instruments.primary_key(),
            Row::LoginProvider(login_providers) => login_providers.primary_key(),
            Row::Material(materials) => materials.primary_key(),
            Row::MixCountableProcedureModel(mix_countable_procedure_models) => {
                mix_countable_procedure_models.primary_key()
            }
            Row::MixSolidProcedureModel(mix_solid_procedure_models) => {
                mix_solid_procedure_models.primary_key()
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
            Row::ProcessingProcedure(processing_procedures) => processing_procedures.primary_key(),
            Row::ProjectState(project_states) => project_states.primary_key(),
            Row::Project(projects) => projects.primary_key(),
            Row::Rank(ranks) => ranks.primary_key(),
            Row::Reagent(reagents) => reagents.primary_key(),
            Row::Role(roles) => roles.primary_key(),
            Row::Room(rooms) => rooms.primary_key(),
            Row::SampleState(sample_states) => sample_states.primary_key(),
            Row::SamplingProcedureModel(sampling_procedure_models) => {
                sampling_procedure_models.primary_key()
            }
            Row::SamplingProcedure(sampling_procedures) => sampling_procedures.primary_key(),
            Row::ShakingProcedureModel(shaking_procedure_models) => {
                shaking_procedure_models.primary_key()
            }
            Row::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                shared_procedure_model_trackables.primary_key()
            }
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_key(),
            Row::Spectrum(spectra) => spectra.primary_key(),
            Row::SpectraCollection(spectra_collections) => spectra_collections.primary_key(),
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
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_key()
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_key()
            }
            Row::WeighingProcedureModel(weighing_procedure_models) => {
                weighing_procedure_models.primary_key()
            }
        }
    }
}
