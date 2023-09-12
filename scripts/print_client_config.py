#!/bin/python3

import re
import sys
import click


@click.command()
@click.argument('client')
@click.argument('ssh_config_file')
def print_client_config(client: str, ssh_config_file: str): 
    pattern = re.compile(
        f"^Host CloudCourse_VM_{client}\n(.*\n)*?.*IdentityFile.*\n", 
        re.MULTILINE
    )
    with open(ssh_config_file, "r") as f: 
        fstring = f.read()

    for item in pattern.finditer(fstring):
        print(item[0], end="")

print_client_config()
