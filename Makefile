.PHONY: test
test: target/debug/chess_fingerprint
	./$^


target/debug/chess_fingerprint: src/main.rs
	cargo test
	cargo build

.PHONY: clean
clean:
	cargo clean
