default: build

build: build-client

build-client:
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-unknown-linux-musl-gcc \
  CC=aarch64-unknown-linux-musl-gcc \
  cargo build -p client --target aarch64-unknown-linux-musl
