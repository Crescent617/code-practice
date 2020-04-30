import asyncio
import time
import requests
import aiohttp
import threading
from concurrent.futures.thread import ThreadPoolExecutor

headers = {
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36',
}


def now():
    return time.time()


async def _get(session, url):
    async with session.get(url, headers=headers) as r:
        # assert r.status == 200
        # print('>>> done')
        # await asyncio.sleep(0.25)
        print(threading.current_thread().name)
        return await r.text(errors='ignore')


async def get(session, url):
    return await _get(session, url)


async def async_do_req(url, n):
    async with aiohttp.ClientSession() as session:
        return await asyncio.gather(
            *[_get(session, url) for i in range(n)])
        
    # tasks.add_done_callback(my_cb)
    # dones, pendings = await asyncio.wait(tasks)


def req(s, url):
    print(threading.current_thread().name)
    return s.get(url).text


def do_requests(url, n):
    max_workers = min(1000, n)
    with ThreadPoolExecutor(max_workers) as executor, requests.Session() as s:
        res = executor.map(lambda x: req(s, url).content, range(n))
    return res


if __name__ == "__main__":

    url = 'http://localhost:8000'
    # url = 'http://baidu.com'
    n = 300
    start = now()
    ans = asyncio.run(async_do_req(url, n))
    as_time = now() - start
    start = now()
    res = do_requests(url, n)
    print('asyncio time: ', as_time)
    print('requests time: ', now() - start)
