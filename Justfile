set shell := ["zsh", "-cu"]

# Show available recipes.
default:
  @just --list

# Install JavaScript dependencies with Bun.
install:
  bun install

# Build the wasm package consumed by the web app.
pkg:
  wasm-pack build --target web --out-dir pkg --features wasm

# Build the native Rust crate.
rust:
  cargo build

# Build the wasm Rust target without generating JS bindings.
rust-wasm:
  cargo build --target wasm32-unknown-unknown --features wasm

# Start the Vite dev server after refreshing pkg/.
dev:
  just pkg
  bun run dev

# Run the TypeScript project build.
typecheck:
  bun run typecheck

# Build the web bundle.
build-web:
  bun run build:web

# Build Rust wasm bindings, then compile the web app.
build:
  just pkg
  bun run typecheck
  bun run build:web

# Preview the production bundle locally.
preview:
  bun run preview

# Run ESLint.
lint:
  bun run lint

# Run ESLint with autofix.
lint-fix:
  bun run lint:fix

# Format the repo with Oxfmt.
fmt:
  bun run fmt

# Check formatting with Oxfmt.
fmt-check:
  bun run fmt:check

# Run the common CI-style checks.
check:
  cargo build
  cargo build --target wasm32-unknown-unknown --features wasm
  bun run typecheck
  bun run lint
  bun run build:web

# Remove generated build outputs.
clean:
  rm -rf dist target
