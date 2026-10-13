Open ports for security group:

tcp 22     external
tcp 8888
tcp 6443
tcp 10250
udp 8285   internal
udp 8472   internal

to make it easier, all tcp and udp ports are available for the security group the vms are in
0-65535 tcp launch-wizard-1
0-65535 udp launch-wizard-1

```bash
nixos-rebuild --flake .#cec-k8s-master --target-host root@34.253.139.248 switch
nixos-rebuild --flake .#cec-k8s-worker1 --target-host root@108.132.235.40 switch
nixos-rebuild --flake .#cec-k8s-worker2 --target-host root@34.249.239.191 switch
nixos-rebuild --flake .#cec-k8s-worker3 --target-host root@34.248.168.117 switch
nixos-rebuild --flake .#cec-k8s-worker-medium1 --target-host root@54.77.59.188 switch
nixos-rebuild --flake .#cec-k8s-worker-medium2 --target-host root@54.220.253.212 switch
nixos-rebuild --flake .#cec-k8s-worker-medium3 --target-host root@34.247.118.96 switch
nixos-rebuild --flake .#cec-k8s-worker-medium4 --target-host root@34.245.192.14 switch
```

# Adding a new node

Add entry to `nixos/flake.nix`:
```nix
    # ...
    cec-k8s-worker-medium4 = nixpkgs.lib.nixosSystem {
        inherit system;

        specialArgs = { inherit inputs; };
        modules = [
            agenix.nixosModules.default
            ./hosts/k8s-worker-medium4.nix
        ];
    };
    # ...
```

Add hosts file to `nixos/hosts`, e.g.,:
```nix
# nixos/hosts/k8s-worker-medium4.nix
{
    imports = [
        ../modules/base-ami.nix
        ../modules/k8s-worker.nix
    ];
    
    services.kubernetes.kubelet.hostname = "k8s-worker-medium4";
}
```

Add build command to `nixos/README.md` with VM public IP, e.g.:
```bash
nixos-rebuild --flake .#cec-k8s-worker-medium4 --target-host root@34.245.192.14 switch
```

Add kubelet hostname to private IP mapping to `nixos/modules/base-ami.nix`, e.g.:
```nix
    # ...
    "10.0.1.29" = [ "k8s-worker-medium4" ];
    # ...
```

Add host public ed25519 key to secrets.nix and add the host's public key variable to the `apitoken.age` and `root-ca.pem.age` secrets, e.g.,:
```
--- a/nixos/secrets/secrets.nix
+++ b/nixos/secrets/secrets.nix
@@ -7,19 +7,21 @@ let
     k8s-worker-medium1 = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJsi8rWE14VFJM4ArQ3SsKaaPhJQvITzxfGGyWL/Geg+ root@ip-10-0-1-135.eu-west-1.compute.internal";
     k8s-worker-medium2 = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAGBtTGBDXCpiwm+QCZ9uby07bxDkPjyqSnxSMX/4Eh5 root@ip-10-0-1-17.eu-west-1.compute.internal";
     k8s-worker-medium3 = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIBVMWgwdgCVKabm1nO1h6a7Vuy0DteGFX+pclhTMBADH root@ip-10-0-1-198.eu-west-1.compute.internal";
+    k8s-worker-medium4 = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKoJvVdZ9WyBwU25+E1JKLU0XrpPc1PQMAZjOaFUJvdK root@ip-10-0-1-29.eu-west-1.compute.internal";
+
 in
 {
     "apitoken.age".publicKeys = [
         landaudiogo
         k8s-master
         k8s-worker1 k8s-worker2 k8s-worker3
-        k8s-worker-medium1 k8s-worker-medium2 k8s-worker-medium3
+        k8s-worker-medium1 k8s-worker-medium2 k8s-worker-medium3 k8s-worker-medium4
     ];
     "root-ca.pem.age".publicKeys = [
         landaudiogo
         k8s-master
         k8s-worker1 k8s-worker2 k8s-worker3
-        k8s-worker-medium1 k8s-worker-medium2 k8s-worker-medium3
+        k8s-worker-medium1 k8s-worker-medium2 k8s-worker-medium3 k8s-worker-medium4
     ];
```

Finally, rekey the apitoken and root-ca.pem keys with:
```bash
for key in apitoken root-ca.pem; do
    agenix -d "${key}.age" > ".decrypted/${key}"
    rm "${key}.age"
    agenix -e "${key}.age" < ".decrypted/${key}"
done
```

After doing so, you can build the new worker node, and make sure to also build the master node as well so that it has its /etc/hosts file updated with the new worker node's IP, e.g.:
```bash
nixos-rebuild --flake .#cec-k8s-worker-medium4 --target-host root@34.245.192.14 switch
nixos-rebuild --flake .#cec-k8s-master --target-host root@34.253.139.248 switch
```
