"""
Test goes here

"""

import pandas as pd
import sqlite3
from mylib.query import create, delete


def test_create_delete():
    conn = sqlite3.connect("HateCrimesDB.db")
    # get the length of the HateCrimesDB table
    len_original = len(pd.read_sql_query("SELECT * FROM HateCrimesDB", conn))
    # insert a new record
    create()
    # get the length of the HateCrimesDB table after insertion
    len_after_create = len(pd.read_sql_query("SELECT * FROM HateCrimesDB", conn))
    assert len_after_create == len_original + 1

    # delete record from HateCrimesDB table
    delete()
    len_after_delete = len(pd.read_sql_query("SELECT * FROM HateCrimesDB", conn))
    assert len_original == len_after_delete

    # close connection
    conn.close()
