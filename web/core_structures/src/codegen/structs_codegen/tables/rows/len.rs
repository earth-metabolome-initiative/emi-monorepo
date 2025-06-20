impl super::Rows {
    pub fn len(&self) -> usize {
        match self {
            super::Rows::Address(rows) => rows.len(),
            super::Rows::AliquotingProcedureModel(rows) => rows.len(),
            super::Rows::BallMillMachineModel(rows) => rows.len(),
            super::Rows::BallMillProcedureModel(rows) => rows.len(),
            super::Rows::BinaryQuestionProcedureModel(rows) => rows.len(),
            super::Rows::Brand(rows) => rows.len(),
            super::Rows::CameraModel(rows) => rows.len(),
            super::Rows::CappingProcedureModel(rows) => rows.len(),
            super::Rows::CentrifugeModel(rows) => rows.len(),
            super::Rows::CentrifugeProcedureModel(rows) => rows.len(),
            super::Rows::City(rows) => rows.len(),
            super::Rows::Color(rows) => rows.len(),
            super::Rows::CommercialProductLot(rows) => rows.len(),
            super::Rows::CommercialProduct(rows) => rows.len(),
            super::Rows::CommercialReagent(rows) => rows.len(),
            super::Rows::CompatibilityRule(rows) => rows.len(),
            super::Rows::ContainerModel(rows) => rows.len(),
            super::Rows::Container(rows) => rows.len(),
            super::Rows::Country(rows) => rows.len(),
            super::Rows::DisposalProcedureModel(rows) => rows.len(),
            super::Rows::Document(rows) => rows.len(),
            super::Rows::EmailProvider(rows) => rows.len(),
            super::Rows::FractioningProcedureModel(rows) => rows.len(),
            super::Rows::FreezeDrierModel(rows) => rows.len(),
            super::Rows::FreezeDryingProcedureModel(rows) => rows.len(),
            super::Rows::FreezerModel(rows) => rows.len(),
            super::Rows::FreezingProcedureModel(rows) => rows.len(),
            super::Rows::GeolocationProcedureModel(rows) => rows.len(),
            super::Rows::InstrumentModel(rows) => rows.len(),
            super::Rows::InstrumentState(rows) => rows.len(),
            super::Rows::LoginProvider(rows) => rows.len(),
            super::Rows::Material(rows) => rows.len(),
            super::Rows::MixCountableProcedureModel(rows) => rows.len(),
            super::Rows::MixSolidProcedureModel(rows) => rows.len(),
            super::Rows::MountTipProcedureModel(rows) => rows.len(),
            super::Rows::NextProcedureModel(rows) => rows.len(),
            super::Rows::ObservationSubject(rows) => rows.len(),
            super::Rows::OrganismTaxon(rows) => rows.len(),
            super::Rows::Organism(rows) => rows.len(),
            super::Rows::Organization(rows) => rows.len(),
            super::Rows::PackagingProcedureModel(rows) => rows.len(),
            super::Rows::ParentProcedureModel(rows) => rows.len(),
            super::Rows::PermanenceCategory(rows) => rows.len(),
            super::Rows::PhotographProcedureModel(rows) => rows.len(),
            super::Rows::PipetteModel(rows) => rows.len(),
            super::Rows::PipetteTipModel(rows) => rows.len(),
            super::Rows::PositioningDeviceModel(rows) => rows.len(),
            super::Rows::PouringProcedureModel(rows) => rows.len(),
            super::Rows::ProcedureModelTrackable(rows) => rows.len(),
            super::Rows::ProcedureModel(rows) => rows.len(),
            super::Rows::ProcedureTrackable(rows) => rows.len(),
            super::Rows::Procedure(rows) => rows.len(),
            super::Rows::Processable(rows) => rows.len(),
            super::Rows::ProjectState(rows) => rows.len(),
            super::Rows::Project(rows) => rows.len(),
            super::Rows::Rank(rows) => rows.len(),
            super::Rows::Reagent(rows) => rows.len(),
            super::Rows::Role(rows) => rows.len(),
            super::Rows::Room(rows) => rows.len(),
            super::Rows::SampleState(rows) => rows.len(),
            super::Rows::SharedProcedureModelTrackable(rows) => rows.len(),
            super::Rows::SpatialRefSy(rows) => rows.len(),
            super::Rows::Spectrum(rows) => rows.len(),
            super::Rows::SpectraCollection(rows) => rows.len(),
            super::Rows::StorageProcedureModel(rows) => rows.len(),
            super::Rows::SupernatantProcedureModel(rows) => rows.len(),
            super::Rows::Taxon(rows) => rows.len(),
            super::Rows::TeamMember(rows) => rows.len(),
            super::Rows::TeamProject(rows) => rows.len(),
            super::Rows::TeamState(rows) => rows.len(),
            super::Rows::Team(rows) => rows.len(),
            super::Rows::TemporaryUser(rows) => rows.len(),
            super::Rows::TrackableLocation(rows) => rows.len(),
            super::Rows::Trackable(rows) => rows.len(),
            super::Rows::Unit(rows) => rows.len(),
            super::Rows::UserEmail(rows) => rows.len(),
            super::Rows::UserOrganization(rows) => rows.len(),
            super::Rows::User(rows) => rows.len(),
            super::Rows::VolumetricContainerModel(rows) => rows.len(),
            super::Rows::VolumetricProcessable(rows) => rows.len(),
            super::Rows::WeighingInstrumentModel(rows) => rows.len(),
            super::Rows::WeighingProcedureModel(rows) => rows.len(),
            super::Rows::WeighingProcedure(rows) => rows.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
