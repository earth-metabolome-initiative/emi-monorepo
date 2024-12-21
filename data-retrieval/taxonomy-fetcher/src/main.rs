use std::env;
use std::path::PathBuf;
use taxonomy_fetcher::{process_taxa_csv, helpers::*};

const DEFAULT_URL: &str = "https://www.inaturalist.org/taxa/inaturalist-taxonomy.dwca.zip";
const DEFAULT_OUTPUT_DIR: &str = "taxonomy-output";

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- download [options]    Download and extract taxonomy");
    println!("  cargo run -- process [options]     Process the downloaded taxa.csv");
    println!("\nOptions:");
    println!("  --url <url>          Taxonomy download URL (default: {})", DEFAULT_URL);
    println!("  --output-dir <path>  Output directory (default: {})", DEFAULT_OUTPUT_DIR);
    println!("\nExamples:");
    println!("  cargo run -- download --url https://example.com/taxonomy.zip --output-dir ./my-data");
    println!("  cargo run -- process --output-dir ./my-data");
}

fn parse_args(args: &[String]) -> Result<(String, PathBuf), String> {
    let mut url = DEFAULT_URL.to_string();
    let mut output_dir = PathBuf::from(DEFAULT_OUTPUT_DIR);
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--url" => {
                if let Some(val) = args.get(i + 1) {
                    url = val.to_string();
                    i += 2;
                } else {
                    return Err("Missing value for --url".to_string());
                }
            }
            "--output-dir" => {
                if let Some(val) = args.get(i + 1) {
                    output_dir = PathBuf::from(val);
                    i += 2;
                } else {
                    return Err("Missing value for --output-dir".to_string());
                }
            }
            _ => i += 1,
        }
    }

    Ok((url, output_dir))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "download" => {
            match parse_args(&args[2..]) {
                Ok((url, output_dir)) => {
                    println!("Downloading taxonomy from: {}", url);
                    println!("Output directory: {:?}", output_dir);
                    
                    let config = DownloadConfig::new(&url, &output_dir);
                    
                    match download_and_extract(&config) {
                        Ok(extract_path) => println!("Successfully downloaded and extracted to {:?}", extract_path),
                        Err(e) => eprintln!("Failed to download and extract dataset: {:?}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing arguments: {}", e);
                    print_usage();
                }
            }
        }
        "process" => {
            match parse_args(&args[2..]) {
                Ok((_, output_dir)) => {
                    let input_path = output_dir.join("taxa.csv");
                    let output_path = output_dir.join("extracted_taxa.csv");
                    
                    if let Err(e) = process_taxa_csv(&input_path, &output_path) {
                        eprintln!("Failed to process taxa.csv: {}", e);
                    } else {
                        println!("Successfully processed taxa.csv");
                        println!("Output file: {:?}", output_path);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing arguments: {}", e);
                    print_usage();
                }
            }
        }
        _ => print_usage(),
    }
}