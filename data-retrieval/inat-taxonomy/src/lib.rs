use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use reqwest::blocking::get;
use zip::ZipArchive;
use csv::{ReaderBuilder, WriterBuilder};

// Download and extract function
pub fn download_and_extract(output_dir: &Path, url: &str) -> io::Result<()> {
    let zip_path = output_dir.join("inat_taxonomy.zip");

    let response = get(url).expect("Failed to fetch the file");
    let bytes = response.bytes().expect("Failed to read response bytes");
    let mut zip_file = File::create(&zip_path)?;
    zip_file.write_all(&bytes)?;

    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;

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
    Ok(())
}

// Process taxa.csv and generate extracted_taxa.csv
pub fn process_taxa_csv(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(&input_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(input_file);
    let output_file = File::create(&output_path)?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    // Write the header for the new CSV
    wtr.write_record(&["id", "parent_id", "taxon_name"])?;

    // Cache column indices from headers
    let headers = rdr.headers()?.clone(); // Clone headers so they're available outside the mutable borrow
    let id_index = headers.iter().position(|h| h == "id").unwrap();
    let parent_id_index = headers.iter().position(|h| h == "parentNameUsageID").unwrap();
    let taxon_name_index = headers.iter().position(|h| h == "scientificName").unwrap();

    // Process each record
    for result in rdr.records() {
        let record = result?;
        let id = record.get(id_index).unwrap_or("");
        let parent_id_full = record.get(parent_id_index).unwrap_or("");
        let taxon_name = record.get(taxon_name_index).unwrap_or("");

        let parent_id = parent_id_full.rsplit('/').next().unwrap_or("").trim();

        wtr.write_record(&[id, parent_id, taxon_name])?;
    }

    println!("Processing complete: {:?}", output_path);
    Ok(())
}
