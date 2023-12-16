all:
	cargo run $(shell date +%d)

release:
	cargo run --release $(shell date +%d)

flame:
	cargo flamegraph --root -- $(shell date +%d)
	open -a Firefox flamegraph.svg

clean:
	rm -rf target/ Cargo.lock flamegraph.svg
