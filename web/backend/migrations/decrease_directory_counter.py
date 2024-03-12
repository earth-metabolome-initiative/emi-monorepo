# Small python utility to decrease the directory counter in the Diesel migrations
# directories, so as to more easily rename them.
# A diesel directory has a name of the form "00000000000045_some_name", where the number
# is the counter. This script decreases the counter by 1 for all directories in the
# migrations directory starting from the one specified in the command line argument.

import os
import sys
import re


def decrease_directory_counter(starting_counter):
    migrations_dir = os.path.join(os.getcwd(), "migrations")
    for dir in os.listdir(migrations_dir):
        if os.path.isdir(os.path.join(migrations_dir, dir)):
            match = re.match(r"(\d+)_", dir)
            if match:
                counter = int(match.group(1))
                if counter >= starting_counter:
                    decreased_counter = str(counter - 1)
                    padded_decreased_counter = (
                        "0" * (len(match.group(1)) - len(decreased_counter))
                        + decreased_counter
                    )

                    new_dir = os.path.join(
                        migrations_dir, padded_decreased_counter + dir[len(match.group(1)) :]
                    )

                    # We make sure to add back the appropriate number of leading zeroes,
                    # removing one in the case of the counter being 9.

                    os.rename(os.path.join(migrations_dir, dir), new_dir)
                    print(f"Renamed {dir} to {new_dir}")
    print("Done")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python decrease_directory_counter.py <starting_counter>")
        sys.exit(1)
    starting_counter = int(sys.argv[1])
    decrease_directory_counter(starting_counter)
