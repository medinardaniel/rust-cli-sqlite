"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create, read, update, delete
import fire

def main(command=None):

    # Extract
    print("Extracting data...")
    extract()

    # Transform and load
    print("Transforming data...")
    load()

    # Query
    if command == "r":
        print("Read top 5 rows...")
        read()
        return
    elif command == 'c':
        print("Create a new record for North Carolina...")
        print(create())
    elif command == 'u':
        print("Update the record for North Carolina...")
        print(update())
    elif command == 'd':
        print("Delete record for North Carolina...")
        print(delete())
    elif command == 'all' or command == None:
        print("Read top 5 rows...")
        read()
        print("Create a new record for North Carolina...")
        print(create())
        print("Update the record for North Carolina...")
        print(update())
        print("Delete record for North Carolina...")
        print(delete())


if __name__ == "__main__":
    main()
    fire.Fire(main)