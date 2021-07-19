import pandas as pd

df = pd.read_csv('./hhh.csv')

for i, row in df.iterrows():
    row['bah']  = 'hhh'

print(df)