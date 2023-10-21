# IDS Mini Project 5: CRUD Operations with SQLite

#### By Daniel Medina
![ccid workflow](https://github.com/medinardaniel/ids706-miniproj-5/actions/workflows/cicd.yml/badge.svg)

## Project Description
In this project, I created a SQLite database using a HateCrimes CSV file obtained from https://github.com/emorisse/FBI-Hate-Crime-Statistics/blob/master/2013/table13.csv.

I perform the following tasks:
* [E] Extract a dataset from a URL.
* [T] Transform the data by cleaning, filtering, enriching, etc to get it ready for analysis.
* [L] Load the transformed data into a SQLite database table using Python's sqlite3 module.
* [Q] Write and execute SQL queries on the SQLite database to analyze and retrieve insights from the data.

For the query [Q] portion, I perform the following queries:
* [C] Create a new row in the dataset for Durham, North Carolaina
* [R] Read the last five rows of the dataset
* [U] Update the population for the Durham, North Carolina created record 
* [D] Delete the Durham, North Carolina created record

In addition, I made main.py a command line tool using Python Fire. The tool can run the following commands:
* [r] Read the top 5 rows
* [c] Create a new record
* [u] Update the newly created record
* [d] Delete the newly created record

In the test_main.py file, I ensure that my script can succesfully create and delete a record.

