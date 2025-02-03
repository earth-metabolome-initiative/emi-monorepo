//! we are trying to read every molecule in the file molecule_names.txt
//! and count the occurence of each molecule in PubMed.
//!
//! Pubmed is split into difference files so we are going to download one by one, parse it and count the occurence of each molecule.

use downloader::{Downloader, Task};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

const PUBMED_URL: &'static str =
    "https://ftp.ncbi.nlm.nih.gov/pubmed/baseline/pubmed25n0001.xml.gz";

fn read_molecule_names() -> Vec<String> {
    let path = Path::new("molecule_names.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut molecule_names: Vec<String> = Vec::new();
    for line in reader.lines() {
        molecule_names.push(line.unwrap());
    }

    molecule_names
}

async fn single_task(file_number: usize, molecule_names: &Vec<String>) {
    let url = get_pubmed_url_from_counter(file_number);
    let task: Downloader = Downloader::default()
        .extract()
        .show_loading_bar()
        .task(
            Task::try_from(url)
                .unwrap()
                .target_path(&format!("pubmed25n{:04}.xml.gz", file_number)),
        )
        .unwrap();
    task.execute().await.unwrap();

    let file_name = format!("pubmed25n{:04}.xml", file_number);
    let text = std::fs::read_to_string(&file_name).unwrap();
}

#[tokio::main]
async fn main() {
    let task: Downloader = Downloader::default()
        .extract()
        .show_loading_bar()
        .task(
            Task::try_from(PUBMED_URL)
                .unwrap()
                .target_path(&"pubmed25n0001.xml.gz"),
        )
        .unwrap();
    task.execute().await.unwrap();

    let text = std::fs::read_to_string("pubmed25n0001.xml").unwrap();

    let molecule_names = read_molecule_names();
    let pb = ProgressBar::new(molecule_names.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // we create a vector of 0 of the same length as the molecule_names
    let mut molecule_counts: Vec<usize> = vec![0; molecule_names.len()];

    // we create the regex for each molecule
    let res = molecule_names
        .iter()
        .map(|molecule| regex::Regex::new(regex::escape(&molecule.as_str()).as_str()).unwrap())
        .collect::<Vec<regex::Regex>>();

    molecule_counts
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, count)| {
            let re = &res[i];
            let res = re.find_iter(&text).map(|m| m.as_str()).count();
            *count = res;
            pb.inc(1);
        });

    // we write the results to a tsv
    let mut file = File::create("molecule_occurence.tsv").unwrap();
    for (molecule, count) in molecule_names.iter().zip(molecule_counts.iter()) {
        writeln!(file, "{}\t{}", molecule, count).unwrap();
    }
}

fn get_pubmed_url_from_counter(counter: usize) -> String {
    format!(
        "https://ftp.ncbi.nlm.nih.gov/pubmed/baseline/pubmed25n{:04}.xml.gz",
        counter
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pubmed_url_from_counter() {
        let url = get_pubmed_url_from_counter(27);
        assert_eq!(
            url,
            "https://ftp.ncbi.nlm.nih.gov/pubmed/baseline/pubmed25n0027.xml.gz"
        );

        assert_eq!(
            get_pubmed_url_from_counter(352),
            "https://ftp.ncbi.nlm.nih.gov/pubmed/baseline/pubmed25n0352.xml.gz"
        )
    }
}
