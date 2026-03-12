@echo off

echo Building Rust project...
cargo build

echo Starting server
start cmd /k cargo run

echo Waiting for server to start
timeout /t 3 >nul

echo Opening browser...
start http://localhost:8080