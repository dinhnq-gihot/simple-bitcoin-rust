test:
	cargo test --all -- --nocapture

check-fmt:
	cargo +nightly fmt --all -- --check

fmt:
	cargo +nightly fmt --all

clippy:
	cargo clippy --all --all-targets --all-features -- \
		-D warnings -D clippy::enum_glob_use

ci: fmt check-fmt clippy test