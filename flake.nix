{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    };
    outputs = { self, nixpkgs, ... }@inputs: 
    let 
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
    in
    {
        devShells.${system} = 
            {
                default = pkgs.callPackage (import ./shell.nix) {};
                frontend = pkgs.mkShell {
                    packages = with pkgs; [
                        nodejs
                        yarn
                    ];
                };
                backend = pkgs.mkShell {
                    packages = with pkgs; [
                        cargo
                        rustc
                        rust-analyzer
                        sqlite
                        pkg-config
                    ];
                    CREDENTIALS_DIR = "/home/landaudiogo/Repos/teaching/cec/cec-infrastructure/creds";
                };
            };
    };
}

