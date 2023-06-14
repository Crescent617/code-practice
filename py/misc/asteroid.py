import requests
from bs4 import BeautifulSoup
import json

url = 'https://zh.wikipedia.org/wiki/Category:1995%E5%B9%B4%E5%8F%91%E7%8E%B0%E7%9A%84%E5%B0%8F%E8%A1%8C%E6%98%9F'
response = requests.get(url)
soup = BeautifulSoup(response.text, 'lxml')

asteroids = []
for link in soup.find_all('a'):
    if link.has_attr('href') and link['href'].startswith('/wiki/'):
        name = link.text
        page_url = 'https://zh.wikipedia.org' + link['href']
        page_response = requests.get(page_url)
        page_content = BeautifulSoup(page_response.text, 'lxml').find('div', id='mw-content-text')
        content = page_content.text.strip()
        asteroid = {
            'name': name,
            'link': page_url,
            'content': content
        }
        asteroids.append(asteroid)

with open('1995asteroids.json', 'w') as f:
    json.dump(asteroids, f, ensure_ascii=False, indent=4)

