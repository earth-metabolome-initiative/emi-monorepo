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

2. Run the Client:
Update the endpoint, query file, and output file directly in your Rust program:

```rust
let endpoint = "https://qlever.cs.uni-freiburg.de/api/wikidata";
let query_file = "query.rq";
let output_file = "output.csv";

run_sparql_query(endpoint, query_file, output_file).expect("Query failed");
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