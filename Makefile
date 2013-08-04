all: aurora examples
aurora: lib/.aurora-token
examples: bin/wav-to-wav-f64

clean:
	rm -rf lib/ bin/

dirs:
	mkdir -p "lib" && mkdir -p "bin"

lib/.aurora-token: dirs
	rustc --out-dir lib/ src/aurora.rs && touch lib/.aurora-token

bin/wav-to-wav-f64: aurora dirs
	rustc --out-dir bin/ -L lib/ examples/wav-to-wav-f64.rs
