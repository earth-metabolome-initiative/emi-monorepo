# Small python utility to densify the counter in the migration directories.
# This is useful when you have a lot of migrations and you have eliminated a few,
# and you want to make sure that the counter is dense and there are no gaps.

import os
import sys
import re


def densify_directory_counter():
    migrations_dir = os.path.join(os.getcwd(), "migrations")
    counter = 0
    directories = os.listdir(migrations_dir)
    directories.sort()
    for dir in directories:
        if os.path.isdir(os.path.join(migrations_dir, dir)):
            match = re.match(r"(\d+)_", dir)
            if match:
                padded_counter = "0" * (14 - len(str(counter))) + str(counter)
                new_dir = os.path.join(migrations_dir, padded_counter + dir[len(match.group(1)) :])
                os.rename(os.path.join(migrations_dir, dir), new_dir)
                print(f"Renamed {dir} to {new_dir}")
                counter += 1
    print("Done")


if __name__ == "__main__":
    densify_directory_counter()