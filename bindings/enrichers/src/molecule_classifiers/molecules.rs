use std::fmt::Display;

/// An enum for the different chemical identifiers
pub enum ChemicalIdentifier {
    /// The Simplified molecular-input line-entry system (SMILES) is a specification
    /// in the form of a line notation for describing the structure of chemical species using short ASCII strings.
    Smiles,

    /// International Chemical Identifier (InChi)is a textual identifier for chemical substances,
    /// designed to provide a standard way to encode molecular information and to facilitate
    /// the search for such information in databases and on the web.
    Inchi,
}

impl Display for ChemicalIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChemicalIdentifier::Smiles => write!(f, "smiles"),
            ChemicalIdentifier::Inchi => write!(f, "inchi"),
        }
    }
}
