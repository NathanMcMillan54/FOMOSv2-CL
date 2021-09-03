install:
	@ curl https://raw.githubusercontent.com/NathanMcMillan54/novusk/master/targets/x86_64-novusk.json > x86_64-novusk.json
	@ cargo install bootimage

all:
	@ cargo build --target x86_64-novusk.json
	@ cargo bootimage --target x86_64-novusk.json

clean:
	@ cargo clean
