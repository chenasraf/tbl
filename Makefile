build:
	cargo build --release
build-dev:
	cargo build
install:
	cp target/release/tblf /usr/local/bin/tblf
install-dev:
	cp target/debug/tblf /usr/local/bin/tblf
dev:
	make build-dev
	make install-dev

clean:
	cargo clean
	rm -f /usr/local/bin/tblf
