#!/bin/bash 


mkdir -p ca

openssl req -new -nodes \
   -x509 \
   -days 365 \
   -newkey rsa:2048 \
   -keyout ca/ca.key \
   -out ca/ca.crt \
   -config config/ca.cnf

cat ca/ca.crt ca/ca.key > ca/ca.pem
