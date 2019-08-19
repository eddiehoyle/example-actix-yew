.PHONY: client server

client:
	cd client
	cargo web deploy --target=wasm32-unknown-unknown -p client
	cd ..

server:
	cargo build -p server

all: client server
	mv ./target/deploy/client.js ./static/client.js
	mv ./target/deploy/client.wasm ./static/client.wasm

clean:
	rm ./static/js/client.js
	cargo clean