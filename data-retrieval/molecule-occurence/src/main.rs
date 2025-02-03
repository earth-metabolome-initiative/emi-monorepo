//! we are trying to read every molecule in the file molecule_names.txt
//! and count the occurence of each molecule in PubMed.
//!
//! Pubmed is split into difference files so we are going to download one by one, parse it and count the occurence of each molecule.

use downloader::{CompressionExtension, Downloader, Task};
use indicatif::{ProgressBar, ProgressIterator};
use rayon::prelude::*;
use std::collections::HashMap;
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

    let file = File::open("pubmed25n0001.xml").unwrap();
    let reader = BufReader::new(file);

    let molecule_names = read_molecule_names();
    let pb = ProgressBar::new(molecule_names.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    // we create a vector of 0 of the same length as the molecule_names
    let mut molecule_counts: Vec<usize> = vec![0; molecule_names.len()];

    molecule_counts
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, count)| {
            let molecule = &molecule_names[i];
            for line in lines.iter() {
                if line.contains(molecule) {
                    *count += 1;
                }
            }
            pb.inc(1);
        });

    // we write the results to a tsv
    let mut file = File::create("molecule_occurence.tsv").unwrap();
    for (molecule, count) in molecule_names.iter().zip(molecule_counts.iter()) {
        writeln!(file, "{}\t{}", molecule, count).unwrap();
    }
}
