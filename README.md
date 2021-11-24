# README

## build in local

`cargo build --release --target x86_64-unknown-linux-musl`

`zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap`

## build in docker

`docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.56.1-alpine3.14 cargo build --release`

## build in docker alpine

`docker run --rm -it -v \\wsl$\Ubuntu\home\kangming\Projects\lambda-npm-proxy:/usr/src/myapp -w /usr/src/myapp rust:1.56.1-alpine3.14 sh`

`apk add --update alpine-sdk openssl-dev`

`cargo build --release`

`zip -j rust.zip ./target/release/bootstrap`
