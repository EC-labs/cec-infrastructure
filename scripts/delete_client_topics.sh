#!/bin/bash


set -e


clients=($(find creds/clients -maxdepth 1 -mindepth 1 -type d -printf '%f\n'))

for client in "${clients[@]}"; do
    echo
    echo "=============== $client ==============="
    echo

    # Delete tutorials topic
    kafka-topics --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server localhost:19093 \
        --delete \
        --topic "$client"

done
