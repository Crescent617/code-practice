import sys
import time

while True:
    print(time.time(), "1")
    print(time.time(), "2", file=sys.stderr)
    time.sleep(1)


