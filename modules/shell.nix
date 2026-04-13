{ inputs, ... }: {
  perSystem = { pkgs, system, ... }: 
  let
    f = with inputs.fenix.packages.${system}; combine [
      complete.toolchain
    ];
  in {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        f
      ];
    };
  };
}