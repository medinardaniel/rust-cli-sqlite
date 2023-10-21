"""Query the database"""

import sqlite3
import pandas as pd


def read(top=True):
    """Query the database for the top 5 rows of the HateCrimesDB table"""
    conn = sqlite3.connect("HateCrimesDB.db")
    cursor = conn.cursor()
    if top:
        df = pd.read_sql_query("SELECT * FROM HateCrimesDB LIMIT 5", conn)
    else:
        df = pd.read_sql_query("SELECT * FROM HateCrimesDB ORDER BY population DESC LIMIT 5", conn)
    conn.close()
    print("Top 5 rows of the HateCrimesDB table:")
    print(df)
    return "Success"

def create():
    """
    Create a new record and insert in the database.
    """
    conn = sqlite3.connect("HateCrimesDB.db")
    cursor = conn.cursor()
    cursor.execute("INSERT INTO HateCrimesDB VALUES ('North Carolina', 'Cities', 'Durham', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '263016')")
    conn.commit()
    cursor.execute("SELECT * FROM HateCrimesDB WHERE state = 'North Carolina' AND agency_type = 'Cities' AND agency_name = 'Durham' AND population = '263016'")
    result = cursor.fetchone()
    conn.close()
    if result:
        return "Record inserted successfully"
    else:
        return "Record not inserted"


def update():
    """
    Update the population of the North Carolina record.
    """
    conn = sqlite3.connect("HateCrimesDB.db")
    cursor = conn.cursor()
    cursor.execute("UPDATE HateCrimesDB SET population = '263017' WHERE state = 'North Carolina'")
    conn.commit()
    cursor.execute("SELECT population FROM HateCrimesDB WHERE state = 'North Carolina'")
    result = cursor.fetchone()
    conn.close()
    if result and result[0] == '263017':
        return "Record updated successfully"
    else:
        return "Record not updated"

def delete():
    """Delete the record for North Carolina"""
    conn = sqlite3.connect("HateCrimesDB.db")
    cursor = conn.cursor()
    cursor.execute("DELETE FROM HateCrimesDB WHERE state = 'North Carolina'")
    conn.commit()
    conn.close()
    return "Record deleted successfully"



