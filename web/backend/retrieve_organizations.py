import multiprocessing

import pandas as pd
import requests
from cache_decorator import Cache
from downloaders import BaseDownloader
from tqdm import tqdm


def download_organisation_json():
    """"""
    url = "https://raw.githubusercontent.com/Hipo/university-domains-list/master/world_universities_and_domains.json"
    BaseDownloader().download(url)


@Cache(use_approximated_hash=True)
def url_ok(url):

    # exception block
    try:

        # pass the url into
        # request.hear
        response = requests.head(url, timeout=5)

        # check the status code
        if response.status_code < 500:
            return True
        else:
            return False
    except requests.ConnectionError as e:
        return False
    except TimeoutError:
        return False
    except requests.exceptions.ReadTimeout:
        return False


@Cache(use_approximated_hash=True)
def get_organisation_domains():
    """The main function to get the organisation domains, filter them and save them to a csv file."""

    # download the json file
    download_organisation_json()

    # read the json file
    df = pd.read_json("./downloads/world_universities_and_domains.json")

    # explode the web_pages column
    df = df.explode(["web_pages"])

    # replace all the http with https
    df["web_pages"] = df["web_pages"].str.replace("http:", "https:")

    with multiprocessing.Pool(processes=64) as pool:
        domain_is_valid = list(
            tqdm(
                pool.imap(url_ok, df["web_pages"], chunksize=100),
                desc="Checking URLs",
                total=len(df["web_pages"]),
            )
        )
    df["url_ok"] = domain_is_valid

    df = df[df.url_ok == True]
    df.reset_index(drop=True, inplace=True)

    shortest_domains = []
    for index, row in df.iterrows():
        # Find the shortest domain for each row
        shortest_domain = min(row["domains"], key=len)
        shortest_domains.append(shortest_domain)

    # Create a new DataFrame with only the shortest domains
    shortest_df = pd.DataFrame({"shortest_domain": shortest_domains})

    # Concatenate the new DataFrame with the original DataFrame to preserve other columns
    result_df = pd.concat([df, shortest_df], axis=1)

    result_df.drop(columns=["domains", "url_ok"], inplace=True)

    # rename the columns
    result_df.rename(
        columns={
            "shortest_domain": "domain",
            "web_pages": "url",
            "state-province": "state_province",
        },
        inplace=True,
    )

    # handle corner cases
    # The country Vietnam has two writings : "Viet Nam" and "Vietnam". We will replace "Vietnam" with "Viet Nam"
    result_df["country"] = result_df["country"].replace("Vietnam", "Viet Nam")

    result_df.drop_duplicates(subset=["domain"], inplace=True)

    # save the result to a csv file
    result_df.to_csv("db_data/organizations.csv.gz", index=False, compression="gzip")


if __name__ == "__main__":
    get_organisation_domains()
