# Python script to remove migrations directories that are empty.

import os
import sys

def delete_empty_directories():
    migrations_dir = os.path.join(os.getcwd(), "migrations")
    for dir in os.listdir(migrations_dir):
        if os.path.isdir(os.path.join(migrations_dir, dir)):
            if not os.listdir(os.path.join(migrations_dir, dir)):
                os.rmdir(os.path.join(migrations_dir, dir))
                print(f"Deleted {dir}")
    print("Done")

if __name__ == "__main__":
    delete_empty_directories()