import csv
import requests
from lxml import etree

default_headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
    ' AppleWebKit/537.36 (KHTML, like Gecko)'
    ' Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763'
}
url = 'http://en.wikipedia.org/wiki/Comparison_of_text_editors'

r = requests.get(url, headers=default_headers)
root = etree.HTML(r.text)
# The main comparison table is currently the first table on the page
table =  root.xpath('//table[contains(@class, "wikitable")]')[0]
rows = table.xpath('//tr')

with open('editors.csv', 'w') as csv_file:
    writer = csv.writer(csv_file)
    for row in rows:
        csvRow = []
        for cell in row.xpath('//tr | //th'):
            csvRow.append(cell.text)
        writer.writerow(csvRow)
