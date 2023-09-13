#!/bin/sh

HOST_DIR="${HOST_DIR:-$(pwd)/..}"
CARGO_GIT_DIR="${HOST_DIR}/tmp/contract_builder_cargo/git"
CARGO_REGISTRY_DIR="${HOST_DIR}/tmp/contract_builder_cargo/registry"

mkdir -p $CARGO_GIT_DIR
mkdir -p $CARGO_REGISTRY_DIR

docker run \
     --mount type=bind,source=$HOST_DIR,target=/host \
     --mount type=bind,source=$CARGO_GIT_DIR,target=/usr/local/cargo/git \
     --mount type=bind,source=$CARGO_REGISTRY_DIR,target=/usr/local/cargo/registry \
     --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
     --rm -i -t contract-builder \
     /bin/bash
