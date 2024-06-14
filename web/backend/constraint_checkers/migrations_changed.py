"""
Checks whether any of the migrations was changed. This function is primarily meant to
be used to skip steps when they only depend on the migrations and none of the other
files.
"""

import os
from functools import cache
from glob import glob
import compress_json
from dict_hash import sha256
from tqdm.auto import tqdm

@cache
def compute_migrations_hash() -> str:
    """Compute the hash of the migrations."""
    paths = glob("./migrations/**/*.sql", recursive=True)
    paths = sorted(paths)

    current_hash = None

    for path in tqdm(paths, desc="Computing migrations hash", unit="file", leave=False):
        with open(path, "r", encoding="utf8") as file:
            document_hash = sha256({"path": path, "content": file.read()})

        if current_hash is None:
            current_hash = document_hash
        else:
            current_hash = sha256(
                {"current_hash": current_hash, "document_hash": document_hash}
            )

    return current_hash

def update_migrations_hash():
    """Update the migrations hash."""
    migrations_metadata = "./migrations_metadata.json"
    data = {"hash": compute_migrations_hash()}
    print(f"Updating migrations hash {migrations_metadata} to ", data["hash"])
    compress_json.dump(data, migrations_metadata)


@cache
def are_migrations_changed() -> bool:
    """Check whether any of the migrations was changed."""
    migrations_metadata = "./migrations_metadata.json"
    current_hash: str = compute_migrations_hash()

    if os.path.exists(migrations_metadata):
        old_hash = compress_json.load(migrations_metadata)["hash"]
        if old_hash == current_hash:
            return False
        print(f"Migrations were changed: {old_hash} -> {current_hash}")
    return True
