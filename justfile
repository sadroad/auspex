client_triple := "aarch64-unknown-linux-musl"
client_compiler := client_triple + "-gcc"
client_compiler_variables := "CARGO_TARGET_" + shoutysnakecase(client_triple) + "_LINKER=" + client_compiler + " CC=" + client_compiler

default: build-debug

build-debug: build-client-debug build-server-debug

build-client-debug:
  {{client_compiler_variables}} \
  cargo build -p client --target {{client_triple}}

build-server-debug:
  cargo build -p server

build-release: build-client-release build-server-release

build-client-release:
  {{client_compiler_variables}} \
  cargo build -p client --profile client-release --target {{client_triple}}

build-server-release:
  cargo build --release -p server

upload-client: build-client-release
  scp target/{{client_triple}}/client-release/client sadroad@192.168.1.22:~/
