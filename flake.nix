{
  description = "A basic Rust development shell";

  inputs = {
    # Use nixpkgs unstable branch for potentially newer packages
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # Utility library to generate flake outputs for multiple systems
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    # Iterate over default systems (x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin)
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Import nixpkgs for the specific system
        pkgs = nixpkgs.legacyPackages.${system};

        # Get the latest stable Rust toolchain from pre-built binaries
        # This includes rustc, cargo, rustfmt, clippy, and the standard library source
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        # Define the default development shell
        devShells.default = pkgs.mkShell {
          name = "rust-dev-shell";

          # List of packages to make available in the shell environment
          packages = [
            rustToolchain      # The core Rust toolchain
            pkgs.rust-analyzer # Language server for IDEs/editors

            # --- Add other native build dependencies or tools here ---
            # Example: For crates needing system libraries
            # pkgs.openssl
            # pkgs.pkg-config

            # Example: Useful Rust ecosystem tools
            # pkgs.cargo-watch # Automatically recompile/run on changes
            # pkgs.cargo-edit  # Add/remove dependencies via command line
          ];

          # Optional: Set environment variables if needed
          # shellHook = ''
          #   echo "Entered Rust development shell."
          #   export MY_ENV_VAR="some_value"
          # '';

          # Note: rust-analyzer usually finds the stdlib source automatically
          # when using rust-bin. If you encounter issues, you might need:
          # RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
