run:
	cargo run

release:
	cargo build --release

wasm-run:
	basic-http-server .

wasm-release:
	cargo build --target wasm32-unknown-unknown --profile wasm-release \
	&& wasm-bindgen --target web --no-typescript --out-dir ./pkg --out-name trix_piping ./target/wasm32-unknown-unknown/wasm-release/trix_piping.wasm