#!/bin/bash

set -e

BROKERS="13.60.146.188:19093,13.60.146.188:29093,13.60.146.188:39093"

clients=($(find creds/groups -maxdepth 1 -mindepth 1 -type d -printf '%f\n'))

for client in "${clients[@]}"; do
    # Add read permissions for topic
    kafka-acls --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server "${BROKERS}" \
        --add \
        --allow-principal "User:${client}" \
        --operation read \
        --topic "experiment"
done
