#!/bin/bash

script="$(basename $0)"
script_d="$(dirname "$(realpath $0)")"

USAGE="Usage: $script <number-clients> [Options]

Args:
    <number-clients> is a required argument and must be an integer

Options: 
    --json: output flag
"

if (( $# < 1 )); then
    echo "numerical positional argument expected"
    exit 1
fi

if ! [[ $1 =~ [0-9]+ ]]; then 
    cat <<< $USAGE
    exit 1
fi

num_groups=$1; shift
json=$1; shift

if ! [[ -z $json ]]; then
    echo "["
fi

for (( i=0; i<$num_groups; i++ )); do
    ip=$($script_d/print_client_config.py --ip-only "$i" "$script_d/../creds/group_vms/ssh_config")
    if ! [[ -z $json ]]; then
        entry='        {
            "host_name": "'"group$i"'",
            "base_url": "http://'"$ip"':3003"
        }'
        sep=$( (( i < num_groups-1 )) && printf "," || printf "")
        echo "${entry}${sep}"
    else 
        echo "GROUP${i}_HOST=${ip}"
    fi
done

if ! [[ -z $json ]]; then
    echo "]"
fi
