from downloaders import BaseDownloader
from tqdm.auto import tqdm
import pandas as pd
from tqdm.auto import tqdm
from cache_decorator import Cache
from pyinaturalist.docs.font_awesome_icons import EXTENDED_FONT_AWESOME_ICONS


def download_taxon_document():
    """Download the taxonomy document from the Open Tree of Life project"""
    url = "https://files.opentreeoflife.org/ott/ott3.6/ott3.6.tgz"
    BaseDownloader().download(url)


@Cache(use_approximated_hash=True)
def load_dataframe():
    """Load the taxonomy document from the Open Tree of Life project"""
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

    df = df.drop(columns=["flags", "silva", "uniqname"])

    return df.set_index("uid", drop=True)


def get_rank(df: pd.DataFrame, row: pd.Series, rank_name: str) -> str:
    """Get the rank of a given row"""
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


def propagate_down(
    df: pd.DataFrame, row: pd.Series, column: str, unknown: str = "unknown"
) -> str:
    """Propagate the value of a column down the tree"""
    if row[column] != "":
        return row[column]

    if pd.isna(row.parent_uid):
        return "unknown"

    parent_row = df.loc[int(row.parent_uid)]

    rank = get_rank(df, parent_row, column)
    df.at[row.name, column] = rank

    return rank


@Cache(use_approximated_hash=True)
def enrich_ranks(df: pd.DataFrame):
    """Enrich the dataframe with the ranks of the species"""
    ranks = ["kingdom", "phylum", "class", "order", "family", "genus", "domain"]

    for rank in ranks:
        df[rank] = ""

    for index, row in tqdm(df.iterrows(), total=df.shape[0], desc=f"Parsing species", leave=False):
        for rank in ranks:
            get_rank(df, row, rank)
    return df


@Cache(use_approximated_hash=True)
def populate_font_awesome_icons(df: pd.DataFrame):
    """Populate the font awesome icons"""
    otol_to_inat = pd.read_csv("./migrations/otol_to_inat.csv").set_index("identifier")

    column_name = "font_awesome_icon"

    df[column_name] = ""

    for inat_identifier in tqdm(
        EXTENDED_FONT_AWESOME_ICONS.keys(), desc="Populating icons",
        leave=False
    ):
        # We lookup the OTT_ID identifier
        try:
            row = otol_to_inat.loc[inat_identifier]
        except KeyError:
            pass

        if pd.notna(row.OTT_ID):
            df.at[row.OTT_ID, column_name] = EXTENDED_FONT_AWESOME_ICONS[
                inat_identifier
            ].name

    for index, row in tqdm(df.iterrows(), total=df.shape[0], desc=f"Parsing species", leave=False):
        propagate_down(df, row, column_name, unknown="fa-question-circle")

    return df

@Cache(use_approximated_hash=True)
def main() -> pd.DataFrame:
    """Main function"""
    print("Downloading the taxonomy document")
    download_taxon_document()
    print("Loading the taxonomy document")
    df = load_dataframe()
    print("Enriching the ranks")
    df = enrich_ranks(df)
    print("Populating the font awesome icons")
    df = populate_font_awesome_icons(df)
    return df


if __name__ == "__main__":
    main()
