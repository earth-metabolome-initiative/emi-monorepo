"""Utility functions for constraint checkers."""
import os

def infer_route_from_document(document: "Document") -> str:
    """Return the route inferred from the document's path."""
    if "/backend/" in os.path.abspath(document.name):
        return "backend"
    if "/web_common/" in os.path.abspath(document.name):
        return "web_common"
    if "/frontend/" in os.path.abspath(document.name):
        return "frontend"

    raise RuntimeError(
        "The document must be in the backend, web_common, or frontend directory."
    )