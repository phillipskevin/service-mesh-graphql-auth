# wasm istio/envoy plugin

## Usage

Install the needed Rust dependencies

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

Build the plugin

```
cargo build --target=wasm32-unknown-unknown --release
```

Start the server and proxy with plugin

```
docker-compose up --build
```

Test

```
curl -H "token":"323232" http://localhost:18000
Access forbidden.

curl -H "token":"32323" http://localhost:18000
Hello World!
```

## Resources

- [Extending Envoy with WASM and Rust](https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/)
- [Extending Istio with Rust and WebAssembly](https://blog.red-badger.com/extending-istio-with-rust-and-webassembly)
