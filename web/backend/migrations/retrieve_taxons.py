from downloaders import BaseDownloader
from tqdm.auto import tqdm
import pandas as pd


def download_taxon_document():
    url = "https://files.opentreeoflife.org/ott/ott3.6/ott3.6.tgz"
    BaseDownloader().download(url)


def load_dataframe():
    import polars as pl

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

    df = (
        ottl_lazy.lazy()
        .drop(
            ["source", "source_applied", "sourceinfo"]
        )  # drop the columns we don't need
        .cast({"ncbi": pl.Int32, "gbif": pl.Int32})  # cast to int the values
        # .filter(pl.col("rank") == "species")  # filter only species
        .collect()
        .sort("name")  # sort by name
    ).to_pandas()
    
    df = df.drop(columns=[
        "flags",
        "uniqname"
    ])
    
    return df.set_index("uid", drop=True)


def get_rank(df: pd.DataFrame, row: pd.Series, rank_name: str) -> str:
    if row["rank"] == rank_name:
        return row["name"]
    
    if row[rank_name] != "":
        return row[rank_name]
        
    if pd.isna(row.parent_uid):
        return "unknown"
            
    parent_row = df.loc[int(row.parent_uid)]
    
    rank = get_rank(df, parent_row, rank_name)
    df.at[row.name, rank_name] = rank
    
    return rank

from cache_decorator import Cache

@Cache(use_approximated_hash=True)
def enrich_ranks(df: pd.DataFrame):
    ranks = [
        "kingdom",
        "phylum",
        "class",
        "order",
        "family",
        "genus",
        "domain"
    ]

    for rank in ranks:
        df[rank] = ""

    for index, row in tqdm(
        df.iterrows(),
        total=df.shape[0],
        desc=f"Parsing species"
    ):
        for rank in ranks:
            get_rank(df, row, rank)

if __name__ == "__main__":
    download_taxon_document()
    df = load_dataframe()
    enrich_ranks(df)
    species = df[df["rank"] == "species"]
    species.to_csv("./db_data/taxonomy.csv.gz")