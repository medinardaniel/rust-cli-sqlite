"""
Extract a dataset from a URL like Kaggle or data.gov. JSON or CSV formats tend to work well

food dataset
"""
import requests
import pandas as pd
import io

def extract(url="https://raw.githubusercontent.com/emorisse/FBI-Hate-Crime-Statistics/master/2013/table13.csv", 
            file_path="/workspaces/sqlite-lab-dm/data/HateCrimes.csv"):
    """"Extract a url to a file path"""
    with requests.get(url) as r:
        df = pd.read_csv(io.StringIO(r.text))
        df = df.rename(columns={"State":"state", "Agency type":"agency_type", "Agency name":"agency_name", "Race":"race",
                                "Religion":"religion", "Sexual orientation":"sexual_orientation","Ethnicity":"ethnicity",
                                "Disability":"disability", "Gender":"gender", "Gender Identity":"gender_identity",
                                "1st quarter":"q1","2nd quarter":"q2","3rd quarter":"q3","4th quarter":"q4","Population":"population"
                                })
        df.to_csv(file_path, index=False)
    return file_path



