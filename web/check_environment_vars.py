"""
Simple python script to check whether all
of the environment variables present in
the file default_docker_env are present both
in the .env file and in the docker-compose.yml
document.
"""

import os
import re
import sys


def read_env_file(path: str):
    """Read the environment variables from the .env file.

    Parameters
    ----------
    path : str
        The path to the .env file.
    """
    # First, we check if the file exists
    if not os.path.exists(path):
        print(f"The file {path} does not exist")
        sys.exit(1)

    env_vars = {}
    with open(path, "r", encoding="utf8") as f:
        for line in f:
            if not line.startswith("#"):
                key, value = line.strip().split("=", maxsplit=1)
                env_vars[key] = value
    return env_vars


def read_docker_compose_file():
    """Read the environment variables from the docker-compose.yml file"""
    env_vars = {}
    with open("docker-compose.yml", "r", encoding="utf8") as f:
        for line in f:
            match = re.search(r"\${(\w+)}", line)
            if match:
                env_vars[match.group(1)] = None
    return env_vars


def main():
    """Main function"""
    default_docker_vars = read_env_file("default_docker_env")
    dot_env_vars = read_env_file(".env")
    docker_compose_vars = read_docker_compose_file()

    for key in default_docker_vars:
        if key not in docker_compose_vars:
            print(
                f"Environment variable {key} is not present in the docker-compose.yml file"
            )
            sys.exit(1)
        if key not in dot_env_vars:
            print(
                f"Environment variable {key} is not present in the .env file"
            )
            sys.exit(1)

    # Additionally, we check that there are no environment
    # variables that appear in the .env file but not in the
    # default_docker_env file
    for key in dot_env_vars:
        if key not in default_docker_vars:
            print(
                f"Environment variable {key} is present in the .env file but not in the default_docker_env file"
            )
            sys.exit(1)

    # Finally, we check that there are no environment variables
    # that appear in the docker-compose.yml file but not in the
    # default_docker_env file

    print("All environment variables are accounted for!")


if __name__ == "__main__":
    main()
