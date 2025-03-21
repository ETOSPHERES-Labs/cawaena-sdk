test:
	cargo test

build:
	cargo build --release

doc:
	cargo doc

run_rust_examples:
	cd examples && chmod +x run_all_examples.sh && ./run_all_examples.sh

run_wasm_node_examples:
	cd ../bindings/wasm/examples && chmod +x run_all_examples.sh && ./run_all_examples.sh

.phony: test build doc run_rust_examples run_wasm_node_examples