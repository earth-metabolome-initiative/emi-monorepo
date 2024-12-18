use std::env;
use std::path::PathBuf;
use taxonomy_fetcher::{download_and_extract, process_taxa_csv};

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_dir = env::current_dir().unwrap().join("inat-taxonomy-output");

    match args.get(1).map(|s| s.as_str()) {
        Some("download") => {
            let url = "https://www.inaturalist.org/taxa/inaturalist-taxonomy.dwca.zip";
            download_and_extract(&output_dir, url).expect("Failed to download and extract dataset.");
        }
        Some("process") => {
            let input_path = output_dir.join("taxa.csv");
            let output_path = output_dir.join("extracted_taxa.csv");
            process_taxa_csv(&input_path, &output_path).expect("Failed to process taxa.csv.");
        }
        _ => println!("Usage: cargo run -- [download | process]"),
    }
}
