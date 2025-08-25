{ 
    pkgs ? import <nixpkgs> {}
}:
with pkgs; mkShell {
    packages = [ 
        openssl 
        jre_minimal 
        confluent-platform
    ];
}
