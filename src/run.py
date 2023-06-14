import subprocess
import os
import pandas as pd
from matplotlib import pyplot as plt
from sys import argv

dirname = os.path.dirname(__file__)
cwd = os.getcwd()
os.chdir(os.path.join(dirname, "./neophoebe"))
s = subprocess.Popen(["cargo", "run", "-q", "--release", os.path.join(cwd, argv[1])] + argv[2:], shell=True, stdout=subprocess.PIPE).stdout

df = pd.read_csv(s)
s.close()
fig, axs = plt.subplots(2, 2, figsize=(12, 7))

axs[0, 0].plot(df.t, df.s, color="Lime")
axs[0, 0].set_title("Susceptible")
axs[1, 0].plot(df.t, df.e + df.c, color="Red")
axs[1, 0].set_title("Infected")
axs[0, 1].plot(df.t, df.r, color="Cyan")
axs[0, 1].plot(df.t, df.d, color="Black")
axs[0, 1].set_title("Recovered/Deceased")

d = df.iloc[-1]
current_status = [d[1], d[2]+d[3], d[4], d[5]]
axs[1, 1].pie(current_status, colors = ["Lime", "Red", "Cyan", "Black"], labels=["Susceptible", "Infected", "Recovered", "Deceased"])
axs[1, 1].set_title(f"Status at day {d[0]}")
fig.tight_layout(pad=2.0)
plt.show()