# infinite-canvas-webgl
A project for learning Webgl and Webassembly while building a Rust based infinite canvas. Focus on Performance.


# Infinite Canvas Setup Guide

This guide will walk you through setting up, building, and running the Infinite Canvas project for both native and WebAssembly (wasm) targets.

## Prerequisites

- Rust and Cargo (latest stable version)
- Git (for version control)

## Project Setup
---

tldr: cd into project base (install http-server)
```bash
wasm-pack build --target web
python3 -m http.server 8080
```

## Building and Running (Native)

1. Build the project:
   ```
   cargo build
   ```

2. Run the project:
   ```
   cargo run
   ```

## Building and Running (WebAssembly)

1. Install the wasm32-unknown-unknown target:
   ```
   rustup target add wasm32-unknown-unknown
   ```

2. Install wasm-pack:
   ```
   cargo install wasm-pack
   ```

3. Build the project for wasm:
   ```
   wasm-pack build --target web
   ```

4. Serve the directory (you can use any HTTP server, here's an example with Python):
   ```
   python3 -m http.server 8000
   ```

5. Open a web browser and navigate to `http://localhost:8000`

## Development Workflow for WebAssembly

For a more streamlined development workflow when working with WebAssembly:

1. Install wasm-server-runner:
   ```
   cargo install wasm-server-runner
   ```

2. Run your wasm app directly:
   ```
   cargo run --target wasm32-unknown-unknown
   ```

This will automatically compile your project for wasm and start a local server.

## Switching Between Native and WebAssembly Builds

- To build for native:
  ```
  cargo build
  ```

- To build for WebAssembly:
  ```
  cargo build --target wasm32-unknown-unknown --features wasm
  ```

## Verifying the Setup

1. Native:
   - When you run the application, you should see a window open with your canvas.
   - You should be able to interact with the canvas as defined in your code.

2. Web:
   - Open the browser console (usually F12 or right-click and select "Inspect").
   - You should see the message "WASM Loaded" in the console.
   - You should see a canvas element with your infinite canvas.
   - You should be able to interact with the canvas as defined in your code.

If you encounter any issues or don't see the expected results, check the console for error messages and ensure all dependencies are correctly installed.

## Troubleshooting

- If you encounter any "unresolved import" errors, make sure you've added all necessary dependencies to your `Cargo.toml` file.
- For wasm-specific issues, ensure you've included the `wasm-bindgen` feature and that your `#[cfg(target_arch = "wasm32")]` directives are correct.
- If you're having trouble with wgpu in wasm, make sure you're using the `webgl` feature of wgpu in your `Cargo.toml`.

Remember to refer back to this guide as you develop your project. As you add new features or dependencies, you may need to update your setup accordingly.