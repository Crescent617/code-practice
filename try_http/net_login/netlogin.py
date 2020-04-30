import pickle
import re
import subprocess
import sys
import time
from typing import List

import requests
import requests.exceptions as err


class MyAccount:

    def __init__(self, username, password, flux_usage=0):
        self.username = username
        self.password = password
        self.flux_usage = flux_usage
        self.info_url = 'http://net.tsinghua.edu.cn/rad_user_info.php'

    def checkout(self):
        r = requests.get(self.info_url)
        if not r.ok:
            raise err.ConnectionError
        data = r.text.split(',')
        self.flux_usage = int(data[6])
        return round(self.flux_usage / 1e9, 2)


class MyWlan:

    def __init__(self, ssid):
        self.ssid = ssid

    def enabled(self):
        cmd = "netsh wlan show interface"
        msg = subprocess.getoutput(cmd)
        ptn = r'(?:软件\s*开|software\s*On)|(?:已连接|connected)'
        return re.search(ptn, msg, re.I)

    def connected(self):
        subprocess.call(f'netsh wlan connect name={self.ssid}')
        time.sleep(2)
        cmd = 'netsh wlan show interface'
        msg = subprocess.getoutput(cmd)
        ptn = fr'\s*{self.ssid}\s*'
        return re.search(ptn, msg)


class MyLogin(MyWlan):

    def __init__(self, ssid, url, account=None):
        self.ssid = ssid
        self.url = url
        self.account = account

    def set_data(self):
        data = {
            'ac_id': 1,
            'action': 'login',
            'username': self.account.username,
            'password': self.account.password
        }
        return data

    def login(self):
        requests.post(self.url, data=self.set_data())

    def status(self):
        status = 0
        test_url = 'https://www.baidu.com'
        try:
            r1 = requests.get(test_url, timeout=1)
            if r1.ok:
                status = 2
        except err.ConnectionError:
            r2 = requests.get(self.url)
            if r2.ok:
                status = 1
        return status

    @staticmethod
    def get_time():
        return time.strftime('[%Y-%m-%d %H:%M:%S]',
                             time.localtime(time.time()))


def reset_flux_usage(accounts):
    for account in accounts:
        account.flux_usage = 0


def main():
    with open('./my_accounts', 'rb') as f:
        accounts = pickle.load(f)

    # reset flux_usage when new month coming
    month_day = time.localtime().tm_mday
    if month_day == 1:
        reset_flux_usage(accounts)

    # choose the minmum usage account
    thu_url = 'https://net.tsinghua.edu.cn/do_login.php'
    thu_net = MyLogin(ssid='Tsinghua', url=thu_url,
                      account=min(accounts, key=lambda x: x.flux_usage))

    for _ in range(5):
        if thu_net.enabled():
            break
    else:
        print(MyLogin.get_time(), 'wlan is not enabled! Turn on it first.')
        sys.exit(0)

    for _ in range(5):
        if thu_net.connected():
            break
    else:
        print(MyLogin.get_time(), 'Something wrong. Perhaps your PC are offline.')
        sys.exit(0)

    print(MyLogin.get_time(), 'Already to go! It is time to login!')

    for _ in range(5):
        stat = thu_net.status()
        if stat == 1:
            thu_net.login()
        elif stat == 2:
            print(MyLogin.get_time(), 'Congratulations~ Have a good time!')
            time.sleep(1)
            break
        else:
            print(MyLogin.get_time(), 'Try again!')
            time.sleep(2)
    else:
        sys.exit(0)

    print(MyLogin.get_time(),
          f'{thu_net.account.username} has used'
          f' {thu_net.account.checkout()} GB.')
    with open('./my_accounts', 'wb') as f:
        pickle.dump(accounts, f)
    time.sleep(2)


if __name__ == "__main__":

    main()
    # os.system('pause')
