#!/bin/bash

echo Setting up Python Virtual Environment...
if [ ! -d ".venv" ]
then
    python3 -m venv ./.venv || { echo Cannot build python venv, try to install it using \"sudo apt install python3.10-venv\" or similar; exit; }
    source .venv/bin/activate
    echo Installing Python Libraries...
    pip install -r requirements.txt > /dev/null || { echo Cannot build python venv, try to install it using \"sudo apt install python3.10-venv\" or similar; exit; }
fi
source .venv/bin/activate { echo Error activating Virtual Environment; exit; }
echo Building Simulator using Cargo...
cd src/neophoebe
cargo build --release -q || { echo Building Simulator Failed; exit; }
cd ../../
echo Setup done! Run \"python src/run.py -h\" for more help
