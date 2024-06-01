"""Submodule providing helper methods to find whether a given Rust implementation already exists."""

from typing import List
from glob import glob
from multiprocessing import Pool


def exists_in_file(path: str, search_string: str) -> bool:
    """Searches for the given search_string in the file at the given path.

    Parameters
    ----------
    path : str
        The path to the file to search in.
    search_string : str
        The string to search for in the file.
    """
    with open(path, "r", encoding="utf8") as file:
        return search_string in file.read()


def _exists_in_file(args):
    return exists_in_file(*args)


def trait_implementation_exist(
    trait_name: str, struct_name: str, deny_file_list: List[str] = (), root: str = "all"
) -> bool:
    """Searches all rust files under the root directory and checks whether the given trait implementation exists.

    Parameters
    ----------
    trait_name : str
        The name of the trait.
    struct_name : str
        The name of the struct.
    deny_file_list : List[str], optional
        List of files to ignore.
    root : str, optional
        The root directory to search for the trait implementation.
        Can be one of "all", "webcommon", "backend", "frontend".
    """
    assert root in ("all", "webcommon", "backend", "frontend")

    if root == "all":
        return any(
            trait_implementation_exist(trait_name, struct_name, deny_file_list, r)
            for r in ("webcommon", "backend", "frontend")
        )

    paths = [
        path
        for path in glob(f"../{root}/**/*.rs", recursive=True)
        if not any(deny_file in path for deny_file in deny_file_list)
    ]

    with Pool() as pool:
        return any(
            pool.imap(
                _exists_in_file,
                ((path, f"impl {trait_name} for {struct_name}") for path in paths),
            )
        )
