# sparql-client

A Rust-based SPARQL client to execute queries and save results as a CSV file.

## Features

Executes SPARQL queries on any endpoint.
Reads queries from a file (e.g., query.rq).
Outputs results dynamically to a CSV file.

## Usage

1. Prepare Your Query File:
    Save your SPARQL query in query.rq. Example:

```sql
PREFIX wd: <http://www.wikidata.org/entity/>
SELECT ?item WHERE { ?item wdt:P31 wd:Q5. } LIMIT 10
```

2. Run the Client with CLI
Use the CLI to specify the endpoint, query file, and output file:

```bash
cargo run -- --endpoint "https://qlever.cs.uni-freiburg.de/api/wikidata" \
            --query-file "query.rq" \
            --output-file "output.csv"
```

3. Output:

Results are saved in output.csv with headers automatically inferred from the query.

## Testing

Run tests with:

```bash
cargo test
```

Test data is available under tests/data/.

## Requirements

Rust and Cargo installed.
A SPARQL endpoint.