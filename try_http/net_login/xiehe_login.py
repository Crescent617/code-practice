import os
import time
import sys
from urllib import request, parse
from bs4 import BeautifulSoup
# from netlogin import WlanLogin


class SchoolNetworkLogin:

    def __init__(self, username, password):
        self.username = username
        self.password = password

    def set_data(self):
        url = 'http://10.161.254.51:90/p/cdbdd4a09a64909694281aec503746fd/index.html?MTAuMTYxLjI1NC41MTo5MC9sb2dpbg=='
        with request.urlopen(url) as u:
            html = u.read()
            soup = BeautifulSoup(html, 'html.parser')
            check_passwd = soup.find(
                attrs={'name': 'check_passwd'}).get('value')
            read = soup.find(attrs={'name': 'read'}).get('value')
            short_message = soup.find(
                attrs={'name': 'short_message'}).get('value')
            show_assure = soup.find(attrs={'name': 'show_assure'}).get('value')
            show_captcha = soup.find(
                attrs={'name': 'show_captcha'}).get('value')
            show_change_password = soup.find(
                attrs={'name': 'show_change_password'}).get('value')
            show_read = soup.find(attrs={'name': 'show_read'}).get('value')
            show_tip = soup.find(attrs={'name': 'show_tip'}).get('value')
            terminal = soup.find(attrs={'name': 'terminal'}).get('value')

        data = {
            'assure_phone': "",
            'captcha_value': "",
            'check_passwd': check_passwd,
            'login_type': "login",
            'new_password': "",
            'password': "%B7%9A%D14%3F%9E%85%F4%2Bp%13",
            'password1': self.password,
            'read': read,
            'retype_newpassword': "",
            'short_message': short_message,
            'show_assure': show_assure,
            'show_captcha': show_captcha,
            'show_change_password': show_change_password,
            'show_read': show_read,
            'show_tip': show_tip,
            'terminal': terminal,
            'uri': 'MTAuMTYxLjI1NC41MTo5MC9sb2dpbg==',
            'username': self.username,
        }
        return data

    def login1(self):
        url = "http://1.1.1.1/login.html"
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763'
        }
        r = request.Request(url, headers=headers)
        postData = {

            'buttonClicked': "4",
            'err_flag': "0",
            'password': self.password,
            # 'redirect_url': 'www.msftconnecttest.com/redirect',
            'username': self.username
        }

        try:
            data = parse.urlencode(postData).encode('utf-8')
            # print(data)
        except:
            print(self.getTime(), 'Time out...')
            time.sleep(2)
            return ''
        else:
            with request.urlopen(r, data=data):
                return True

    def login2(self):
        url = 'http://10.161.254.51:90/login'
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763',
        }
        r = request.Request(url, headers=headers)

        try:
            data = parse.urlencode(self.set_data()).encode('utf-8')
        except:
            print(self.getTime(), 'Time out...')
            time.sleep(2)
            return ''
        else:
            with request.urlopen(r, data=data):
                return True

    def getTime(self):
        return time.strftime('[%Y-%m-%d %H:%M:%S]', time.localtime(time.time()))

    def status(self):
        try:
            url = 'http://1.1.1.1/login.html'
            r = request.urlopen(url)
            flag = 1 if r.status < 400 else 0
        except:
            return 0
        else:
            try:
                url1 = 'http://10.161.254.51:90/login'
                r1 = request.urlopen(url1)
                flag = 2 if r1.status < 400 else flag
            except:
                return 0
            else:
                try:
                    url2 = 'https://www.baidu.com'
                    r2 = request.urlopen(url2, timeout=5)
                    flag = 3 if r2.status == 200 else flag
                except:
                    pass
                return flag


if __name__ == "__main__":
    ssid = SchoolNetworkLogin('2013012457', 'Aug@pumch24')
    print(ssid.getTime(), 'Already to go! It is time to login!')
    count = 0
    ssid.login1()
    time.sleep(0.6)
    ssid.login2()

    while count < 5:
        count += 1
        status = ssid.status()
        if status == 1:
            ssid.login1()
        elif status == 2:
            ssid.login2()
        elif status == 3:
            print(ssid.getTime(), 'Congratulations~ Have a good time!')
            time.sleep(1)
            break
        else:
            print(ssid.getTime(), 'somrthing worng')
            break
        print(ssid.getTime(), 'continue to try...')
        time.sleep(3)
