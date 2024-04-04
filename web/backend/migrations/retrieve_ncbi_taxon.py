"""Retrieve the list of NCBI Taxons and writes it out to a file."""

from downloaders import BaseDownloader
import pandas as pd
import os
import shutil


def retrieve_ncbi_taxon():
    """Retrieve the list of NCBI Taxons and writes it out to a file."""
    url = "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/new_taxdump/new_taxdump.tar.gz"
    BaseDownloader().download(url)

    df = pd.read_csv("downloads/new_taxdump/rankedlineage.dmp", sep="\t", header=None)[
        [0, 2]
    ]
    df.columns = ["taxon_id", "taxon_name"]

    # We save the taxons to a TSV file
    df.to_csv("./db_data/taxons.tsv", sep="\t", index=False)
    # We delete the downloaded files
    shutil.rmtree("downloads/new_taxdump")
    # We delete the compressed downloaded files
    os.remove("downloads/new_taxdump.tar.gz")
