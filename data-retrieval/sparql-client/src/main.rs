use sparql_client::run_sparql_query;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = "https://qlever.cs.uni-freiburg.de/api/wikidata";
    let query_file = "query.rq";
    let output_file = "results.csv";

    println!("Running SPARQL query from file: {}", query_file);
    run_sparql_query(endpoint, query_file, output_file)?;

    Ok(())
}
