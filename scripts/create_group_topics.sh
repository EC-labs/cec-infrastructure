#!/bin/bash


set -e

clients=($(find creds/groups -maxdepth 1 -mindepth 1 -type d -printf '%f\n'))
CLUSTER_ENDPOINT=13.60.146.188:19093

for client in "${clients[@]}"; do
    echo
    echo "=============== $client ==============="
    echo

    # Create tutorials topic
    kafka-topics --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server "$CLUSTER_ENDPOINT" \
        --create \
        --topic "$client" \
        --partitions 16

    # Add write permissions to client
    kafka-acls --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server "$CLUSTER_ENDPOINT" \
        --add --allow-principal "User:${client}" \
        --operation write \
        --topic "$client"

    # Add read permissions for topic
    kafka-acls --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server "$CLUSTER_ENDPOINT" \
        --add \
        --allow-principal "User:${client}" \
        --operation read \
        --topic "$client"

    # Add read permissions for all groups
    kafka-acls --command-config ./creds/admins/landau/client-ssl.properties \
        --bootstrap-server "$CLUSTER_ENDPOINT" \
        --add \
        --allow-principal "User:${client}" \
        --operation read \
        --group '*'

done
