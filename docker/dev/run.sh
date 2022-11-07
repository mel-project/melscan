#!/bin/bash

# BACKEND_URL
# NETWORK

# I need to default BACKEND_URL to https://scan.themelio.org


COMMON_SERVER_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep -v map)"
COMMON_SERVER_MAP_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep map)"
COMMON_CLIENT_IMMUTATABLE_FILENAME="$(ls -d /var/www/melscan-frontend/build/client/_app/immutable/chunks/* | grep common | grep -v map)"

sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"


#if [ -z "${BACKEND_URL}" || -z "${NETWORK}" ]; then
#  echo ""
#  exit 1
#elif [ -n "${ADVERTISE_MANUAL}" ]; then
#  exec themelio-node --database {{ pkg.svc_data_path }}/main3 --listen 0.0.0.0:{{ cfg.port }} --advertise "${ADVERTISE_MANUAL}":{{ cfg.port }}
#else
#  PUBLIC_IP_ADDRESS="$(curl -s http://checkip.amazonaws.com)"
#  exec themelio-node --database {{ pkg.svc_data_path }}/main3 --listen 0.0.0.0:{{ cfg.port }} --advertise "${PUBLIC_IP_ADDRESS}":{{ cfg.port }}
#fi


# I need a way of selecting the .db file here for mainnet and testnet.
melscan-backend --connect 146.59.84.29:41814 --listen 127.0.0.1:13000 --blkidx-db /var/melscan-mainnet.db &

sleep 3

node /var/www/melscan-frontend/build/index.js &

sleep 3

bats --print-output-on-failure /tmp/ci.bats