"""Retrieve the species taxonomy from the Open Tree of Life and cleans it up."""

import os
import shutil

import polars as pl
from downloaders import BaseDownloader


def clean_ottl_taxonomy():
    url = "https://files.opentreeoflife.org/ott/ott3.6/ott3.6.tgz"
    BaseDownloader().download(url)
    ottl_lazy = (
        pl.scan_csv(
            source="downloads/ott3.6/ott3.6/taxonomy.tsv", separator="\t"
        )  # load file
        .drop(  # drop useless columns
            [
                "|",
                "|_duplicated_0",
                "|_duplicated_1",
                "|_duplicated_2",
                "|_duplicated_3",
                "|_duplicated_4",
                "|_duplicated_5",
                "",
            ]
        )
        .with_columns(
            pl.col("sourceinfo").str.split(",").alias("source")  # split at the comma
        )
        .with_columns(
            pl.col("source")
            .map_elements(
                lambda x: {i.split(":")[0]: i.split(":")[1] for i in x},
                return_dtype=pl.Object,
            )
            .alias(
                "source_applied"
            )  # split at the colon to crate a dictionary of column_name: value
        )
        .collect()
    )

    ottl_lazy = pl.concat(
        [
            ottl_lazy,
            pl.DataFrame(
                ottl_lazy["source_applied"].to_list()
            ),  # expand the dictionary to create a dataframe and concat
        ],
        how="horizontal",
    )

    out_df = (
        ottl_lazy.lazy()
        .drop(
            ["source", "source_applied", "sourceinfo"]
        )  # drop the columns we don't need
        .cast({"ncbi": pl.Int32, "gbif": pl.Int32})  # cast to int the values
        .filter(pl.col("rank") == "species")  # filter only species
        .collect()
        .sort("name")  # sort by name
    )

    out_df.write_csv("./db_data/ottl_taxonomy_clean.tsv", separator="\t")  # save to csv

    # We delete the downloaded files
    shutil.rmtree("downloads/ott3.6")
    # We delete the compressed downloaded files
    os.remove("downloads/ott3.6.tgz")
