# runas - simple sudo in rust
runas is a simple alternative to sudo / doas. It is written in rust, accepts no command
line flags and does not have a configuration file (the source code is the configuration).
check src/config.rs to configure it. and run `make install` to install it.
You should not install it from crates.io because you would have no way of configuring it.
