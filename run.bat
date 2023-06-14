@echo off

cd grapher/.venv/Scripts
call activate
cd ../../../neophoebe
cargo run --release > ../out.csv && python ../grapher/grapher.py ../out.csv
cd ../