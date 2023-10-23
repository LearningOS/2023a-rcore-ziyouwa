DOCKER_NAME ?= rcore-tutorial-v3
DOCKER_PROG := $(shell if [ -x /usr/bin/podman ]; then echo podman; else echo docker; fi)

.PHONY: docker build_docker

	
docker:
	${DOCKER_PROG} run --rm -it -v ${PWD}:/mnt -w /mnt ${DOCKER_NAME} bash

build_docker: 
	${DOCKER_PROG} build -t ${DOCKER_NAME} .

fmt:
	cd os ; cargo fmt;  cd ..

