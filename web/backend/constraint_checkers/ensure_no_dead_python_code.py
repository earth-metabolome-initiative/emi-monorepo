"""Submodule for ensuring that there is no dead Python code in the codebase.

Implementation details
----------------------
This module seeks all of the *.py files in the backend directory, finds all of the defined functions,
and checks if they are used anywhere in the codebase. If they are not used, then the function is considered dead code
and an appropriate error message is raised.
"""
from glob import glob
from typing import List
import ast
from tqdm.auto import tqdm

ALLOW_LIST = [
    "url_ok"
]

class DeadCodeException(Exception):
    """Exception raised when dead code is found in the codebase."""

def get_all_defined_functions_in_file(file_path: str) -> List[str]:
    """Get all of the defined functions in a Python file.

    Args:
        file_path (str): The path to the Python file.

    Returns:
        List[str]: A list of all of the defined functions in the file.
    """
    with open(file_path, "r", encoding="utf8") as file:
        tree = ast.parse(file.read())

    return [
        node.name for node in ast.walk(tree)
        if isinstance(node, ast.FunctionDef) and not node.name.startswith("__")
    ]

def get_all_defined_functions_in_codebase() -> List[str]:
    """Get all of the defined functions in the codebase.

    Returns:
        List[str]: A list of all of the defined functions in the codebase.
    """
    defined_functions = []

    paths = glob("../backend/**/*.py", recursive=True)
    assert len(paths) > 0, "No Python files found in the codebase."

    for file_path in tqdm(
        paths,
        desc="Finding defined functions",
        unit="file",
        leave=False,
    ):
        defined_functions.extend(get_all_defined_functions_in_file(file_path))

    return defined_functions

def get_all_used_functions_in_codebase() -> List[str]:
    """Get all of the used functions in the codebase.

    Returns:
        List[str]: A list of all of the used functions in the codebase.
    """
    used_functions = []
    paths = glob("../backend/**/*.py", recursive=True)
    assert len(paths) > 0, "No Python files found in the codebase."

    for file_path in tqdm(
        paths,
        desc="Finding used functions",
        unit="file",
        leave=False,
    ):
        with open(file_path, "r", encoding="utf8") as file:
            tree = ast.parse(file.read())

        for node in ast.walk(tree):
            if isinstance(node, ast.Call):
                if isinstance(node.func, ast.Name):
                    used_functions.append(node.func.id)
                elif isinstance(node.func, ast.Attribute):
                    used_functions.append(node.func.attr)

    return used_functions

def ensure_no_dead_python_code() -> None:
    """Ensure that there is no dead Python code in the codebase.

    Raises:
        DeadCodeException: If there is dead Python code in the codebase.
    """
    defined_functions = get_all_defined_functions_in_codebase()
    used_functions = get_all_used_functions_in_codebase()

    dead_functions = set(defined_functions) - set(used_functions) - set(ALLOW_LIST)

    if dead_functions:
        raise DeadCodeException(f"Dead code found in the codebase: {dead_functions}")

