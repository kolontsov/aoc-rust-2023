all:
	cargo run $(shell date +%d)

clean:
	rm -rf target/ Cargo.lock
