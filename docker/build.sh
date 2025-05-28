#!/bin/bash

docker build \
    -t samp-rng/build:ubuntu-18.04 ./ \
|| exit 1

docker run \
    --rm \
    -t \
    -w /code \
    -v $PWD/..:/code \
    samp-rng/build:ubuntu-18.04