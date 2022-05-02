debug:
	cargo build
	bash install.sh debug

release:
	cargo build --release
	bash install.sh release

