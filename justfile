default: build-debug

build-debug: build-client-debug

build-client-debug:
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-unknown-linux-musl-gcc \
  CC=aarch64-unknown-linux-musl-gcc \
  cargo build -p client --target aarch64-unknown-linux-musl

build-release: build-client-release

build-client-release:
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-unknown-linux-musl-gcc \
  CC=aarch64-unknown-linux-musl-gcc \
  cargo build -p client --profile client-release --target aarch64-unknown-linux-musl

upload-client: build-client-release
  scp target/aarch64-unknown-linux-musl/client-release/client sadroad@192.168.1.22:~/

