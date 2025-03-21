build:
	cargo build --release --locked
	
code-coverage:
	cargo llvm-cov nextest --release --locked --all-features --lib --workspace --html --exclude etopay-sdk-jni --exclude etopay-sdk-swift --exclude etopay-sdk-wasm --ignore-filename-regex "bindings/|api_types/|.*/error\.rs" --no-fail-fast

unit-tests:
	cargo nextest run --profile ci --release --locked --lib --no-fail-fast

sdk-regression-tests:
	@export $$(cat .env | sed 's/#.*//g;/^$$/d;s/=\(.*\)/=\1/g' | xargs) && cargo nextest run --profile ci --release --locked --test "rt_*" --features="sdk/postident sdk/viviswap-kyc sdk/viviswap-swap" -p sdk
