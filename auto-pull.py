import os
import time
import random

if __name__ == '__main__':
    while 1:
        print(f"[{time.ctime()}] Pulling...")
        os.system("git checkout master")
        os.system("git pull")
        os.system("git log --pretty --oneline -n5>git_latest.txt")

        time.sleep(random.randint(5, 10))
