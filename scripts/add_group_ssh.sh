#!/bin/bash


set -e 

clients=($(find creds/groups -maxdepth 1 -mindepth 1 -type d -printf '%f\n'))

for client in "${clients[@]}"; do
    echo
    echo "=============== $client ==============="

    [[ "$client" =~ ^[a-z]+([0-9]+)$ ]]

    client_id="${BASH_REMATCH[1]}"
    ssh_config=$(scripts/print_client_config.py "$client_id" "creds/group_vms/ssh_config")
    if [[ -z "$ssh_config" ]]; then
        continue
    fi

    echo "$ssh_config"
    cat <<< "$ssh_config" > "creds/groups/${client}/ssh_config"
    cp "creds/group_vms/ssh_key_${client_id}"{,.pub} creds/groups/${client}
done
