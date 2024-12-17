use sparql_client::run_sparql_query;
use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "SPARQL Client", about = "Execute SPARQL queries and save results to CSV.")]
struct Cli {
    /// The SPARQL endpoint URL
    #[arg(short, long)]
    endpoint: String,

    /// Path to the SPARQL query file (.rq)
    #[arg(short, long)]
    query_file: String,

    /// Path to save the output CSV file
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Cli::parse();

    println!("Running query from file: {}", args.query_file);
    println!("Saving results to: {}", args.output_file);

    if let Err(err) = run_sparql_query(&args.endpoint, &args.query_file, &args.output_file) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    println!("Query executed successfully, results saved to {}", args.output_file);
}
