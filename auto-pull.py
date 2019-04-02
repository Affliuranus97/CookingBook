import os
import time
import random

if __name__ == '__main__':
    while 1:
        print(f"[{time.ctime()}] Pulling...")
        os.system("git checkout -b master")
        os.system("git pull")
        os.system("git log --pretty --oneline -n5>git_latest.txt")

        time.sleep(15 + random.randint(3, 10))
