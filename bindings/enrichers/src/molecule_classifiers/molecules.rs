use std::fmt::Display;
pub enum ChemicalIdentifier {
    Smiles,
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
