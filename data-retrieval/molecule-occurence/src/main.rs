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
use tokio::task;
use tokio::time::error::Error;

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

async fn single_task(file_number: usize, molecule_names: &Vec<String>) -> Result<(), Error> {
    // check if output file already exists
    let out_file_name = format!("results/molecule_occurence_{:04}.tsv", file_number);
    if Path::new(&out_file_name).exists() {
        println!("File {} already exists. Skipping...", out_file_name);
        return Ok(());
    }

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

    let mut molecule_counts: Vec<usize> = vec![0; molecule_names.len()];
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
        });

    // we create a directory to store the results if it does not exist
    std::fs::create_dir_all("results").unwrap();

    let mut file = File::create(out_file_name).unwrap();
    for (molecule, count) in molecule_names.iter().zip(molecule_counts.iter()) {
        writeln!(file, "{}\t{}", molecule, count).unwrap();
    }

    // remove the downloaded file
    std::fs::remove_file(&file_name).unwrap();
    std::fs::remove_file(format!("pubmed25n{:04}.xml.gz", file_number)).unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let n_cpus = num_cpus::get() / 2;
    rayon::ThreadPoolBuilder::new()
        .num_threads(n_cpus)
        .build_global()
        .unwrap();
    let molecule_names = read_molecule_names();
    let n_files = 1274;

    let pb = ProgressBar::new(n_files as u64);

    for file_number in 1..=n_files {
        let molecule_names = molecule_names.clone(); // Clone per task
        let pb = pb.clone(); // Clone progress bar
        let _ = single_task(file_number, &molecule_names).await;
        pb.inc(1);
    }

    // now we collect the results
    let mut molecule_counts: Vec<usize> = vec![0; molecule_names.len()];
    for file_number in 1..=n_files {
        let out_file_name = format!("results/molecule_occurence_{:04}.tsv", file_number);
        let file = File::open(out_file_name).unwrap();
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut parts = line.split("\t");
            let molecule = parts.next().unwrap();
            let count = parts.next().unwrap().parse::<usize>().unwrap();
            molecule_counts[i] += count;
        }
    }

    let mut file = File::create("molecule_occurence_final.tsv").unwrap();
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

    #[tokio::test]
    async fn single_task_test() {
        let molecule_names = read_molecule_names();
        let _ = single_task(1, &molecule_names).await.is_ok();
    }
}
