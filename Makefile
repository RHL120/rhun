all:
	cargo build --release
install: all
	sudo cp ./target/release/runas /usr/sbin/ 
	sudo chown root:root /usr/sbin/runas 
	sudo chmod a+s /usr/sbin/runas
