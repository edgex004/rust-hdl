CLIENT_SRC_FILES = $(shell find ./client -path ./client/dist -prune -false -o -path ./client/node_modules -prune -false -o -type f -name '*')
SERVER_SRC_FILES = $(shell find ./server -path ./server/target -prune -false -o -type f -name '*')
DIST_FILES = $(shell find ./client/dist -type f -name '*')


all: pcb-decoupage

client/node_modules: client/package.json
	cd client && npm install

client/dist: client/node_modules $(CLIENT_SRC_FILES)
	cd client && npm run copy_images && ng build

server/target/release: client/dist $(DIST_FILES) $(SERVER_SRC_FILES)
	cd server && cargo build --release

pcb-decoupage: server/target/release
	cp ./server/target/release/pcb-decoupage .

clean: 
	cd client && rm -rf node_modules/
	cd server && cargo clean