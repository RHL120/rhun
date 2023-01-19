all:
	cargo build --release
install: all
	sudo cp ./target/release/rhun /usr/sbin/ 
	sudo chown root:root /usr/sbin/rhun 
	sudo chmod a+s /usr/sbin/rhun
