//! Test submodule for parsing molecular formulas

use sophia_api::{
    sparql::{SparqlDataset, SparqlResult},
    term::Term,
};
use sophia_sparql_client::SparqlClient;

#[derive(Debug)]
struct CompoundRecord {
    wikidata_qid: String,
    pubchem_cid: String,
    molecular_formula: String,
}

#[test]
fn test_pubchem_formulas() {
    let cli = SparqlClient::new("https://qlever.cs.uni-freiburg.de/api/wikidata");

    let query = r#"
        PREFIX wikibase: <http://wikiba.se/ontology#>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        SELECT ?compound ?pubchemcid ?formula WHERE {
            ?compound wdt:P662 ?pubchemcid;
                      wdt:P274 ?formula.
        }
        LIMIT 100000
    "#;

    if let SparqlResult::Bindings(bindings) = cli.query(query).expect("Query failed") {
        let mut records = Vec::new();

        for b in bindings {
            let b = b.unwrap();

            let wikidata_qid = b[0]
                .as_ref()
                .and_then(|t| t.iri())
                .map(|iri| iri.as_str().to_string()) // clone it early
                .and_then(|s| s.rsplit('/').next().map(|s| s.to_string()))
                .unwrap_or("<no compound>".to_string());


            let pubchem_cid = b[1]
                .as_ref()
                .and_then(|t| t.lexical_form())
                .unwrap_or("NULL".into())
                .to_string();

            let molecular_formula = b[2]
                .as_ref()
                .and_then(|t| t.lexical_form())
                .unwrap_or("NULL".into())
                .to_string();

            records.push(CompoundRecord {
                wikidata_qid,
                pubchem_cid,
                molecular_formula,
            });
        }

        // Print or assert something
        for record in &records {
            println!("{:?}", record);
        }

        // The total number of records
        let total_records = records.len();
        println!("Total records: {}", total_records);
    }
}
