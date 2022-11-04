import matplotlib as mpl
mpl.use('pgf')
from matplotlib import pyplot as plt
import matplotlib.ticker as ticker


sums = []
frequency = []
mydata = []
file1 = open('graphdata', 'r')
  
while True:
    line = file1.readline()
    if not line:
        break
    newelems = line.strip().split(",")
    mydata.append([int((newelems[0])),int((newelems[1]))])

file1.close()

sorteddata = sorted(mydata,key=lambda x: (x[0],x[1]))

for elem in sorteddata:
    sums.append(elem[0])
    frequency.append(elem[1])

plt.rc('text', usetex=True)
plt.rc('font', family='serif', size='10')
fig, ax1 = plt.subplots(figsize=(3.93, 2.4)) #set for IEEE

ax1.plot(sums,frequency,linewidth=1)
ax1.set_xlabel('Frequency')
ax1.set_ylabel('Sum')
plt.tight_layout() #prevent cut-off labels
#plt.show()
plt.savefig('wots.pgf')