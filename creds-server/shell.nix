let 
    pkgs = import <nixpkgs> {};
in pkgs.mkShell {
  name = "react-bootstrap-shell";
  packages = with pkgs; [
    nodePackages.create-react-app
    nodejs
    yarn
  ];
}
