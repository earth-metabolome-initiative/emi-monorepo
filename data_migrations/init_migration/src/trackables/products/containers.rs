//! Submodule defining the commercial products which are containers.

pub mod wet_lab_containers;
pub(crate) use wet_lab_containers::{
    init_avion_interchim_sealed_cap, init_eppendorf_safelock_tube, init_greiner_cct,
    init_machinery_nagel_splitted_cap, init_machinery_nagel_vial, init_vici_schweiz_insert,
};
