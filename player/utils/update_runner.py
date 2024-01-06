import subprocess
import sys
import os

p = subprocess.Popen(["git", "pull"], cwd=os.getcwd(), shell=True)
p.wait()
p.kill()

subprocess.Popen([sys.executable, "run.py", "--updated"], shell=True)


current_pid = os.getpid()
os.kill(current_pid, 9)