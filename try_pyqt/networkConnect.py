import os, time, sys, re
from urllib import request, parse


class NetworkLogin(object):

    def __init__(self, ssid=None, username=None, password=None, print=print):
        self.ssid = ssid
        self.username = username
        self.password = password
        self.print = print

    def setloginData(self):
        data = {
            'ac_id': 1,
            'action': 'login',
            'username': self.username,
            'password': self.password
        }
        return data

    def login(self):
        url = 'https://net.tsinghua.edu.cn/do_login.php'
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763',
        }
        u = request.Request(url, headers=headers)
        try:
            data = parse.urlencode(self.setloginData()).encode('utf-8')
        except:
            self.print(self.getTime(), 'Time out...')
            return ''
        else:
            r = request.urlopen(u, data=data, timeout=3)
            r.close()

    def connect(self):
        try:
            os.system(f'netsh wlan connect name={self.ssid}')
            time.sleep(1)
            pattern = re.compile(r'\s*' + self.ssid + r'\s*')
            command = 'netsh wlan show interface'
            with os.popen(command) as p:
                flag = re.search(pattern, p.read())
            return True if flag else False
        except:
            return False
        
    def isConnected(self):
        try:
            url1 = 'https://net.tsinghua.edu.cn/do_login.php'
            r1 = request.urlopen(url1, timeout=2)
            flag = 1 if r1.status < 400 else 0
            r1.close()
        except:
            pass
        finally:
            try:
                url2 = 'https://www.baidu.com'
                r2 = request.urlopen(url2, timeout=2)
                flag = 2 if r2.status == 200 else flag
                r2.close()
                # print('>>>>flag: ', flag)
            except:
                pass
            return flag
    
    def isWifiEnable(self):
        pattern1 = re.compile(r'\s软件\s*开|\ssoftware\s*On', re.I)
        pattern2 = re.compile(r'\s已连接|\sconnected', re.I)
        command = "netsh wlan show interface"
        with os.popen(command) as p:
            message = p.read()
            flag1 = re.search(pattern1, message)
            flag2 = re.search(pattern2, message)
            # self.print(flag1, flag2)
        return flag1 or flag2

    @staticmethod
    def getTime():
        return time.strftime('[%Y-%m-%d %H:%M:%S]', time.localtime(time.time()))


def main(ssid: NetworkLogin):

    if ssid.isWifiEnable() == 0:
        ssid.print(ssid.getTime(), 'Please open wlan and then try again.')
        time.sleep(3)
        sys.exit(0)

    ssid.print(ssid.getTime(), 'Already to go! It is time to login!')

    count = 0
    while count < 5:
        count += 1
        status = ssid.isConnected()
        if status == 1:
            ssid.login()
        elif status == 2:
            ssid.print(ssid.getTime(), 'Congratulations~ Have a good time!')
            time.sleep(1)
            break
        else:
            ssid.connect()
            ssid.print(ssid.getTime(), f'Connect to {ssid.ssid}')
            break
        if count > 1:
            ssid.print(ssid.getTime(), 'continue to try...')
            time.sleep(3)


if __name__ == "__main__":

    thuSchoolNet = NetworkLogin('Tsinghua')
    accountFlag = '0'
    for _ in range(5):
        accountFlag = input(thuSchoolNet.getTime() +
                            ' Which account, yanglie13 or lihr18? ')
        if accountFlag[0] in ['y', 'l']:
            break
        else:
            thuSchoolNet.print('no such acount, try again!')

    if accountFlag[0] == 'y':
        usernameAndPassword = ('yanglie13', 'ljk5yanglie,')
    elif accountFlag[0] == 'l':
        usernameAndPassword = ('lihr18', 'LHRtsinghua115')

    thuSchoolNet.username, thuSchoolNet.password = usernameAndPassword
    main(thuSchoolNet)
