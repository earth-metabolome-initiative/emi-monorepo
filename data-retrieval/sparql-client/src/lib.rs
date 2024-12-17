use crate::errors::SparqlClientError;
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashSet;
use urlencoding::encode;

pub mod errors;

pub fn run_sparql_query(endpoint: &str, query_file: &str, output_file: &str) -> Result<(), SparqlClientError> {
    // Step 1: Check if query file exists
    if !Path::new(query_file).exists() {
        return Err(SparqlClientError::QueryFileNotFound(query_file.to_string()));
    }

    // Step 2: Read the query file content
    let query = fs::read_to_string(query_file)?;
    if query.trim().is_empty() {
        return Err(SparqlClientError::QueryFileEmpty(query_file.to_string()));
    }

    // Step 3: Encode query for URL
    let encoded_query = encode(&query);
    let url = format!("{}?query={}&format=json", endpoint, encoded_query);

    println!("Executing SPARQL query...");

    // Step 4: Send the request
    let client = Client::new();
    let response = client.get(&url)
        .timeout(std::time::Duration::from_secs(30))
        .send()?;
    
    if !response.status().is_success() {
        return Err(SparqlClientError::InvalidResponseFormat(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    // Step 5: Parse JSON response
    let json: Value = response.json()
        .map_err(|_| SparqlClientError::InvalidResponseFormat("Failed to parse JSON.".to_string()))?;

    let results = json["results"]["bindings"]
        .as_array()
        .ok_or(SparqlClientError::InvalidResponseFormat(
            "Response missing 'bindings'.".to_string(),
        ))?;

    if results.is_empty() {
        println!("Warning: Query returned no results.");
        return Ok(());
    }

    // Step 6: Dynamically determine headers
    let mut headers = HashSet::new();
    for row in results {
        for key in row.as_object().unwrap().keys() {
            headers.insert(key.clone());
        }
    }
    let mut headers: Vec<String> = headers.into_iter().collect();
    headers.sort(); // Optional: Sort headers alphabetically

    // Step 7: Write results to CSV dynamically
    let mut wtr = csv::Writer::from_path(output_file)
        .map_err(|_| SparqlClientError::CsvWriteError("Failed to create output CSV.".to_string()))?;
    wtr.write_record(&headers)?;

    for row in results {
        let mut record = vec![];
        for header in &headers {
            let value = row.get(header)
                .and_then(|v| v.get("value"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            record.push(value.to_string());
        }
        wtr.write_record(&record)
            .map_err(|_| SparqlClientError::CsvWriteError("Failed to write row to CSV.".to_string()))?;
    }

    wtr.flush().map_err(|_| SparqlClientError::CsvWriteError("Failed to flush CSV.".to_string()))?;

    println!("Results written to '{}'.", output_file);
    Ok(())
}
