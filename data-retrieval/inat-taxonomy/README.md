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
git clone hhttps://github.com/earth-metabolome-initiative/emi-monorepo.git
git branch inat-taxonomy
cd data-retrieval/inat-taxonomy
cargo build --release
```

## Testing

Run the tests using the following command:

```bash
cargo test
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

## Notes for further development

At the moment we have such tables :

id|parent_id|taxon_name
---|---|---
1|48460|Animalia
2|1|Chordata

If insert as such in the DB this is an issue. Indeed parent_id beeing a foreign key to id, we can't have a parent_id that is not in the id column.
One solution would be to decouple the table in two tables :

id|taxon_name
---|---
1|Animalia
2|Chordata

and

id|parent_id
---|---
1|48460
2|1

However, this is not optimal from the performance point of view.
We will thus need to make sure that the table is topologically sorted before inserting it in the DB.
In this case it woul dmean having Life at the very first row etc.

