import datetime
import random
import re
from urllib.parse import urlparse

import requests
from pyquery import PyQuery

pages = set()
random.seed(datetime.datetime.now())


def get_one_page(url):
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
        ' AppleWebKit/537.36 (KHTML, like Gecko)'
        ' Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763'
    }
    r = requests.get(url, headers=headers)
    if r.status_code == 200:
        return r
    return None


def get_internal_links(d: PyQuery, included):
    """ Retrieves a se  t of all Internal links found on a page """

    included = '{}://{}'.format(urlparse(included).scheme,
                                urlparse(included).netloc)
    links = set()
    # Finds all links that begin with a "/"
    p = '^/|.*' + included
    for link in d('a[href]').filter(lambda i, x: re.match(p, x.attrib['href'])):
        url = link.attrib['href']
        if not url:
            continue
        if url not in links:
            if url.startswith('/'):
                links.add(included + url)
            else:
                links.add(url)
    return links


def get_external_links(d: PyQuery, excluded):
    """ Retrieves a set of all externals links found on a page """

    links = set()
    # Finds all links that start with "http" that do
    # not contain the current URL
    p = f'^(?!{excluded})http'
    for link in d('a[href]').filter(lambda i, x: re.match(p, x.attrib['href'])):
        url = link.attrib['href']
        if not url:
            continue
        if url not in links:
            links.add(url)
    return links


def get_random_external_link(url, headers=headers):
    r = requests.get(url)
    d = PyQuery(r.text)
    domain = '{}://{}'.format(urlparse(url).scheme,
                              urlparse(url).netloc)
    externals = get_external_links(d, domain)

    if not externals:
        print(f'{url} has no externals links, looking around the site for one')
        domain = '{}://{}'.format(urlparse(url).scheme,
                                  urlparse(url).netloc)
        internals = get_internal_links(d, domain)
        if internals:
            return get_random_external_link(internals.pop())
    else:
        return externals.pop()


def get_all_external_link(url, headers=headers):
    r = requests.get(url)
    d = PyQuery(r.text)
    ex_links = get_external_links(d, urlparse(url).netloc)
    domain = '{}://{}'.format(urlparse(url).scheme,
                              urlparse(url).netloc)
    internals = get_internal_links(d, domain)
    externals = get_external_links(d, domain)

    for link in externals:
        if link not in all_ext_links:
            all_ext_links.add(link)
            print(link)
    for link in internals:
        if link not in all_int_links:
            all_int_links.add(link)
            get_all_external_link(link)


def follow_external_only(start_url):
    ex_url = get_random_external_link(start_url)
    print('Random externals link is: {}'.format(ex_url))
    follow_external_only(ex_url)


if __name__ == "__main__":
    all_ext_links = set()
    all_int_links = set()
    all_pdf_files = set()
    start_site = 'https://kns.cnki.net/KCMS/detail/42.1658.N.20191216.1530.004.html'
    all_int_links.add(start_site)
    get_all_external_link(start_site)
    # follow_external_only('http://oreilly.com')
