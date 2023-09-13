# Contract Builder

This is a helper Dockerfile that allows to build contracts in a reproducible way.

The contract built in the Docker will result in a binary that is the same if built on other machines.

For this you need to setup Docker first. Then use the following instructions.

## Build container

In this folder, run:

```bash
docker build . -t contract-builder:latest
```

## Start docker instance

The following command will launch a docker instance and mount the root folder of this repository under `/host`.

```bash
./run.sh
```

> This command will also create folder `tmp/contract_builder_cargo` for caching cargo files. It can speed up consequent builds in docker.

## Build contracts in docker

Enter mounted path and run `build.sh`:

```bash
cd /host
./build.sh
```
