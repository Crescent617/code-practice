# coding:utf-8
import os
import time
from concurrent.futures import as_completed
from concurrent.futures.thread import ThreadPoolExecutor

import requests


def do_req(n):
    print(f'This is task {n}')
    time.sleep(0.1)
    url = 'http://163.com'
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
        ' AppleWebKit/537.36 (KHTML, like Gecko)'
        ' Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763'
    }
    try:
        # r = requests.get(url, stream=True)
        r = requests.get(url, headers=headers)
    except requests.RequestException as e:
       # pass

        return None
    else:
        print(f'task {n} done')
        return r.status_code
    # else:
    #     print(r.status_code)


def my_print(s):
    s = str(s)
    x = (78 - len(s)) // 2
    print('='*x + ' ' + s + ' ' + '='*x)

if __name__ == "__main__":

    my_print('connection start')
    start_time = time.time()

    with ThreadPoolExecutor(max_workers=os.cpu_count()) as executor:
        # futures = executor.map(do_req, range(50))
        futures = [executor.submit(do_req, i) for i in range(20)]
        # print([f for f in as_completed(futures)])
        for i, f in enumerate(as_completed(futures)):
            print(f'task {i} running: {f.running()}')

    print(f'total time: {time.time() - start_time}')
