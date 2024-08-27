from downloaders import BaseDownloader
import zipfile
import os
import pandas as pd
from tqdm.auto import tqdm
import shutil

# Constants
CHEMONT_OBO_URL = "http://classyfire.wishartlab.com/system/downloads/1_0/chemont/ChemOnt_2_1.obo.zip"
DOWNLOAD_DIR = "downloads/chemont"
OBO_ZIP_FILE = os.path.join("downloads", "ChemOnt_2_1.obo.zip")
OBO_FILE = os.path.join("downloads", "ChemOnt_2_1.obo", "ChemOnt_2_1.obo")
OUTPUT_TSV_FILE = "db_data/ChemOnt_2_1.tsv"

# Function to download the OBO file using BaseDownloader
def download_obo_document():
    """Download the ChemOnt OBO document"""
    print("Downloading the ChemOnt OBO file...")
    os.makedirs(DOWNLOAD_DIR, exist_ok=True)
    BaseDownloader().download(CHEMONT_OBO_URL)
    print(f"Downloaded {OBO_ZIP_FILE}")

# Function to extract the OBO file from the ZIP
def extract_obo_file(zip_file: str, target_file: str):
    """Extract the OBO file from the downloaded ZIP"""
    print("Extracting the OBO file from the ZIP...")
    with zipfile.ZipFile(zip_file, "r") as zip_ref:
        zip_ref.extractall(os.path.dirname(target_file))
    print(f"Extracted {target_file}")

# Function to parse the OBO file and return a DataFrame
def parse_obo_file(obo_file: str) -> pd.DataFrame:
    """Parse the OBO file and return a DataFrame"""
    print(f"Parsing the OBO file: {obo_file}...")
    terms = []
    current_term = {}

    with open(obo_file, "r") as file:
        for line in tqdm(file, desc="Reading OBO file lines"):
            line = line.strip()
            if not line:
                continue

            if line == "[Term]":
                if current_term:
                    # Process current term
                    if "synonyms" in current_term:
                        current_term["synonyms"] = str(current_term["synonyms"])
                    if "xrefs" in current_term:
                        current_term["xrefs"] = str(current_term["xrefs"])
                    terms.append(current_term)
                    current_term = {}
            elif line.startswith("id:"):
                current_term["id"] = line.split("id: ")[1]
            elif line.startswith("name:"):
                current_term["name"] = line.split("name: ")[1]
            elif line.startswith("def:"):
                current_term["def"] = line.split("def: ")[1].strip(' "')
            elif line.startswith("synonym:"):
                synonym = line.split("synonym: ")[1].strip(' "')
                current_term.setdefault("synonyms", []).append(synonym)
            elif line.startswith("xref:"):
                xref = line.split("xref: ")[1].strip(' "')
                current_term.setdefault("xrefs", []).append(xref)
            elif line.startswith("is_a:"):
                current_term["is_a"] = line.split("is_a: ")[1].split(" !")[0]

        # Don't forget to append the last term
        if current_term:
            if "synonyms" in current_term:
                current_term["synonyms"] = str(current_term["synonyms"])
            if "xrefs" in current_term:
                current_term["xrefs"] = str(current_term["xrefs"])
            terms.append(current_term)

    # Create DataFrame from the terms list
    df = pd.DataFrame(terms)
    print(f"Parsed {len(df)} terms from the OBO file.")
    return df


# Function to save the DataFrame to a TSV file
def save_to_tsv(df: pd.DataFrame, output_file: str):
    """Save the parsed data to a TSV file"""
    print(f"Saving the parsed data to {output_file}...")
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    df.to_csv(output_file, sep="\t", index=False)
    print(f"Saved TSV to {output_file}")

def clean_up():
    """Clean up the downloads directory after processing"""
    print(f"Cleaning up download directory: {DOWNLOAD_DIR}")
    shutil.rmtree(DOWNLOAD_DIR)

def main():
    # Download the ChemOnt OBO file using BaseDownloader
    download_obo_document()
    
    # Extract the OBO file from the ZIP
    extract_obo_file(OBO_ZIP_FILE, OBO_FILE)
    
    # Parse the OBO file
    chemont_df = parse_obo_file(OBO_FILE)
    
    # Save the parsed data to a TSV file
    save_to_tsv(chemont_df, OUTPUT_TSV_FILE)
    
    # Clean up the download directory
    clean_up()

if __name__ == "__main__":
    main()
