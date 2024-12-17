# inat-taxonomy

inat-taxonomy is a Rust CLI tool that downloads, extracts, and processes the iNaturalist taxonomy dataset. It focuses on producing a simplified CSV containing taxonomic relationships.

## Features

1. Download and Extract
Fetches the iNaturalist taxonomy archive (inat-taxonomy.dwca.zip) and extracts its contents.

2. Process taxa.csv
Generates a simplified CSV file with the following columns:
    - id
    - parent_id (extracted from parentNameUsageID)
    - taxon_name (scientific name)

## Installation

Ensure you have Rust and Cargo installed.

Clone this repository and build the project:

```bash
git clone https://github.com/yourusername/inat-taxonomy.git
cd inat-taxonomy
cargo build --release
```

## Usage

Run the program using the following commands:

1. Download and Extract Dataset:

```bash
cargo run -- download
```

The dataset will be saved and extracted to the inat-taxonomy-output/ directory.

2. Process taxa.csv:

```bash
cargo run -- process
```

This generates extracted_taxa.csv in the inat-taxonomy-output/ directory.

## Output

After processing, the simplified CSV will look like:


id|parent_id|taxon_name
---|---|---
1|48460|Animalia
2|1|Chordata

