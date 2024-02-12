/// Structure representing a molecule
///
/// A molecule should always contain an Inchi, Inchikey, smiles, molecular formula, and a mass. The rest will be updated in time.
#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
    /// International Chemical Identifier (InChi)is a textual identifier for chemical substances,
    /// designed to provide a standard way to encode molecular information and to facilitate
    /// the search for such information in databases and on the web.
    pub inchi: String,

    /// The Simplified molecular-input line-entry system (SMILES) is a specification
    /// in the form of a line notation for describing the structure of chemical species using short ASCII strings.    
    pub smiles: String,
}
