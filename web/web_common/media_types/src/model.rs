//! Submodule providing the `model` media type and its subtypes. This module is generated from the IANA registry at `https://www.iana.org/assignments/media-types/model.csv`.
/// The `3mf` subtype of the `model` media type.
pub const MT_3MF: &str = "3mf";
/// The `e57` subtype of the `model` media type.
pub const E57: &str = "e57";
/// The `example` subtype of the `model` media type.
pub const EXAMPLE: &str = "example";
/// The `gltf-binary` subtype of the `model` media type.
pub const GLTF_BINARY: &str = "gltf-binary";
/// The `gltf+json` subtype of the `model` media type.
pub const GLTF_JSON: &str = "gltf+json";
/// The `JT` subtype of the `model` media type.
pub const JT: &str = "JT";
/// The `iges` subtype of the `model` media type.
pub const IGES: &str = "iges";
/// The `mesh` subtype of the `model` media type.
pub const MESH: &str = "mesh";
/// The `mtl` subtype of the `model` media type.
pub const MTL: &str = "mtl";
/// The `obj` subtype of the `model` media type.
pub const OBJ: &str = "obj";
/// The `prc` subtype of the `model` media type.
pub const PRC: &str = "prc";
/// The `step` subtype of the `model` media type.
pub const STEP: &str = "step";
/// The `step+xml` subtype of the `model` media type.
pub const STEP_XML: &str = "step+xml";
/// The `step+zip` subtype of the `model` media type.
pub const STEP_ZIP: &str = "step+zip";
/// The `step-xml+zip` subtype of the `model` media type.
pub const STEP_XML_ZIP: &str = "step-xml+zip";
/// The `stl` subtype of the `model` media type.
pub const STL: &str = "stl";
/// The `u3d` subtype of the `model` media type.
pub const U3D: &str = "u3d";
/// The `vnd.bary` subtype of the `model` media type.
pub const VND_BARY: &str = "vnd.bary";
/// The `vnd.cld` subtype of the `model` media type.
pub const VND_CLD: &str = "vnd.cld";
/// The `vnd.collada+xml` subtype of the `model` media type.
pub const VND_COLLADA_XML: &str = "vnd.collada+xml";
/// The `vnd.dwf` subtype of the `model` media type.
pub const VND_DWF: &str = "vnd.dwf";
/// The `vnd.flatland.3dml` subtype of the `model` media type.
pub const VND_FLATLAND_3DML: &str = "vnd.flatland.3dml";
/// The `vnd.gdl` subtype of the `model` media type.
pub const VND_GDL: &str = "vnd.gdl";
/// The `vnd.gs-gdl` subtype of the `model` media type.
pub const VND_GS_GDL: &str = "vnd.gs-gdl";
/// The `vnd.gtw` subtype of the `model` media type.
pub const VND_GTW: &str = "vnd.gtw";
/// The `vnd.moml+xml` subtype of the `model` media type.
pub const VND_MOML_XML: &str = "vnd.moml+xml";
/// The `vnd.mts` subtype of the `model` media type.
pub const VND_MTS: &str = "vnd.mts";
/// The `vnd.opengex` subtype of the `model` media type.
pub const VND_OPENGEX: &str = "vnd.opengex";
/// The `vnd.parasolid.transmit.binary` subtype of the `model` media type.
pub const VND_PARASOLID_TRANSMIT_BINARY: &str = "vnd.parasolid.transmit.binary";
/// The `vnd.parasolid.transmit.text` subtype of the `model` media type.
pub const VND_PARASOLID_TRANSMIT_TEXT: &str = "vnd.parasolid.transmit.text";
/// The `vnd.pytha.pyox` subtype of the `model` media type.
pub const VND_PYTHA_PYOX: &str = "vnd.pytha.pyox";
/// The `vnd.rosette.annotated-data-model` subtype of the `model` media type.
pub const VND_ROSETTE_ANNOTATED_DATA_MODEL: &str = "vnd.rosette.annotated-data-model";
/// The `vnd.sap.vds` subtype of the `model` media type.
pub const VND_SAP_VDS: &str = "vnd.sap.vds";
/// The `vnd.usda` subtype of the `model` media type.
pub const VND_USDA: &str = "vnd.usda";
/// The `vnd.usdz+zip` subtype of the `model` media type.
pub const VND_USDZ_ZIP: &str = "vnd.usdz+zip";
/// The `vnd.valve.source.compiled-map` subtype of the `model` media type.
pub const VND_VALVE_SOURCE_COMPILED_MAP: &str = "vnd.valve.source.compiled-map";
/// The `vnd.vtu` subtype of the `model` media type.
pub const VND_VTU: &str = "vnd.vtu";
/// The `vrml` subtype of the `model` media type.
pub const VRML: &str = "vrml";
/// The `x3d-vrml` subtype of the `model` media type.
pub const X3D_VRML: &str = "x3d-vrml";
/// The `x3d+fastinfoset` subtype of the `model` media type.
pub const X3D_FASTINFOSET: &str = "x3d+fastinfoset";
/// The `x3d+xml` subtype of the `model` media type.
pub const X3D_XML: &str = "x3d+xml";
/// Set of all subtypes of the `model` media type. This set is generated from the IANA registry at `https://www.iana.org/assignments/media-types/model.csv`.
pub static SUBTYPES: ::phf::Set<&'static str> = ::phf::phf_set! { "3mf" , "e57" , "example" , "gltf-binary" , "gltf+json" , "JT" , "iges" , "mesh" , "mtl" , "obj" , "prc" , "step" , "step+xml" , "step+zip" , "step-xml+zip" , "stl" , "u3d" , "vnd.bary" , "vnd.cld" , "vnd.collada+xml" , "vnd.dwf" , "vnd.flatland.3dml" , "vnd.gdl" , "vnd.gs-gdl" , "vnd.gtw" , "vnd.moml+xml" , "vnd.mts" , "vnd.opengex" , "vnd.parasolid.transmit.binary" , "vnd.parasolid.transmit.text" , "vnd.pytha.pyox" , "vnd.rosette.annotated-data-model" , "vnd.sap.vds" , "vnd.usda" , "vnd.usdz+zip" , "vnd.valve.source.compiled-map" , "vnd.vtu" , "vrml" , "x3d-vrml" , "x3d+fastinfoset" , "x3d+xml" };
