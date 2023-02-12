#!/usr/bin/python
import subprocess
from subprocess import Popen, PIPE, STDOUT

turns = ""
updated_means = ""

for i in range(0, 10000):
  p = subprocess.Popen([r'./num_turns.sh'], stdout=subprocess.PIPE)
  turns += p.stdout.read().decode("utf-8")

  mean = Popen(['datamash', 'mean', '1'], stdout=PIPE, stdin=PIPE, stderr=STDOUT)    
  mean_stdout = mean.communicate(input=str.encode(turns))[0]
  updated_means += str(i) + "," + mean_stdout.decode("utf-8").replace("\n", ",\n")

print(updated_means)  
