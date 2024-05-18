from typing import List
from io import StringIO
from downloaders import BaseDownloader
from tqdm.auto import tqdm
import numpy as np
import pandas as pd
from cache_decorator import Cache
from pyinaturalist.docs.font_awesome_icons import EXTENDED_FONT_AWESOME_ICONS
import requests


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


def get_rank(df: pd.DataFrame, row: pd.Series, rank_names: List[str]) -> List[int]:
    """Get the rank of a given row."""
    remaining_rank_names = []
    remaining_rank_indices = []
    results = [np.nan] * len(rank_names)

    for i, rank_name in enumerate(rank_names):
        if row["rank"] == rank_name:
            results[i] = row.name
            continue

        if row[rank_name] != 0.0:
            results[i] = row[rank_name]
            continue

        remaining_rank_names.append(rank_name)
        remaining_rank_indices.append(i)

    if len(remaining_rank_names) == 0 or row.parent_id == row.name:
        return results

    parent_row = df.iloc[row.parent_id]

    ranks = get_rank(df, parent_row, remaining_rank_names)

    for rank, rank_name, i in zip(ranks, remaining_rank_names, remaining_rank_indices):
        df.at[row.name, rank_name] = rank
        results[i] = rank

    return results


def propagate_down(
    df: pd.DataFrame, row: pd.Series, columns: List[str], unknowns: List[int]
) -> List[int]:
    """Propagate the value of a column down the tree"""
    if any(row[column] != df.shape[0] for column in columns):
        return [row[column] for column in columns]

    if row.parent_id == row.name:
        return unknowns

    parent_row = df.iloc[row.parent_id]

    ranks = propagate_down(df, parent_row, columns, unknowns)

    for rank, column in zip(ranks, columns):
        df.at[row.name, column] = rank

    return ranks


@Cache(use_approximated_hash=True)
def enrich_ranks(df: pd.DataFrame):
    """Enrich the dataframe with the ranks of the species"""
    ranks = ["kingdom", "phylum", "class", "order", "family", "genus", "domain"]

    for rank in ranks:
        df[rank] = 0.0

    for index, row in tqdm(
        df.iterrows(), total=df.shape[0], desc=f"Propagating ranks", leave=False
    ):
        get_rank(df, row, ranks)
    return df


@Cache(use_approximated_hash=True)
def populate_font_awesome_icons(df: pd.DataFrame):
    """Populate the font awesome icons"""
    otol_to_inat = pd.read_csv("./db_data/otol_to_inat.csv").set_index("identifier")

    colors = pd.read_csv("./db_data/colors.csv").set_index("name")
    icons = pd.read_csv("./db_data/font_awesome_icons.csv").set_index("name")

    column_name = "font_awesome_icon_id"
    color_column_name = "color_id"

    df[column_name] = df.shape[0]
    df[color_column_name] = df.shape[0]

    df.set_index("uid", inplace=True, drop=True)

    for inat_identifier in tqdm(
        EXTENDED_FONT_AWESOME_ICONS.keys(), desc="Populating icons", leave=False
    ):
        # We lookup the OTT_ID identifier
        try:
            row = otol_to_inat.loc[inat_identifier]
        except KeyError:
            continue

        if pd.notna(row.OTT_ID):
            icon = EXTENDED_FONT_AWESOME_ICONS[inat_identifier]
            # We remove the fa- prefix
            df.at[row.OTT_ID, column_name] = icons.index.get_loc(icon.name[3:])
            df.at[row.OTT_ID, color_column_name] = colors.index.get_loc(icon.color)

    df = df.reset_index()

    unknowns = [icons.index.get_loc("question-circle"), colors.index.get_loc("grey")]

    for _index, row in tqdm(
        df.iterrows(), total=df.shape[0], desc="Propagating icons", leave=False
    ):
        propagate_down(df, row, [column_name, color_column_name], unknowns=unknowns)

    return df


@Cache()
def retrieve_wikidata_to_ott_mapping() -> pd.DataFrame:
    """Retrieve the mapping between Wikidata and OTT

    Implementative details
    ----------------------
    This method executes a sparql query to retrieve the mapping between Wikidata and OTT.

    Returns
    -------
    A dataframe with the following two columns:
    * taxon: the Wikidata item
    * ottid: the OTT identifier
    """

    query = """SELECT ?taxon ?ottid WHERE {
    ?taxon wdt:P31 wd:Q16521 .
    ?taxon wdt:P9157 ?ottid. 
    hint:Prior hint:rangeSafe true.
    }"""
    out = requests.get(
        "https://query.wikidata.org/sparql",
        params={"query": query},
        headers={
            "Accept": "text/csv",
            "Accept-Encoding": "gzip,deflate",
            "User-Agent": "LOTUS project database dumper",
        },
        timeout=70,
    ).text

    df = pd.read_csv(StringIO(out))
    # We convert the url to the identifier
    df["taxon"] = df.taxon.str.rsplit("/", n=1).str[1]
    # We remove the Q prefix
    df["taxon"] = df.taxon.str.replace("Q", "").astype(int)

    return df


@Cache(use_approximated_hash=True)
def add_wikidata_id(df: pd.DataFrame) -> pd.DataFrame:
    """Add the wikidata id to the dataframe"""
    wikidata_to_ott = retrieve_wikidata_to_ott_mapping().set_index("ottid")
    wikidata_to_ott = wikidata_to_ott.sort_index()

    wikidata_ottds: np.ndarray = np.array(wikidata_to_ott.index)
    wikidata_taxa: np.ndarray = np.array(wikidata_to_ott.taxon.values)

    wikidata_ids = np.full(len(df.uid), np.nan)
    uids = np.array(df.uid)

    indices = np.searchsorted(wikidata_ottds, uids)
    mask = indices >= len(wikidata_ottds)
    indices[mask] = 0

    mask = mask | (wikidata_ottds[indices] != uids)

    indices[mask] = 0
    wikidata_ids = wikidata_taxa[indices].astype(float)
    wikidata_ids[mask] = np.nan

    df["wikidata_id"] = wikidata_ids

    return df


@Cache(use_approximated_hash=True)
def retrieve_taxons():
    """Main function"""
    print("Downloading the taxonomy document")
    download_taxon_document()
    print("Loading the taxonomy document")
    df = load_dataframe()

    # We impute the missing parent uid of the life entry with its own uid
    df.loc[df.index == 805080, "parent_uid"] = 805080

    # We replace the parent uid with the number of the row in the dataframe
    # curresponding to the parent uid
    df["parent_id"] = (
        df["parent_uid"].map(lambda x: df.index.get_loc(x)).astype(np.int32)
    )
    df = df.drop(columns=["parent_uid"])
    df = df.reset_index()

    print("Enriching the ranks")
    df = enrich_ranks(df)
    print("Add wikidata identifier")
    df = add_wikidata_id(df)
    print("Populating the font awesome icons")
    df = populate_font_awesome_icons(df)

    # We replace the rank column with the index curresponding to the
    # line in the dataframe 'ranks' that contains the rank
    ranks = pd.read_csv("./db_data/bio_ott_ranks.csv").set_index("name")
    df["ott_rank_id"] = df["rank"].map(lambda x: ranks.index.get_loc(x))

    # We drop the rank column
    df = df.drop(columns=["rank"])

    # Since in SQL the Serial ids do not start from zero but from one,
    # and we populated the ids in the dataframe starting from zero, we
    # need to add one to all the ids.
    for column in [
        "parent_id",
        "color_id",
        "ott_rank_id",
        "font_awesome_icon_id",
        "kingdom",
        "phylum",
        "class",
        "order",
        "family",
        "genus",
        "domain",
    ]:
        df[column] += 1

    # We handle the corner case represented by the "Allocotidus"
    # entry from the irmng dataset. For some reason, while it has
    # always numerical ids, for this entry it has as value "Allocotidus"
    # which is a string. We convert it to NaN.
    df.loc[df.irmng == "Allocotidus", "irmng"] = np.nan

    # We enforce the use of integers even when the column may
    # contain NaN values by switching to object dtype and casting
    # the column to int.
    for column in [
        "ncbi",
        "gbif",
        "irmng",
        "worms",
        "parent_id",
        "wikidata_id",
        "kingdom",
        "phylum",
        "class",
        "order",
        "family",
        "genus",
        "domain",
    ]:
        df[column] = df[column].astype("Int64")

    # In order to avoid collision with SQL terminology, we rename the columns
    # referring to ranks from {rank} to bio_{rank}
    ranks = [
        "kingdom",
        "phylum",
        "class",
        "order",
        "family",
        "genus",
        "domain",
        "worms",
        "irmng",
        "ncbi",
        "gbif",
    ]

    df = df.rename(columns={rank: f"{rank}_id" for rank in ranks})

    # We rename the 'uid' column to 'ott_id'
    df = df.rename(columns={"uid": "ott_id"})

    path = "db_data/bio_ott_taxons.csv.gz"
    print(f"Saving the dataframe to {path}")
    df.to_csv(path, index=False, compression="gzip")


if __name__ == "__main__":
    retrieve_taxons()
