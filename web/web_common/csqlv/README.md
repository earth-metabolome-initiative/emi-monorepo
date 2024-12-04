# CSQLV

The CSQLV crate has to goal to easily integrate CSVs files into your PostgreSQL database.

## Installation

As usual, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
csqlv = "0.1"
```

## Usage

Given a directory containing CSV files, such as `./tests/bands`, you can create the SQL necessary to define the tables and relationships between them as follows:

```rust
use csqlv::{CSVSchemaBuilder, CSVSchema};

let schema: CSVSchema = CSVSchemaBuilder::default()
    // To show a loading bar while processing the CSVs
    .verbose()
    // To include compressed files such as .gz
    .include_gz()
    // For supporting running the tests within
    // containers such as Docker
    .container_directory("/app/bands")
    .from_dir("./tests/bands")
    .unwrap();

let sql: String = schema.to_postgres();

println!("{}", sql);
```

See below a detailed breakdown of the syntax used in the CSVs used in the example above.

## Syntax of the CSV files

If I have a table called `players` with a column `name` and a table called `bands` with a column `name`, I may want to define their relationships via a third table called `band_members` as follows:

So, if I have a CSV file called `players.csv` with the following content:

| name  | age |
|-------|-----|
| Paul  | 20  |
| John  | 30  |
| Ringo | 25  |
| Kurt  | 27  |

And a CSV file called `bands.csv` with the following content:

| band        | foundation_year | founded_by:players.name |
|-------------|-----------------|-------------------------|
| The Beatles | 1960            | Paul                    |
| Nirvana     | 1987            | Kurt                    |

I can define the relationships between the tables in the `band_members.csv` file as follows:

| bands.band  | players.name |
|-------------|--------------|
| The Beatles | Paul         |
| The Beatles | John         |
| The Beatles | Ringo        |
| Nirvana     | Kurt         |

This will generate SQL defining the following tables:

```sql
CREATE TABLE IF NOT EXISTS players (
    name TEXT UNIQUE NOT NULL,
    age SMALLINT UNIQUE NOT NULL,
    id SERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE players_temp (
    name TEXT UNIQUE NOT NULL,
    age SMALLINT UNIQUE NOT NULL
);

COPY players_temp FROM '/app/csvs/bands/players.csv' DELIMITER ',' CSV HEADER;

INSERT INTO players (
    name,
    age
) SELECT
    players_temp.name,
    players_temp.age
FROM
    players_temp;

DROP TABLE players_temp;

CREATE TABLE IF NOT EXISTS bands (
    band TEXT UNIQUE NOT NULL,
    foundation_year SMALLINT UNIQUE NOT NULL,
    founded_by SERIAL UNIQUE NOT NULL REFERENCES players(id),
    id SERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE bands_temp (
    band TEXT UNIQUE NOT NULL,
    foundation_year SMALLINT UNIQUE NOT NULL,
    players_name VARCHAR(4)
);

COPY bands_temp FROM '/app/csvs/bands/bands.csv' DELIMITER ',' CSV HEADER;

INSERT INTO bands (
    band,
    foundation_year,
    founded_by
) SELECT
    bands_temp.band,
    bands_temp.foundation_year,
    players.id
FROM
    bands_temp
    JOIN players ON bands_temp.players_name = players.name;

DROP TABLE bands_temp;

CREATE TABLE IF NOT EXISTS band_members (
    bands_id SERIAL NOT NULL REFERENCES bands(id),
    players_id SERIAL UNIQUE NOT NULL REFERENCES players(id),
    id SERIAL PRIMARY KEY UNIQUE NOT NULL
);
CREATE TEMPORARY TABLE band_members_temp (
    bands_band TEXT,
    players_name TEXT
);

COPY band_members_temp FROM '/app/csvs/bands/band_members.csv' DELIMITER ',' CSV HEADER;

INSERT INTO band_members (
    bands_id,
    players_id
) SELECT
    bands.id,
    players.id
FROM
    band_members_temp
    JOIN bands ON band_members_temp.bands_band = bands.band
    JOIN players ON band_members_temp.players_name = players.name;

DROP TABLE band_members_temp;
```


