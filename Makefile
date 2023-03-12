SHELL := bash
CC := $(shell which cargo)
PWD := $(shell pwd)
PROJECT_NAME := $(shell pwd | sed "s#.*/##")
DOCKER_IMAGE_NAME := $(shell pwd | sed "s#.*/##" | tr [:upper:] [:lower:])
BIN := ${PROJECT_NAME}
SRC_DIR := src
LIB_DIR := 
CARGO_TOML := Cargo.toml

all: build run

# Rust code
clean:
	$(CC) clean

fmt:
	$(CC) fmt

update:
	$(CC) update

build: fmt clean
	mkdir -p bin
	$(CC) build

run:
	./bin/${BIN}

build-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux
	rm docker/build.tar

rebuild-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux --no-cache
	rm docker/build.tar

docker-build: fmt update clean
	mkdir -p bin
	docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux

docker-test: fmt clean
	docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux /bin/bash -c "cargo test"

docker-test-offline: fmt clean
	docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux /bin/bash -c "cargo test --offline"
