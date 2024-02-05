use std::fmt::Display;

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SearchDB {
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

impl Display for SearchDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchDB::Bio => write!(f, "BIO"),
            SearchDB::Metacyc => write!(f, "METACYC"),
            SearchDB::Chebi => write!(f, "CHEBI"),
            SearchDB::Coconut => write!(f, "COCONUT"),
            SearchDB::Ecocycmine => write!(f, "ECOCYCMINE"),
            SearchDB::Gnps => write!(f, "GNPS"),
            SearchDB::Hmdb => write!(f, "HMDB"),
            SearchDB::Hsdb => write!(f, "HSDB"),
            SearchDB::Kegg => write!(f, "KEGG"),
            SearchDB::Keggmine => write!(f, "KEGGMINE"),
            SearchDB::Knapsack => write!(f, "KNAPSACK"),
            SearchDB::Maconda => write!(f, "MACONDA"),
            SearchDB::Mesh => write!(f, "MESH"),
            SearchDB::Norman => write!(f, "NORMAN"),
            SearchDB::Undp => write!(f, "UNDP"),
            SearchDB::Plantcyc => write!(f, "PLANTCYC"),
            SearchDB::Pubchem => write!(f, "PUBCHEM"),
            SearchDB::Pubmed => write!(f, "PUBMED"),
            SearchDB::Ymdb => write!(f, "YMDB"),
            SearchDB::Ymdbmine => write!(f, "YMDBMINE"),
            SearchDB::Zincbio => write!(f, "ZINCBIO"),
        }
    }
}

impl<'a> TryFrom<&'a str> for SearchDB {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "BIO" => Ok(SearchDB::Bio),
            "METACYC" => Ok(SearchDB::Metacyc),
            "CHEBI" => Ok(SearchDB::Chebi),
            "COCONUT" => Ok(SearchDB::Coconut),
            "ECOCYCMINE" => Ok(SearchDB::Ecocycmine),
            "GNPS" => Ok(SearchDB::Gnps),
            "HMDB" => Ok(SearchDB::Hmdb),
            "HSDB" => Ok(SearchDB::Hsdb),
            "KEGG" => Ok(SearchDB::Kegg),
            "KEGGMINE" => Ok(SearchDB::Keggmine),
            "KNAPSACK" => Ok(SearchDB::Knapsack),
            "MACONDA" => Ok(SearchDB::Maconda),
            "MESH" => Ok(SearchDB::Mesh),
            "NORMAN" => Ok(SearchDB::Norman),
            "UNDP" => Ok(SearchDB::Undp),
            "PLANTCYC" => Ok(SearchDB::Plantcyc),
            "PUBCHEM" => Ok(SearchDB::Pubchem),
            "PUBMED" => Ok(SearchDB::Pubmed),
            "YMDB" => Ok(SearchDB::Ymdb),
            "YMDBMINE" => Ok(SearchDB::Ymdbmine),
            "ZINCBIO" => Ok(SearchDB::Zincbio),
            _ => Err(format!("Unknown formula search db: {}", s)),
        }
    }
}

impl TryFrom<String> for SearchDB {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        SearchDB::try_from(s.as_str())
    }
}
