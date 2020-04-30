import json
import time
from concurrent.futures import as_completed, process, thread
from typing import Iterator
from urllib.parse import urlencode, urljoin

import requests
import tqdm
from lxml import etree
from pymongo import MongoClient


class MyCrawler:

    def __init__(self, url, kw):
        self.base_url = url
        self.kw = kw
        self.headers = self.set_headers()

    def set_headers(self) -> dict:
        with open('./headers.json') as f:
            headers = json.load(f)
        # data = {'keyword': self.kw}
        # url = 'https://www.toutiao.com/search/?'
        # headers['referer'] = urljoin(url, urlencode(data))
        return headers

    def set_params(self, page) -> dict:
        offset = page * 20
        params = {
            'aid': '24',
            'app_name': 'web_search',
            'offset': offset,
            'format': 'json',
            'keyword': self.kw,
            'autoload': 'true',
            'count': '20',
            'en_qc': '1',
            'cur_tab': '1',
            'from': 'search_tab',
            'pd': 'synthesis',
            'timestamp': '1576680052114'
        }
        return params

    def get_json(self, page):
        url = self.base_url + urlencode(self.set_params(page))
        try:
            r = requests.get(url, headers=self.headers)
        except Exception:
            return {}
        return r.json()

    def get_content(self, page) -> Iterator:
        r_json = self.get_json(page)
        items = r_json.get('data')
        headers = {
            'user-agent': ('Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
                           ' AppleWebKit/537.36 (KHTML, like Gecko)'
                           ' Chrome/74.0.3729.157 Safari/537.36')
        }

        def dfs(items, depth=1):
            # print('-> '*depth + ' in dfs')
            if hasattr(items, 'get'):
                if items.get('user_id'):
                    toutiao = {}
                    toutiao['id'] = items.get('id')
                    toutiao['title'] = items.get('title')
                    toutiao['abstract'] = items.get('abstract')
                    at_url = items.get('article_url')
                    toutiao['article_url'] = at_url
                    # try:
                    #     r = requests.get(at_url, headers=headers)
                    # except requests.HTTPError as e:
                    #     text = f'{r.status_code}: {e.args}'
                    # else:
                    #     text = ''.join(etree.HTML(r.text).xpath('//text()'))
                    # toutiao['text'] = text
                    yield toutiao

            if hasattr(items, '__iter__') and type(items) != str:
                if hasattr(items, 'values'):
                    items = items.values()
                for it in items:
                    yield from dfs(it, depth+1)
        return dfs(items)


def save_to_mongo(content, collection):
    if type(content) == list:
        collection.insert_many(content)
    else:
        collection.insert_one(content)
    # print('{0} Save content to MongoDB. {0}'.format('='*5))


def get_many(crawler: MyCrawler, pages, collection, max_workers=6):
    with thread.ThreadPoolExecutor(max_workers) as execurtor:
        fs = [execurtor.submit(crawler.get_content, i) for i in pages]
        cnt = 0
        for f in as_completed(fs):
            cnt += 1
            print(f'>>> {cnt} pages completed')
        # to_do = tqdm.tqdm(results, total=len(pages))
        # for content in to_do:
        #     for c in content:
        #         save_to_mongo(c, collection)


if __name__ == '__main__':
    base_url = 'https://www.toutiao.com/api/search/content/?'
    keyword = '杨文 北京医生'

    # client = MongoClient()
    # db = client['web']
    # collection = db[keyword]
    start_time = time.time()
    print('{0} start {0}'.format('='*40))
    toutiao = MyCrawler(base_url, keyword)

    get_many(toutiao, list(range(1, 50)), 1, 20)
    print('Total time usage: {}'.format(time.time() - start_time))
    # print('{0} Saved {1} items to python list.{0}'
    #       .format('='*5, collection.count_documents({})))
