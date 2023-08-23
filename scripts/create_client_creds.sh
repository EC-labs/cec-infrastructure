#!/bin/bash 


client_creds_dir="creds/admins"


function make_cnf () {
    cat << EOF > "$2"
[req]
prompt = no
distinguished_name = dn
default_md = sha256
default_bits = 4096

[ dn ]
countryName = US
organizationName = CONFLUENT
localityName = MountainView
commonName=$1
EOF
}


for i in alice bob
do
	echo "------------------------------- $i -------------------------------"
    client_dir=${client_creds_dir}/${i}

    mkdir -p ${client_dir}

    make_cnf $i ${client_dir}/${i}.cnf
    cp ca/ca.pem ${client_dir}/

    # Create server key & certificate signing request(.csr file)
    openssl req -new \
    -newkey rsa:2048 \
    -keyout ${client_dir}/$i.key \
    -out ${client_dir}/$i.csr \
    -config ${client_dir}/$i.cnf \
    -nodes

    # Sign server certificate with CA
    openssl x509 -req \
    -days 3650 \
    -in ${client_dir}/$i.csr \
    -CA ca/ca.crt \
    -CAkey ca/ca.key \
    -CAcreateserial \
    -out ${client_dir}/$i.crt \
    -extfile ${client_dir}/$i.cnf

    # Convert server certificate to pkcs12 format
    openssl pkcs12 -export \
    -in ${client_dir}/$i.crt \
    -inkey ${client_dir}/$i.key \
    -chain \
    -CAfile ca/ca.pem \
    -name $i \
    -out ${client_dir}/$i.p12 \
    -password pass:cc2023

    # Create server keystore
    keytool -importkeystore \
    -deststorepass cc2023 \
    -destkeystore ${client_dir}/kafka.keystore.pkcs12 \
    -srckeystore ${client_dir}/$i.p12 \
    -deststoretype PKCS12  \
    -srcstoretype PKCS12 \
    -noprompt \
    -srcstorepass cc2023

    keytool -keystore ${client_dir}/kafka.truststore.jks \
    -alias CARoot \
    -importcert -file ca/ca.crt -noprompt -storepass cc2023
done
