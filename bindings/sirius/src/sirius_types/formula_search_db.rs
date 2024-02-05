use std::fmt::Display;

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FormulaSearchDB {
    #[default]
    Bio,
    Metacyc,
    Chebi,
    Coconut,
    Ecocycmine,
    Gnps,
    Hmdb,
    Hsdb,
    Kegg,
    Keggmine,
    Knapsack,
    Maconda,
    Mesh,
    Norman,
    Undp,
    Plantcyc,
    Pubchem,
    Pubmed,
    Ymdb,
    Ymdbmine,
    Zincbio,
}

impl Display for FormulaSearchDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaSearchDB::Bio => write!(f, "BIO"),
            FormulaSearchDB::Metacyc => write!(f, "METACYC"),
            FormulaSearchDB::Chebi => write!(f, "CHEBI"),
            FormulaSearchDB::Coconut => write!(f, "COCONUT"),
            FormulaSearchDB::Ecocycmine => write!(f, "ECOCYCMINE"),
            FormulaSearchDB::Gnps => write!(f, "GNPS"),
            FormulaSearchDB::Hmdb => write!(f, "HMDB"),
            FormulaSearchDB::Hsdb => write!(f, "HSDB"),
            FormulaSearchDB::Kegg => write!(f, "KEGG"),
            FormulaSearchDB::Keggmine => write!(f, "KEGGMINE"),
            FormulaSearchDB::Knapsack => write!(f, "KNAPSACK"),
            FormulaSearchDB::Maconda => write!(f, "MACONDA"),
            FormulaSearchDB::Mesh => write!(f, "MESH"),
            FormulaSearchDB::Norman => write!(f, "NORMAN"),
            FormulaSearchDB::Undp => write!(f, "UNDP"),
            FormulaSearchDB::Plantcyc => write!(f, "PLANTCYC"),
            FormulaSearchDB::Pubchem => write!(f, "PUBCHEM"),
            FormulaSearchDB::Pubmed => write!(f, "PUBMED"),
            FormulaSearchDB::Ymdb => write!(f, "YMDB"),
            FormulaSearchDB::Ymdbmine => write!(f, "YMDBMINE"),
            FormulaSearchDB::Zincbio => write!(f, "ZINCBIO"),
        }
    }
}

impl<'a> TryFrom<&'a str> for FormulaSearchDB {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "BIO" => Ok(FormulaSearchDB::Bio),
            "METACYC" => Ok(FormulaSearchDB::Metacyc),
            "CHEBI" => Ok(FormulaSearchDB::Chebi),
            "COCONUT" => Ok(FormulaSearchDB::Coconut),
            "ECOCYCMINE" => Ok(FormulaSearchDB::Ecocycmine),
            "GNPS" => Ok(FormulaSearchDB::Gnps),
            "HMDB" => Ok(FormulaSearchDB::Hmdb),
            "HSDB" => Ok(FormulaSearchDB::Hsdb),
            "KEGG" => Ok(FormulaSearchDB::Kegg),
            "KEGGMINE" => Ok(FormulaSearchDB::Keggmine),
            "KNAPSACK" => Ok(FormulaSearchDB::Knapsack),
            "MACONDA" => Ok(FormulaSearchDB::Maconda),
            "MESH" => Ok(FormulaSearchDB::Mesh),
            "NORMAN" => Ok(FormulaSearchDB::Norman),
            "UNDP" => Ok(FormulaSearchDB::Undp),
            "PLANTCYC" => Ok(FormulaSearchDB::Plantcyc),
            "PUBCHEM" => Ok(FormulaSearchDB::Pubchem),
            "PUBMED" => Ok(FormulaSearchDB::Pubmed),
            "YMDB" => Ok(FormulaSearchDB::Ymdb),
            "YMDBMINE" => Ok(FormulaSearchDB::Ymdbmine),
            "ZINCBIO" => Ok(FormulaSearchDB::Zincbio),
            _ => Err(format!("Unknown formula search db: {}", s)),
        }
    }
}

impl TryFrom<String> for FormulaSearchDB {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        FormulaSearchDB::try_from(s.as_str())
    }
}
