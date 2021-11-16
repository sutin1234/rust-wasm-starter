# rust-wasm-starter

build wsam with rust

### setup Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

### check version

```bash
rustc --version
```

### setup WASM Wapck

```bash
cargo install wasm-pack
```

### create lib project with cargo

```bash
cargo new --lib hello-wasm
```

### build rust for web (ESM)

```bash
wasm-pack build --target web
```

### build rust for Bundler using with npm

```bash
wasm-pack build --target bundler
```
