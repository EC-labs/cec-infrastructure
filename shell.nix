{ 
    pkgs ? import <nixpkgs> {}
}:
with pkgs; mkShell {
    packages = [ 
        zip
        openssl 
        jre_minimal 
        confluent-platform
    ];
}
