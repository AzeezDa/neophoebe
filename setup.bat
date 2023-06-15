@echo off

echo Setting up Python Virtual Environment...
if not exist .venv\ (
    python -m venv ./.venv || echo Error setting up environment, make sure you have python installed && exit /b 0
    call .venv/Scripts/activate
    echo Installing Python Libraries
    pip install -r requirements.txt > NUL || echo Could not install libraries for python, make sure you have PIP installed && exit /b 0
) 
call .venv/Scripts/activate || echo Could not activate virtual environment && exit /b 0

echo Building Simulator using Cargo...
cd src/neophoebe
cargo build --release -q || echo Building Simulator Failed && cd ../../ && exit /b 0
cd ../../
echo Setup done! Run "python src/run.py -h" for more help