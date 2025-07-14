{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-24.11";
    };
  };
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
      libPath = with pkgs;
        lib.makeLibraryPath [
          libxkbcommon
          xorg.libxcb
          wayland

          # vulkan-headers
          vulkan-loader
        ];
    in
      pkgs.mkShell {
        buildInputs = with pkgs; [
          libxkbcommon
          xorg.libxcb
          xorg.libX11.dev
          pkg-config

          sqlite
        ];

        RUST_LOG = "debug";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = libPath;

        shellHook = ''
          exec fish
        '';
      };
  };
}
