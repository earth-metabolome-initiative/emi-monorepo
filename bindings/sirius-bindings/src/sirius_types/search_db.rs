use std::fmt::Display;

/// The databases to search structures and formulas
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SearchDB {
    /// No search db, the default
    #[default]
    None,

    /// The BIO search db
    Bio,

    /// The METACYC search db
    Metacyc,

    /// The CHEBI search db
    Chebi,

    /// The COCONUT search db
    Coconut,

    /// The ECOCYCMINE search db
    Ecocycmine,

    /// The GNPS search db
    Gnps,

    /// The HMDB search db
    Hmdb,

    /// The HSDB search db
    Hsdb,

    /// The KEGG search db
    Kegg,

    /// The KEGGMINE search db
    Keggmine,

    /// The KNAPSACK search db
    Knapsack,

    /// The MACONDA search db
    Maconda,

    /// The MESH search db
    Mesh,

    /// The NORMAN search db
    Norman,

    /// The UNDP search db
    Undp,

    /// The PLANTCYC search db
    Plantcyc,

    /// The PUBCHEM search db
    Pubchem,

    /// The PUBMED search db
    Pubmed,

    /// The YMDB search db
    Ymdb,

    /// The YMDBMINE search db
    Ymdbmine,

    /// The ZINCBIO search db
    Zincbio,
}

impl Display for SearchDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchDB::None => write!(f, "none"),
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
            "none" => Ok(SearchDB::None),
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

/// A vector of databases to search structures and formulas
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DBVector(Vec<SearchDB>);

impl Display for DBVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut atoms = self.0.iter();
        if let Some(atom) = atoms.next() {
            write!(f, "{}", atom)?;
            for atom in atoms {
                write!(f, ",{}", atom)?;
            }
        }
        Ok(())
    }
}

impl From<Vec<SearchDB>> for DBVector {
    fn from(v: Vec<SearchDB>) -> Self {
        DBVector(v)
    }
}

impl<'a> TryFrom<&'a str> for DBVector {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let v = s
            .split(',')
            .map(|db| {
                SearchDB::try_from(db).map_err(|e| {
                    format!(
                        "Cannot parse database: {} ({}). Maybe forgot to put a comma between databases ?",
                        db, e
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DBVector(v))
    }
}

impl TryFrom<String> for DBVector {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        DBVector::try_from(s.as_str())
    }
}
