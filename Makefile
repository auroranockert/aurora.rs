all: aurora examples
aurora: lib/.aurora-token
examples: bin/wav-to-wav-s16 bin/wav-to-au-s16

test: aurora
	rust test src/aurora.rs

clean:
	rm -rf lib/ bin/

dirs:
	mkdir -p "lib" && mkdir -p "bin"

lib/.aurora-token: dirs
	rustc -Z debug-info --out-dir lib/ src/aurora.rs && touch lib/.aurora-token

bin/wav-to-wav-s16: aurora dirs
	rustc -Z debug-info --out-dir bin/ -L lib/ examples/wav-to-wav-s16.rs

bin/wav-to-au-s16: aurora dirs
	rustc -Z debug-info --out-dir bin/ -L lib/ examples/wav-to-au-s16.rs
