# `NEOPHOEBE` - A Disease Simulator
Namesake: [Phoebe](https://en.wikipedia.org/wiki/Phoebe_(Titaness))

A Disease Simulator written in Rust with a user interface in written in Python.

# Setup

## Windows (Best option)
1. Run the `setup.bat` script
2. If no errors showed up then you are done!

## Linux (Not Recommended)
1. Type `. ./setup.sh` in a terminal (yes the `.` is important)
2. If no errors showed up then you are done!

# Running the Simulation
The simplest way to run the simulation is to run `python src/run.py <parameters_file> -p` where `<parameters_file>` is the path to the .ron parameters file you want to simulate. For more help run `python src/run.py -h`

# Examples
Here are some examples from the [examples/](/examples/) directory

Given the parameters file at [examples/no_restriction/parameters.ron](/examples/no_restriction/parameters.ron) the output figure of that simulation is:

![Output figure of an example simulation](/examples/no_restriction/figure.png)

Given the parameters file at [examples/community_restriction/parameters.ron](/examples/community_restriction/parameters.ron) the output figure of that simulation is:

![Output figure of an example simulation](/examples/community_restriction/figure.png)

# Theory / Thought Process
The theory and thought process behind the implementation can be found in [docs/neophoebe.pdf](/docs/neophoebe.pdf).