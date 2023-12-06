all:
	cargo run $(shell date +%d)

release:
	cargo run --release $(shell date +%d)

clean:
	rm -rf target/ Cargo.lock
