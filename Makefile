check-format:
	cargo fmt -- --check && cargo clippy -- -Dwarnings

build-web:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen ./target/wasm32-unknown-unknown/release/sha256_pow.wasm --out-dir pkg/web --target web --reference-types --weak-refs

build-bundler:
	wasm-pack build --release --out-dir ./pkg/bundler --target bundler

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs

build-wasm:
	make build-web ; make build-bundler ; make build-nodejs