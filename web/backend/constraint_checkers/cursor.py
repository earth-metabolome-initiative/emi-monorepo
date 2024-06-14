"""This module contains the function to get the cursor to the database."""
from typing import Tuple
import os
import psycopg2

def get_cursor() -> Tuple[psycopg2.extensions.connection, psycopg2.extensions.cursor]:
    """Get the cursor to the database."""
    dbname = os.getenv("POSTGRES_DB")
    user = os.getenv("POSTGRES_USER")
    password = os.getenv("POSTGRES_PASSWORD")
    port = os.getenv("PGPORT")
    # url = os.getenv("POSTGRES_URL")

    # Establishing a connection to the PostgreSQL database
    conn = psycopg2.connect(
        dbname=dbname,
        user=user,
        password=password,
        host="localhost",
        port=port,
    )

    return conn, conn.cursor()