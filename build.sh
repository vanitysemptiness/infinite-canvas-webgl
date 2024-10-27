#!/bin/bash

# Clean target directory
rm -rf target
rm -rf pkg

# Build wasm package
wasm-pack build --target web

# Optional: Copy to your web directory if needed
# cp -r pkg/* www/

# Start server with no-cache headers
python3 -m http.server 8080 --bind 127.0.0.1 &
SERVER_PID=$!

# Use watchexec to watch for file changes
watchexec -w src -w Cargo.toml -w index.html -- "\
  echo 'Rebuilding...' && \
  rm -rf target pkg && \
  wasm-pack build --target web && \
  echo 'Build complete'"

# Cleanup when script is terminated
trap "kill $SERVER_PID" EXIT