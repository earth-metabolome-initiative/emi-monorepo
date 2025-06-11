//! Submodule defining the commercial products which are wet-lab containers.

pub mod conical_centrifugal_tubes;
pub(crate) use conical_centrifugal_tubes::init_greiner_cct;
pub mod eppendorfs;
pub(crate) use eppendorfs::init_eppendorf_safelock_tube;
pub mod vials;
pub(crate) use vials::{
    init_avion_interchim_sealed_cap, init_machinery_nagel_splitted_cap, init_machinery_nagel_vial,
    init_vici_schweiz_insert,
};
