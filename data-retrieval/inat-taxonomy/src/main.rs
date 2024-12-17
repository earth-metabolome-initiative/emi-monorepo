use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use reqwest::blocking::get;
use zip::ZipArchive;
use csv::{ReaderBuilder, WriterBuilder};

// Function to download and extract the dataset
fn download_and_extract() -> io::Result<PathBuf> {
    println!("Fetching the iNaturalist taxonomy dataset...");

    // URL to the dataset
    let url = "https://www.inaturalist.org/taxa/inaturalist-taxonomy.dwca.zip";

    // Define output directory
    let output_dir = env::current_dir()?.join("inat-taxonomy-output");
    std::fs::create_dir_all(&output_dir)?;
    let zip_path = output_dir.join("inat_taxonomy.zip");

    // Step 1: Download the ZIP file
    let response = get(url).expect("Failed to fetch the file");
    let bytes = response.bytes().expect("Failed to read response bytes");
    let mut zip_file = File::create(&zip_path)?;
    zip_file.write_all(&bytes)?;
    println!("Dataset downloaded and saved to {:?}", zip_path);

    // Step 2: Extract the ZIP file
    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    println!("Extracting files...");
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to access file in archive");
        let outpath = output_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Extraction complete. Files are available in {:?}", output_dir);
    Ok(output_dir)
}

// Function to process taxa.csv
fn process_taxa_csv() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::current_dir()?.join("inat-taxonomy-output/taxa.csv");
    let output_path = env::current_dir()?.join("inat-taxonomy-output/extracted_taxa.csv");

    // Open input and output CSV files
    let input_file = File::open(&input_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(input_file);
    let output_file = File::create(&output_path)?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    // Write the header for the new CSV
    wtr.write_record(&["id", "parent_id", "taxon_name"])?;

    // Process each record
    for result in rdr.records() {
        let record = result?;
        let id = record.get(0).unwrap_or("");
        let parent_id_full = record.get(3).unwrap_or("");
        let taxon_name = record.get(13).unwrap_or("");

        // Extract numeric parent_id from the URL
        let parent_id = parent_id_full.rsplit('/').next().unwrap_or("").trim();

        // Write to the new CSV file
        wtr.write_record(&[id, parent_id, taxon_name])?;
    }

    println!("Taxa CSV processing complete. Output saved to {:?}", output_path);
    Ok(())
}

// Main function to dispatch tasks
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run -- download    # Download and extract the dataset");
        println!("  cargo run -- process     # Process taxa.csv and create new CSV");
        return Ok(());
    }

    match args[1].as_str() {
        "download" => {
            download_and_extract()?;
        }
        "process" => {
            process_taxa_csv()?;
        }
        _ => {
            println!("Invalid command. Use 'download' or 'process'.");
        }
    }

    Ok(())
}
