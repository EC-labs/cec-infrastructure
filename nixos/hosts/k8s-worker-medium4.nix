{
    imports = [
        ../modules/base-ami.nix
        ../modules/k8s-worker.nix
    ];
    
    services.kubernetes.kubelet.hostname = "k8s-worker-medium4";
}
