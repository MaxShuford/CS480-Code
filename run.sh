#!/bin/bash

echo "Building Rust project..."
cargo build

if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

echo "Starting server..."
cargo run &

SERVER_PID=$!

echo "Waiting for server to start..."
sleep 3

echo "Opening browser..."
if command -v xdg-open > /dev/null; then
    xdg-open http://localhost:8080
elif command -v open > /dev/null; then
    open http://localhost:8080
else
    echo "Please open http://localhost:8080 manually"
fi

wait $SERVER_PID