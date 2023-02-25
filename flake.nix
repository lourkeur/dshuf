{
  description = "generate reproducible random permutations using public randomness";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {pkgs, ...}: {
        packages.default = with pkgs;
          mkShell {
            buildInputs = [openssl.dev pkg-config];
          };
        formatter = pkgs.alejandra;
      };
    };
}
