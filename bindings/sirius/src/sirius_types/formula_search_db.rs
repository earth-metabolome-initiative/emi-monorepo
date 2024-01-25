enum FormulaSearchDB {
    Bio,
}

impl<'a> TryFrom<&'a str> for FormulaSearchDB {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "BIO" => Ok(FormulaSearchDB::Bio),
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

impl ToString for FormulaSearchDB {
    fn to_string(&self) -> String {
        match self {
            FormulaSearchDB::Bio => "BIO".to_string(),
        }
    }
}
