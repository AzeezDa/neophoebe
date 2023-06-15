from io import StringIO
import subprocess
import os
import pandas as pd
from matplotlib import pyplot as plt
from sys import argv
import argparse

# Setup paths
dirname = os.path.dirname(__file__)
cwd = os.getcwd()
os.chdir(os.path.join(dirname, "./neophoebe"))

# Set up args parser
parser = argparse.ArgumentParser(description="The Neophoebe Disease Simulator.")
parser.add_argument("parameter_file", help="File path to the .ron file that has the simulator parameters")
parser.add_argument("-c", "--csv", action="store", help="filepath to where the .csv simulation result is to be stored")
parser.add_argument("-f", "--fig", action="store", help="filepath to where the matplotlib figure of the simulation result is to be stored")
parser.add_argument("-p", "--plot", action="store_true", help="show the matplotlib figure window at the end of the simulation")
parser.add_argument("-d", "--days", action="store", help="the amount of iterations the simulation is run on", default="500")
args = parser.parse_args()


# Run simulation and get its stdout
s = subprocess.Popen(f"cargo run -q --release {os.path.join(cwd, args.parameter_file)} {args.days}", shell=True, stdout=subprocess.PIPE).stdout

# Read simulation's stdout (which should have csv format)
df = pd.read_csv(s)
s.close()

if args.csv:
    df.to_csv(os.path.join(cwd, args.csv), index=False)

if not (args.plot or args.fig):
    exit(0)

# Plot data with matplotlib
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

if args.fig:
    plt.savefig(os.path.join(cwd, args.fig))

if args.plot:
    plt.show()
