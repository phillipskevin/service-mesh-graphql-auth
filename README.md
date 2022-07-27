# Service Mesh GraphQL Authorization

Example Service Mesh Plugin for Istio/Envoy for handling field-level authorization for GraphQL Services.

## Usage

1. Install Rust

https://www.rust-lang.org/tools/install

2. Install the needed Rust dependencies

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

3. Build the plugin

```
cargo build --target=wasm32-unknown-unknown --release
```

4. Start the server and proxy with plugin

```
docker-compose up --build
```

Test

```
curl -X POST -H "User: Kevin" -H "Content-Type: application/json" -d '{ "query": "query { people { name age email } }" }' http://localhost:4001
{ data }

curl -X POST -H "User: Matt" -H "Content-Type: application/json" -d '{ "query": "query { people { name age email } }" }' http://localhost:4001
403
```

## Resources

- [Extending Envoy with WASM and Rust](https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/)
- [Extending Istio with Rust and WebAssembly](https://blog.red-badger.com/extending-istio-with-rust-and-webassembly)
- [apollo-rs: spec-compliant GraphQL tools in Rust](https://www.apollographql.com/blog/announcement/tooling/apollo-rs-graphql-tools-in-rust/)
