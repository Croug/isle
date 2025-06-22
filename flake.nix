{
  inputs = {
    naersk.url  = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url   = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs       = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        # Libraries that winit & libudev-sys will dlopen at runtime
        runtimeLibs = with pkgs; [
          wayland         # libwayland-client.so
          libxkbcommon    # keyboard support
          vulkan-loader   # libvulkan.so
          systemd         # provides libudev.so & libudev.pc
        ];

        # colon-separated LD_LIBRARY_PATH for both devShell & wrapped binaries
        libPath = pkgs.lib.makeLibraryPath runtimeLibs;
      in {
        # Development shell: ensures `cargo run` can find all .so and pkg-config
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.pre-commit
            pkgs.rustPackages.clippy
            pkgs.pkg-config    # so build.rs can call pkg-config
            pkgs.rust-analyzer
          ] ++ runtimeLibs;

          LD_LIBRARY_PATH = libPath;
          RUST_SRC_PATH   = pkgs.rustPlatform.rustLibSrc;
        };

        # Default package: wraps the built binaries so they carry LD_LIBRARY_PATH
        defaultPackage = naersk-lib.buildPackage {
          src               = ./.;
          buildInputs       = runtimeLibs;
          nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];

          postInstall = ''
            for bin in "$out/bin"/*; do
              wrapProgram "$bin" \
                --prefix LD_LIBRARY_PATH : "${libPath}"
            done
          '';
        };
      }
    );
}
