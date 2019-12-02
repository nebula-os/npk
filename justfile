build: build-x86_64

build-aarch64:
    CC=aarch64-linux-gnu-musl-gcc cargo rustc --release --target aarch64-unknown-linux-musl --verbose -- -C linker=aarch64-linux-gnu-musl-gcc -C link-arg=-lgcc
build-x86_64:
    SODIUM_USE_PKG_CONFIG=1 CC=musl-gcc CXX=musl-gcc cargo build --release --target x86_64-unknown-linux-musl

docker-build-build:
    sudo docker build -t npk-build -f docker/build.dockerfile .
docker-build-run:
    mkdir -p ./cache
    mkdir -p ./target
    sudo chown -R $USER ./cache
    sudo chown -R $USER ./target
    sudo docker run \
        -v $(pwd)/:/work \
        -v $(pwd)/cache/:/cache -i npk-build
    sudo chown -R $USER ./cache
    sudo chown -R $USER ./target