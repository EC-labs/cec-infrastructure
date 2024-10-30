#!/bin/python3

import re
import sys
import click


@click.command()
@click.argument('client')
@click.argument('ssh_config_file')
@click.option('--ip-only', is_flag=True, default=False)
def print_client_config(client: str, ssh_config_file: str, ip_only: bool): 
    pattern = re.compile(
        f"^Host [a-zA-Z-_]+?{client}\n.*HostName (.*)\n(.*\n)*?.*IdentityFile.*\n", 
        re.MULTILINE
    )

    with open(ssh_config_file, "r") as f: 
        fstring = f.read()

    for item in pattern.finditer(fstring):
        if not ip_only:
            print(item[0], end="")
        else:
            print(item.groups()[0])

print_client_config()
