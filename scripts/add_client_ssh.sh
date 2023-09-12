#!/bin/bash


set -e 

clients=($(find creds/clients -maxdepth 1 -mindepth 1 -type d -printf '%f\n'))

for client in "${clients[@]}"; do
    echo
    echo "=============== $client ==============="

    [[ "$client" =~ ^[a-z]+([0-9]+)$ ]]

    client_id="${BASH_REMATCH[1]}"
    ssh_config=$(scripts/print_client_config.py "$client_id" "creds/ssh_config")
    if [[ -z "$ssh_config" ]]; then
        continue
    fi

    cat <<< "$ssh_config" > "creds/clients/${client}/ssh_config"
    cp "creds/ssh_keys/ssh_key_${client_id}"{,.pub} creds/clients/${client}

done
