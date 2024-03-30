use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Archivable {
    pub id: Uuid,
    pub archived_at: NaiveDateTime,
    pub archived_by: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContainerHorizontalRule {
    pub id: Uuid,
    pub item_type_id: Option<Uuid>,
    pub other_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContainerVerticalRule {
    pub id: Uuid,
    pub container_item_type_id: Option<Uuid>,
    pub contained_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContinuousUnit {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Describable {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscreteUnit {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DocumentFormat {
    pub id: Uuid,
    pub mime_type: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Document {
    pub id: Uuid,
    pub path: String,
    pub format_id: Uuid,
    pub bytes: i32,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Editable {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Edit {
    pub id: Uuid,
    pub editable_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategory {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategoryRelationship {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub child_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategoryUnit {
    pub id: Uuid,
    pub item_category_id: Uuid,
    pub unit_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemContinuousQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub weight: f64,
    pub unit_id: Option<Uuid>,
    pub sensor_id: Option<Uuid>,
    pub measured_at: DateTime<Utc>,
    pub measured_by: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemDiscreteQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
    pub measured_at: DateTime<Utc>,
    pub measured_by: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemLocation {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemUnit {
    pub id: Uuid,
    pub item_id: Uuid,
    pub unit_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LocationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Location {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub address: Option<String>,
    pub geolocalization_device_id: Option<Uuid>,
    pub altitude_device_id: Option<Uuid>,
    pub parent_location_id: Option<Uuid>,
    pub state_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LoginProvider {
    pub id: Uuid,
    pub name: String,
    pub font_awesome_icon: String,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ManufacturedItemCategory {
    pub id: Uuid,
    pub cost: f64,
    pub cost_per_day: f64,
    pub currency: String,
    pub manifacturer_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub operation: String,
    pub table_name: String,
    pub row_id: Option<Uuid>,
    pub read: bool,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationAuthorization {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationLocation {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Organization {
    pub id: Uuid,
    pub state_id: Option<Uuid>,
    pub parent_organization_id: Option<Uuid>,
    pub logo_id: Option<Uuid>,
    pub website_url: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PrimaryUserEmail {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProcedureContinuousRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProcedureDiscreteRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Procedure {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectContinuousRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectDiscreteRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectMilestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub due_date: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectState {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: Uuid,
    pub parent_project_id: Option<Uuid>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub currency: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub expected_end_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Role {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampleTaxa {
    pub id: Uuid,
    pub sample_id: Uuid,
    pub taxon_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampledIndividualTaxa {
    pub id: Uuid,
    pub sampled_individual_id: Uuid,
    pub taxon_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampledIndividual {
    pub id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Sample {
    pub id: Uuid,
    pub derived_from: Option<Uuid>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Spectra {
    pub id: Uuid,
    pub spectra_collection_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SpectraCollection {
    pub id: Uuid,
    pub sample_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Taxa {
    pub id: Uuid,
    pub name: String,
    pub ncbi_taxon_id: Option<i32>,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamAuthorization {
    pub id: Uuid,
    pub team_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamState {
    pub id: Uuid,
    pub font_awesome_icon: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Team {
    pub id: Uuid,
    pub parent_team_id: Option<Uuid>,
    pub team_state_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Unit {
    pub id: Uuid,
    pub symbol: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserAuthorization {
    pub id: Uuid,
    pub user_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserEmail {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub login_provider_id: Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum Table {
    Archivable,
    ContainerHorizontalRule,
    ContainerVerticalRule,
    ContinuousUnit,
    Describable,
    DiscreteUnit,
    DocumentFormat,
    Document,
    Editable,
    Edit,
    ItemCategory,
    ItemCategoryRelationship,
    ItemCategoryUnit,
    ItemContinuousQuantity,
    ItemDiscreteQuantity,
    ItemLocation,
    ItemUnit,
    Item,
    LocationState,
    Location,
    LoginProvider,
    ManufacturedItemCategory,
    Notification,
    OrganizationAuthorization,
    OrganizationLocation,
    OrganizationState,
    Organization,
    PrimaryUserEmail,
    ProcedureContinuousRequirement,
    ProcedureDiscreteRequirement,
    Procedure,
    ProjectContinuousRequirement,
    ProjectDiscreteRequirement,
    ProjectMilestone,
    ProjectState,
    Project,
    Role,
    SampleTaxa,
    SampledIndividualTaxa,
    SampledIndividual,
    Sample,
    Spectra,
    SpectraCollection,
    Taxa,
    TeamAuthorization,
    TeamState,
    Team,
    Unit,
    UserAuthorization,
    UserEmail,
    User,
}

impl Table {
    pub fn name(&self) -> &'static str {
        match self {
            Table::Archivable => "users",
            Table::ContainerHorizontalRule => "users",
            Table::ContainerVerticalRule => "users",
            Table::ContinuousUnit => "users",
            Table::Describable => "users",
            Table::DiscreteUnit => "users",
            Table::DocumentFormat => "users",
            Table::Document => "users",
            Table::Editable => "users",
            Table::Edit => "users",
            Table::ItemCategory => "users",
            Table::ItemCategoryRelationship => "users",
            Table::ItemCategoryUnit => "users",
            Table::ItemContinuousQuantity => "users",
            Table::ItemDiscreteQuantity => "users",
            Table::ItemLocation => "users",
            Table::ItemUnit => "users",
            Table::Item => "users",
            Table::LocationState => "users",
            Table::Location => "users",
            Table::LoginProvider => "users",
            Table::ManufacturedItemCategory => "users",
            Table::Notification => "users",
            Table::OrganizationAuthorization => "users",
            Table::OrganizationLocation => "users",
            Table::OrganizationState => "users",
            Table::Organization => "users",
            Table::PrimaryUserEmail => "users",
            Table::ProcedureContinuousRequirement => "users",
            Table::ProcedureDiscreteRequirement => "users",
            Table::Procedure => "users",
            Table::ProjectContinuousRequirement => "users",
            Table::ProjectDiscreteRequirement => "users",
            Table::ProjectMilestone => "users",
            Table::ProjectState => "users",
            Table::Project => "users",
            Table::Role => "users",
            Table::SampleTaxa => "users",
            Table::SampledIndividualTaxa => "users",
            Table::SampledIndividual => "users",
            Table::Sample => "users",
            Table::Spectra => "users",
            Table::SpectraCollection => "users",
            Table::Taxa => "users",
            Table::TeamAuthorization => "users",
            Table::TeamState => "users",
            Table::Team => "users",
            Table::Unit => "users",
            Table::UserAuthorization => "users",
            Table::UserEmail => "users",
            Table::User => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum TableRow {
    Archivable(Archivable),
    ContainerHorizontalRule(ContainerHorizontalRule),
    ContainerVerticalRule(ContainerVerticalRule),
    ContinuousUnit(ContinuousUnit),
    Describable(Describable),
    DiscreteUnit(DiscreteUnit),
    DocumentFormat(DocumentFormat),
    Document(Document),
    Editable(Editable),
    Edit(Edit),
    ItemCategory(ItemCategory),
    ItemCategoryRelationship(ItemCategoryRelationship),
    ItemCategoryUnit(ItemCategoryUnit),
    ItemContinuousQuantity(ItemContinuousQuantity),
    ItemDiscreteQuantity(ItemDiscreteQuantity),
    ItemLocation(ItemLocation),
    ItemUnit(ItemUnit),
    Item(Item),
    LocationState(LocationState),
    Location(Location),
    LoginProvider(LoginProvider),
    ManufacturedItemCategory(ManufacturedItemCategory),
    Notification(Notification),
    OrganizationAuthorization(OrganizationAuthorization),
    OrganizationLocation(OrganizationLocation),
    OrganizationState(OrganizationState),
    Organization(Organization),
    PrimaryUserEmail(PrimaryUserEmail),
    ProcedureContinuousRequirement(ProcedureContinuousRequirement),
    ProcedureDiscreteRequirement(ProcedureDiscreteRequirement),
    Procedure(Procedure),
    ProjectContinuousRequirement(ProjectContinuousRequirement),
    ProjectDiscreteRequirement(ProjectDiscreteRequirement),
    ProjectMilestone(ProjectMilestone),
    ProjectState(ProjectState),
    Project(Project),
    Role(Role),
    SampleTaxa(SampleTaxa),
    SampledIndividualTaxa(SampledIndividualTaxa),
    SampledIndividual(SampledIndividual),
    Sample(Sample),
    Spectra(Spectra),
    SpectraCollection(SpectraCollection),
    Taxa(Taxa),
    TeamAuthorization(TeamAuthorization),
    TeamState(TeamState),
    Team(Team),
    Unit(Unit),
    UserAuthorization(UserAuthorization),
    UserEmail(UserEmail),
    User(User),
}

impl From<&str> for Table {
    fn from(item: &str) -> Self {
        match item {
            "users" => Table::Archivable,
            "users" => Table::ContainerHorizontalRule,
            "users" => Table::ContainerVerticalRule,
            "users" => Table::ContinuousUnit,
            "users" => Table::Describable,
            "users" => Table::DiscreteUnit,
            "users" => Table::DocumentFormat,
            "users" => Table::Document,
            "users" => Table::Editable,
            "users" => Table::Edit,
            "users" => Table::ItemCategory,
            "users" => Table::ItemCategoryRelationship,
            "users" => Table::ItemCategoryUnit,
            "users" => Table::ItemContinuousQuantity,
            "users" => Table::ItemDiscreteQuantity,
            "users" => Table::ItemLocation,
            "users" => Table::ItemUnit,
            "users" => Table::Item,
            "users" => Table::LocationState,
            "users" => Table::Location,
            "users" => Table::LoginProvider,
            "users" => Table::ManufacturedItemCategory,
            "users" => Table::Notification,
            "users" => Table::OrganizationAuthorization,
            "users" => Table::OrganizationLocation,
            "users" => Table::OrganizationState,
            "users" => Table::Organization,
            "users" => Table::PrimaryUserEmail,
            "users" => Table::ProcedureContinuousRequirement,
            "users" => Table::ProcedureDiscreteRequirement,
            "users" => Table::Procedure,
            "users" => Table::ProjectContinuousRequirement,
            "users" => Table::ProjectDiscreteRequirement,
            "users" => Table::ProjectMilestone,
            "users" => Table::ProjectState,
            "users" => Table::Project,
            "users" => Table::Role,
            "users" => Table::SampleTaxa,
            "users" => Table::SampledIndividualTaxa,
            "users" => Table::SampledIndividual,
            "users" => Table::Sample,
            "users" => Table::Spectra,
            "users" => Table::SpectraCollection,
            "users" => Table::Taxa,
            "users" => Table::TeamAuthorization,
            "users" => Table::TeamState,
            "users" => Table::Team,
            "users" => Table::Unit,
            "users" => Table::UserAuthorization,
            "users" => Table::UserEmail,
            "users" => Table::User,
            _ => panic!("Unknown table name"),
        }
    }
}
impl TableRow {
    pub fn table(&self) -> &'static Table {
        match self {
            TableRow::Archivable(_) => &Table::Archivable,
            TableRow::ContainerHorizontalRule(_) => &Table::ContainerHorizontalRule,
            TableRow::ContainerVerticalRule(_) => &Table::ContainerVerticalRule,
            TableRow::ContinuousUnit(_) => &Table::ContinuousUnit,
            TableRow::Describable(_) => &Table::Describable,
            TableRow::DiscreteUnit(_) => &Table::DiscreteUnit,
            TableRow::DocumentFormat(_) => &Table::DocumentFormat,
            TableRow::Document(_) => &Table::Document,
            TableRow::Editable(_) => &Table::Editable,
            TableRow::Edit(_) => &Table::Edit,
            TableRow::ItemCategory(_) => &Table::ItemCategory,
            TableRow::ItemCategoryRelationship(_) => &Table::ItemCategoryRelationship,
            TableRow::ItemCategoryUnit(_) => &Table::ItemCategoryUnit,
            TableRow::ItemContinuousQuantity(_) => &Table::ItemContinuousQuantity,
            TableRow::ItemDiscreteQuantity(_) => &Table::ItemDiscreteQuantity,
            TableRow::ItemLocation(_) => &Table::ItemLocation,
            TableRow::ItemUnit(_) => &Table::ItemUnit,
            TableRow::Item(_) => &Table::Item,
            TableRow::LocationState(_) => &Table::LocationState,
            TableRow::Location(_) => &Table::Location,
            TableRow::LoginProvider(_) => &Table::LoginProvider,
            TableRow::ManufacturedItemCategory(_) => &Table::ManufacturedItemCategory,
            TableRow::Notification(_) => &Table::Notification,
            TableRow::OrganizationAuthorization(_) => &Table::OrganizationAuthorization,
            TableRow::OrganizationLocation(_) => &Table::OrganizationLocation,
            TableRow::OrganizationState(_) => &Table::OrganizationState,
            TableRow::Organization(_) => &Table::Organization,
            TableRow::PrimaryUserEmail(_) => &Table::PrimaryUserEmail,
            TableRow::ProcedureContinuousRequirement(_) => &Table::ProcedureContinuousRequirement,
            TableRow::ProcedureDiscreteRequirement(_) => &Table::ProcedureDiscreteRequirement,
            TableRow::Procedure(_) => &Table::Procedure,
            TableRow::ProjectContinuousRequirement(_) => &Table::ProjectContinuousRequirement,
            TableRow::ProjectDiscreteRequirement(_) => &Table::ProjectDiscreteRequirement,
            TableRow::ProjectMilestone(_) => &Table::ProjectMilestone,
            TableRow::ProjectState(_) => &Table::ProjectState,
            TableRow::Project(_) => &Table::Project,
            TableRow::Role(_) => &Table::Role,
            TableRow::SampleTaxa(_) => &Table::SampleTaxa,
            TableRow::SampledIndividualTaxa(_) => &Table::SampledIndividualTaxa,
            TableRow::SampledIndividual(_) => &Table::SampledIndividual,
            TableRow::Sample(_) => &Table::Sample,
            TableRow::Spectra(_) => &Table::Spectra,
            TableRow::SpectraCollection(_) => &Table::SpectraCollection,
            TableRow::Taxa(_) => &Table::Taxa,
            TableRow::TeamAuthorization(_) => &Table::TeamAuthorization,
            TableRow::TeamState(_) => &Table::TeamState,
            TableRow::Team(_) => &Table::Team,
            TableRow::Unit(_) => &Table::Unit,
            TableRow::UserAuthorization(_) => &Table::UserAuthorization,
            TableRow::UserEmail(_) => &Table::UserEmail,
            TableRow::User(_) => &Table::User,
        }
    }

    pub fn table_name(&self) -> &'static str {
        self.table().name()
    }
}
