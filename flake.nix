{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
	runtimeLibs = with pkgs; [
	  wayland
	  libxkbcommon
	  vulkan-loader
	];

	libPath = pkgs.lib.makeLibraryPath runtimeLibs;
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ] ++ runtimeLibs;
	  LD_LIBRARY_PATH = libPath;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        defaultPackage = naersk-lib.buildPackage {
          src			= ./.;
	  buildInputs		= runtimeLibs;
	  nativeBuildInputs	= [ pkgs.makeWrapper ];

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
