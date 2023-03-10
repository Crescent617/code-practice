import datetime
import random
from pathlib import Path
from urllib.parse import urlparse
from urllib.request import urlretrieve

import requests
from lxml import etree

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
    ' AppleWebKit/537.36 (KHTML, like Gecko)'
    ' Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763'
}
download_dir = 'downloaded'
base_url = 'http://pythonscraping.com'


def get_absolute_url(base_url, source):
    if source.startswith('http://www.'):
        url = 'http://{}'.format(source[11:])
    elif source.startswith('http://'):
        url = source
    elif source.startswith('www.'):
        url = source[4:]
        url = 'http://{}'.format(source)
    else:
        url = '{}/{}'.format(base_url, source)
    if base_url not in url:
        return None
    return url


def get_download_path(base_url, abs_url, download_dir):
    path = Path(download_dir + urlparse(abs_url).path)
    directory = path.parent

    if not directory.exists():
        directory.mkdir(parents=True)
    return path


if __name__ == "__main__":

    r = requests.get('http://www.pythonscraping.com', headers=headers)
    root = etree.HTML(r.text)
    downloads = root.xpath('//@src')

    for link in downloads:
        file = get_absolute_url(base_url, link)
        if file is not None:
            print(file)
            urlretrieve(file,
                        get_download_path(base_url, file, download_dir))
