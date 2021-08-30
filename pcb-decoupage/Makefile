all: server
	cp ./server/target/release/pcb-decoupage .

client:
	cd client && npm install && ng build

server: client
	cd server && cargo build --release

clean: 
	cd client && rm -rf node_modules/
	cd server && cargo clean
