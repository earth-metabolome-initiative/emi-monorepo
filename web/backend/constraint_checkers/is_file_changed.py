"""Determines whether a python script file or the python script files from which it imports have changed."""

import os
from typing import List
import ast
from functools import lru_cache
import compress_json
from dict_hash import sha256

FILE_HASHED_FILE = "file_hashes.json"

@lru_cache
def get_file_hash(file_path: str) -> str:
    """Determines whether a python script file or the python script files from which it imports have changed.

    Parameters
    ----------
    file_path: str
        The file
    """
    # We normalize the path to the absolute path
    file_path = os.path.abspath(file_path)

    with open(file_path, "r", encoding="utf8") as file:
        file_content = file.read()

    # Parse the document using AST
    tree = ast.parse(file_content)
    # List of the files to be checked, all of which are expected
    # to be of the form "./constraint_checkers/{submodule_name}.py"
    file_names: List[str] = []
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom):
            if node.module.startswith("constraint_checkers"):
                assert node.module.count(".") == 1, (
                    "For the time being, we only expect the module to be of the form "
                    f"'constraint_checkers.{{submodule_name}}'. Got {node.module}."
                )
                node.module = node.module.split(".")[1]
                file_names.append(f"./constraint_checkers/{node.module}.py")

    return sha256({
        "file_path": file_path,
        "file_content": file_content,
        "dependencies": sorted([
            get_file_hash(file_name)
            for file_name in file_names
        ])
    })

def update_all_files_hashes():
    """Updates the hashes of all the files in the constraint_checkers directory."""
    file_hashes = {}
    for root, _, files in os.walk("constraint_checkers"):
        for file in files:
            if file.endswith(".py"):
                file_path = os.path.join(root, file)
                file_path = os.path.abspath(file_path)
                file_hash = get_file_hash(file_path)
                file_hashes[file_path] = file_hash
    print("Updating file hashes at ", FILE_HASHED_FILE)
    compress_json.dump(file_hashes, FILE_HASHED_FILE)

@lru_cache
def is_file_changed(file_path: str) -> bool:
    """Determines whether a python script file or the python script files from which it imports have changed.

    Parameters
    ----------
    file_path: str
        The file
    """

    file_hash = get_file_hash(file_path)

    if os.path.exists(FILE_HASHED_FILE):
        file_hashes = compress_json.load(FILE_HASHED_FILE)
    else:
        file_hashes = {}

    if file_hashes.get(file_path, None) != file_hash:
        print(f"File {file_path.split(os.sep)[-1]} has changed according to hash")
        return True
    
    return False