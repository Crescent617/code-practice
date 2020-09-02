import asyncio
import json
import time
from typing import Iterator
from urllib.parse import urlencode, urljoin

import aiohttp
import tqdm
from lxml import etree
from pymongo import MongoClient



class MyCrawler:

    def __init__(self, url, kw):
        self.base_url = url
        self.kw = kw
        self.headers = self.set_headers()
        self.semaphore = asyncio.Semaphore(500)

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

    async def _get_json(self, page):
        async with aiohttp.ClientSession() as session:
            async with session.post(
                    self.base_url, data=self.set_params(page),
                    headers=self.headers) as r:
                assert r.status == 200
                return await r.json()

    async def get_json(self, page):
        async with self.semaphore:
            return await self._get_json(page)

    async def get_content(self, page) -> Iterator:
        def dfs(items, depth=1):
            if depth > 10:
                return
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
        try:
            content = dfs(await self.get_json(page))
        except AssertionError:
            content = []
        return content
        # print(await self.get_json(page))


async def main():
    asyncio.Semaphore(100)
    base_url = 'https://www.toutiao.com/api/search/content/?'
    keyword = '杨文 北京医生'

    # client = MongoClient()
    # db = client['web']
    # collection = db[keyword]
    toutiao = MyCrawler(base_url, keyword)
    tasks = [toutiao.get_content(i) for i in range(1, 100)]
    # content = await task.result()

    start_time = time.time()
    print('{0} start {0}'.format('='*40))

    cnt = 0
    for t in asyncio.as_completed(tasks):
        await t
        cnt += 1
        print(f'>>> {cnt} pages completed')

    print('Total time usage: {}'.format(time.time() - start_time))
    # if type(content) == list:
    #     collection.insert_many(content)
    # else:
    #     collection.insert_one(content)

    # print('{0} Save content to MongoDB. {0}'.format('='*5))



if __name__ == '__main__':

    asyncio.run(main())
