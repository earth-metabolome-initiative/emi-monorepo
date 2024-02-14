# Small python utility to increase the directory counter in the Diesel migrations
# directories, so as to more easily rename them. 
# A diesel directory has a name of the form "00000000000045_some_name", where the number
# is the counter. This script increases the counter by 1 for all directories in the
# migrations directory starting from the one specified in the command line argument.

import os
import sys
import re

def increase_directory_counter(starting_counter):
    migrations_dir = os.path.join(os.getcwd(), "migrations")
    for dir in os.listdir(migrations_dir):
        if os.path.isdir(os.path.join(migrations_dir, dir)):
            match = re.match(r"(\d+)_", dir)
            if match:
                counter = int(match.group(1))
                if counter >= starting_counter:
                    new_dir = os.path.join(migrations_dir, str(counter + 1) + dir[14:])
                    os.rename(os.path.join(migrations_dir, dir), new_dir)
                    print(f"Renamed {dir} to {new_dir}")
    print("Done")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python increase_directory_counter.py <starting_counter>")
        sys.exit(1)
    starting_counter = int(sys.argv[1])
    increase_directory_counter(starting_counter)
    