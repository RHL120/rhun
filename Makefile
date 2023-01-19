all:
	cargo build --release
install: all
	sudo chown root:root ./target/release/rhun
	sudo chmod a+s ./target/release/rhun
	sudo mv ./target/release/rhun /usr/sbin/
