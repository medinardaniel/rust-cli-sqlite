"""
Transforms and Loads data into the local SQLite3 database
Example:
,general name,count_products,ingred_FPro,avg_FPro_products,avg_distance_root,ingred_normalization_term,semantic_tree_name,semantic_tree_node
"""
import sqlite3
import csv
import os

#load the csv file and insert into a new sqlite3 database
def load(dataset="data/HateCrimes.csv"):
    """"Transforms and Loads data into the local SQLite3 database"""

    #prints the full working directory and path
    print(os.getcwd())
    payload = csv.reader(open(dataset, newline=''), delimiter=',')
    conn = sqlite3.connect('HateCrimesDB.db')
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS HateCrimesDB")
    c.execute("CREATE TABLE HateCrimesDB (State, agency_type, agency_name, race, religion, sexual_orientation, ethnicity, disability, gender, gender_identity, q1, q2, q3, q4, population)")
    #insert
    c.executemany("INSERT INTO HateCrimesDB VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", payload)
    conn.commit()
    conn.close()
    return "HateCrimesDB.db"

