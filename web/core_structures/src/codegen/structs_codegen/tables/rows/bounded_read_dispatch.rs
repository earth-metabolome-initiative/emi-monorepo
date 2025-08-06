impl<C> web_common_traits::prelude::BoundedReadDispatch<C> for super::Rows
where
    crate::codegen::structs_codegen::tables::addresses::Address: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::brands::Brand: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cities::City: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::colors::Color: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_models::ContainerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::containers::Container: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::countries::Country: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::documents::Document: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::email_providers::EmailProvider: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::login_providers::LoginProvider: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::materials::Material: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organisms::Organism: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organizations::Organization: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::phone_models::PhoneModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::processables::Processable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::project_states::ProjectState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ranks::Rank: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::reagents::Reagent: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::roles::Role: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::rooms::Room: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_states::SampleState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra::Spectrum: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::taxa::Taxon: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_members::TeamMember: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_projects::TeamProject: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_states::TeamState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::trackables::Trackable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::units::Unit: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_emails::UserEmail: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::users::User: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn bounded_read(
        table_name: Self::TableName,
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        use web_common_traits::database::BoundedRead;
        match table_name {
            crate::codegen::tables::table_names::TableName::Address => {
                crate::codegen::structs_codegen::tables::addresses::Address::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedureModel => {
                crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedure => {
                crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillMachineModel => {
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedureModel => {
                crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BinaryQuestionProcedureModel => {
                crate::codegen::structs_codegen::tables::binary_question_procedure_models::BinaryQuestionProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Brand => {
                crate::codegen::structs_codegen::tables::brands::Brand::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CameraModel => {
                crate::codegen::structs_codegen::tables::camera_models::CameraModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CappingProcedureModel => {
                crate::codegen::structs_codegen::tables::capping_procedure_models::CappingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeModel => {
                crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedureModel => {
                crate::codegen::structs_codegen::tables::centrifuge_procedure_models::CentrifugeProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::City => {
                crate::codegen::structs_codegen::tables::cities::City::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Color => {
                crate::codegen::structs_codegen::tables::colors::Color::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProductLot => {
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProduct => {
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialReagent => {
                crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CompatibilityRule => {
                crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerModel => {
                crate::codegen::structs_codegen::tables::container_models::ContainerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Container => {
                crate::codegen::structs_codegen::tables::containers::Container::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Country => {
                crate::codegen::structs_codegen::tables::countries::Country::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedureModel => {
                crate::codegen::structs_codegen::tables::disposal_procedure_models::DisposalProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Document => {
                crate::codegen::structs_codegen::tables::documents::Document::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::EmailProvider => {
                crate::codegen::structs_codegen::tables::email_providers::EmailProvider::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedureModel => {
                crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDrierModel => {
                crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedureModel => {
                crate::codegen::structs_codegen::tables::freeze_drying_procedure_models::FreezeDryingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezerModel => {
                crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezingProcedureModel => {
                crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::GeolocationProcedureModel => {
                crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentModel => {
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentState => {
                crate::codegen::structs_codegen::tables::instrument_states::InstrumentState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::LoginProvider => {
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Material => {
                crate::codegen::structs_codegen::tables::materials::Material::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::MixingProcedureModel => {
                crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::NextProcedureModel => {
                crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ObservationSubject => {
                crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismTaxon => {
                crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organism => {
                crate::codegen::structs_codegen::tables::organisms::Organism::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organization => {
                crate::codegen::structs_codegen::tables::organizations::Organization::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedureModel => {
                crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ParentProcedureModel => {
                crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PermanenceCategory => {
                crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhoneModel => {
                crate::codegen::structs_codegen::tables::phone_models::PhoneModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhotographProcedureModel => {
                crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteModel => {
                crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteTipModel => {
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PlacingProcedureModel => {
                crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PositioningDeviceModel => {
                crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedureModel => {
                crate::codegen::structs_codegen::tables::pouring_procedure_models::PouringProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModelTrackable => {
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureModel => {
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTrackable => {
                crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Procedure => {
                crate::codegen::structs_codegen::tables::procedures::Procedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Processable => {
                crate::codegen::structs_codegen::tables::processables::Processable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectState => {
                crate::codegen::structs_codegen::tables::project_states::ProjectState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Project => {
                crate::codegen::structs_codegen::tables::projects::Project::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Rank => {
                crate::codegen::structs_codegen::tables::ranks::Rank::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Reagent => {
                crate::codegen::structs_codegen::tables::reagents::Reagent::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Role => {
                crate::codegen::structs_codegen::tables::roles::Role::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Room => {
                crate::codegen::structs_codegen::tables::rooms::Room::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleState => {
                crate::codegen::structs_codegen::tables::sample_states::SampleState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable => {
                crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpatialRefSy => {
                crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Spectrum => {
                crate::codegen::structs_codegen::tables::spectra::Spectrum::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpectraCollection => {
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageProcedureModel => {
                crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedureModel => {
                crate::codegen::structs_codegen::tables::supernatant_procedure_models::SupernatantProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedure => {
                crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Taxon => {
                crate::codegen::structs_codegen::tables::taxa::Taxon::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamMember => {
                crate::codegen::structs_codegen::tables::team_members::TeamMember::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamProject => {
                crate::codegen::structs_codegen::tables::team_projects::TeamProject::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamState => {
                crate::codegen::structs_codegen::tables::team_states::TeamState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Team => {
                crate::codegen::structs_codegen::tables::teams::Team::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TemporaryUser => {
                crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TrackableLocation => {
                crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Trackable => {
                crate::codegen::structs_codegen::tables::trackables::Trackable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Unit => {
                crate::codegen::structs_codegen::tables::units::Unit::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserEmail => {
                crate::codegen::structs_codegen::tables::user_emails::UserEmail::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserOrganization => {
                crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::User => {
                crate::codegen::structs_codegen::tables::users::User::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricContainerModel => {
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricProcessable => {
                crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingInstrumentModel => {
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedureModel => {
                crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedure => {
                crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
        }
    }
}
