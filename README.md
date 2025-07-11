# Cliph – CLI Graphing Calculator & Algebra Engine

**Cliph** is a Rust-based CLI graphing calculator and symbolic algebra engine designed for both command-line use and WebAssembly-powered web UI with Yew.

---

![Example Usage](/xsin(x).png)

## Features

- **Mathematical Expression Parsing**  
  Parse LaTeX-style and standard math expressions into an internal abstract syntax tree (AST).

- **Symbolic Differentiation**  
  Compute derivatives symbolically with respect to variables like `x`.

- **Expression Evaluation**  
  Numerically evaluate expressions.

- **LaTeX Formatting**  
  Convert parsed expressions back into LaTeX for display and further processing.

- **Graphing Interface**  
  Visualize mathematical functions and their derivatives interactively in a web UI (via Yew and WebAssembly).

- **Modular Architecture**  
  - `math/`: Core mathematical engine (parser, algebra, differentiation, evaluation, formatting, utils).  
  - `ui/`: Yew-powered web UI components (graph, input, output, app).

---

## Project Structure

src/

├── lib.rs

├── math/

│ ├── algebra.rs

│ ├── ast.rs

│ ├── diff.rs

│ ├── eval.rs

│ ├── format.rs

│ ├── parser.rs

│ ├── utils.rs

│ └── mod.rs

└── ui/

├── components/

│ ├── app.rs

│ ├── graph.rs

│ ├── input.rs

│ ├── output.rs

│ └── mod.rs

└── mod.rs

---

## Getting Started

### Prerequisites

- Rust toolchain (with `cargo`)
- `trunk` (for building and serving the web UI)
- `wasm32-unknown-unknown` target installed

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Build & Run (Web UI)
```bash
trunk serve
```

This will build the project to WebAssembly and launch a local server at http://localhost:8080.


### Build & Run (CLI)
(Coming soon)


### Dependencies

    Yew for frontend components

    wasm-bindgen for WebAssembly bindings

    gloo for browser utilities

    plotters for plotting graphs

    regex for parsing

### Roadmap

    Add CSS and formatting
    
    Fix and enhance LaTeX parsing and graphing integration

    Add CLI mode with algebraic command-line interface

    Extend symbolic algebra capabilities

    Improve graphing interactivity and UI

    Implement better error handling and user feedback

### License
MIT