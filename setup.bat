@echo off

echo Setting up Python Virtual Environment...
if not exist .venv\ (
    python -m venv ./.venv
    call .venv/Scripts/activate
    pip install -r requirements.txt > NUL
) 
call .venv/Scripts/activate

echo Building Simulator using Cargo...
cd src/neophoebe
cargo build --release -q || echo Building Simulator Failed
cd ../../
echo Setup done! 