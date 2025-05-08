//! Test submodule for parsing molecular formulas

use sophia_api::{
    sparql::{SparqlDataset, SparqlResult},
    term::Term,
};
use sophia_sparql_client::SparqlClient;

#[test]
/// Test on formulas retrieved from PubChem
fn test_pubchem_formulas() {
    let cli = SparqlClient::new("https://qlever.cs.uni-freiburg.de/api/wikidata/");

    let query = r#"
        #Molecular Formula retrieval
        PREFIX wikibase: <http://wikiba.se/ontology#>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        SELECT ?compound ?pubchemCID ?formula WHERE {
        ?compound wdt:P662 ?pubchemCID;       # P662 = PubChem CID
                    wdt:P274 ?formula.           # P274 = chemical formula (molecular formula)

        }
        LIMIT 10
    "#;

    if let SparqlResult::Bindings(bindings) = cli.query(query).expect("Query failed") {
        for b in bindings {
            let b = b.unwrap();
            let compound = b[1].as_ref().and_then(|t| t.lexical_form()).unwrap();
            let pubchemCID = b[4].as_ref().and_then(|t| t.lexical_form()).unwrap_or("NULL".into());
            println!("{}\t{}", compound, pubchemCID);
        }
    }
}
