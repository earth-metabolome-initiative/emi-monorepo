"""Check for common typos in migrations."""
import os

def check_for_common_typos_in_migrations():
    """Check for common typos in migrations."""
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        with open(
            f"migrations/{directory}/down.sql", "r", encoding="utf8"
        ) as down_file:
            down_content = down_file.read()

        if "==" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `==` instead of `=`."
            )

        if "!=" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `!=` instead of `<>`."
            )

        if "CREATE TABLE IF EXISTS" in up_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `CREATE TABLE IF EXISTS` instead of `CREATE TABLE IF NOT EXISTS`."
            )

        if "DROP TABLE IF NOT EXISTS" in down_content:
            raise RuntimeError(
                f"Migration `{directory}` contains a typo: `DROP TABLE IF NOT EXISTS` instead of `DROP TABLE IF EXISTS`."
            )

        # If there is a creation of a temporary table in a up or down migration, in the same document there
        # must be a deletion of the temporary table.
        for content, content_name in (
            (up_content, "up"),
            (down_content, "down"),
        ):
            if "CREATE TEMPORARY TABLE" in content:
                # We retrieve the name of the temporary table.
                table_name = (
                    content.split("CREATE TEMPORARY TABLE")[1].split("(")[0].strip()
                )
                # We check that the deletion of the temporary table is present in the up migration.
                if f"DROP TABLE {table_name}" not in content:
                    raise RuntimeError(
                        f"Migration `{directory}` contains a `CREATE TEMPORARY TABLE` constraint in the {content_name}.sql file, but does not contain a `DROP TABLE {table_name}` constraint in the {content_name}.sql file."
                    )