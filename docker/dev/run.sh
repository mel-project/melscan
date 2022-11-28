#!/bin/bash

if [ "${RUN_FULL_NODE}" = 'false' ]; then
  if [ -z "${FULL_NODE_ADDRESS}" ]; then
    echo "Not running a full node and no node address specified with FULL_NODE_ADDRESS. Exiting."
    exit 1
  fi

  echo "Not running a full node. Connecting to ${FULL_NODE_ADDRESS}."

  # Backend Section
  if [ "${NETWORK}" = 'mainnet' ]; then
    mkdir -p /var/melscan/
    melscan-backend --connect "${FULL_NODE_ADDRESS}" --listen 127.0.0.1:13000 --blkidx-db /var/melscan/mainnet.db &
    sleep 3
  elif [ "${NETWORK}" = 'testnet' ]; then
    mkdir -p /var/melscan/
    melscan-backend --testnet --connect "${FULL_NODE_ADDRESS}" --listen 127.0.0.1:13000 --blkidx-db /var/melscan/testnet.db &
    sleep 3
  else
    echo "No network specified with NETWORK. Please use either 'mainnet' or 'testnet.' Exiting."
    exit 1
  fi

  # Frontend Section
  COMMON_SERVER_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep -v map)"
  COMMON_SERVER_MAP_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep map)"
  COMMON_CLIENT_IMMUTATABLE_FILENAME="$(ls -d /var/www/melscan-frontend/build/client/_app/immutable/chunks/* | grep common | grep -v map)"

  BACKEND_URL="http://127.0.0.1:13000"

  sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
  sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
  sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"

  node /var/www/melscan-frontend/build/index.js &

  sleep 3

  bats --print-output-on-failure /tmp/ci.bats
else
  # Node Section
  if [ "${NETWORK}" = 'mainnet' ]; then
    PUBLIC_IP_ADDRESS="$(curl -s http://checkip.amazonaws.com)"

    echo "Starting mainnet node."

    themelio-node --database /var/lib/themelio-node/mainnet --listen 0.0.0.0:11814 --advertise "${PUBLIC_IP_ADDRESS}":11814 &
    sleep 3
  elif [ "${NETWORK}" = 'testnet' ]; then
    PUBLIC_IP_ADDRESS="$(curl -s http://checkip.amazonaws.com)"

    echo "Starting testnet node."

    themelio-node --database /var/lib/themelio-node/testnet --listen 0.0.0.0:11814 --testnet --bootstrap tm-1.themelio.org:11814 --advertise "${PUBLIC_IP_ADDRESS}":11814 &
    sleep 3
  else
    echo "No network specified with NETWORK. Please use either 'mainnet' or 'testnet.' Exiting."
    exit 1
  fi

  # Backend Section
  if [ "${NETWORK}" = 'mainnet' ]; then
    mkdir -p /var/melscan/

    echo "Starting backend."

    melscan-backend --connect 127.0.0.1:11814 --listen 127.0.0.1:13000 --blkidx-db /var/melscan/mainnet.db &
#      melscan-backend --connect 146.59.84.29:41814 --listen 127.0.0.1:13000 --blkidx-db /var/melscan/mainnet.db &
    sleep 3
  elif [ "${NETWORK}" = 'testnet' ]; then
    mkdir -p /var/melscan/

    echo "Starting backend."

    melscan-backend --testnet --connect 127.0.0.1:11814 --listen 127.0.0.1:13000 --blkidx-db /var/melscan/testnet.db &
#      melscan-backend --testnet --connect 146.59.84.29:11111 --listen 127.0.0.1:13000 --blkidx-db /var/melscan/testnet.db &
    sleep 3
  else
    echo "No network specified with NETWORK. Please use either 'mainnet' or 'testnet.' Exiting."
    exit 1
  fi

  # Frontend Section
  COMMON_SERVER_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep -v map)"
  COMMON_SERVER_MAP_FILENAME="$(ls -d /var/www/melscan-frontend/build/server/chunks/* | grep common | grep map)"
  COMMON_CLIENT_IMMUTATABLE_FILENAME="$(ls -d /var/www/melscan-frontend/build/client/_app/immutable/chunks/* | grep common | grep -v map)"

  if [ "${NETWORK}" = 'mainnet' ]; then
    if [ -z "${BACKEND_URL}" ]; then
      BACKEND_URL="http://127.0.0.1:13000"

      echo "Setting BACKEND_URL to be ${BACKEND_URL} for connection from the frontend."

      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"
    else
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"
    fi
  elif [ "${NETWORK}" = 'testnet' ]; then
    if [ -z "${BACKEND_URL}" ]; then
      BACKEND_URL="http://testnet.local:13000"

      echo "Setting BACKEND_URL to be ${BACKEND_URL} for connection from the frontend."

      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"
    else
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_SERVER_MAP_FILENAME}"
      sed -ri "s|BASE_URL_DYNAMIC|${BACKEND_URL}|g" "${COMMON_CLIENT_IMMUTATABLE_FILENAME}"
    fi
  else
    echo "No network specified with NETWORK. Please use either 'mainnet' or 'testnet.' Exiting."
    exit 1
  fi


  echo "Starting frontend."

  node /var/www/melscan-frontend/build/index.js &

  sleep 3

  echo "Running BATS tests."

  bats --print-output-on-failure /tmp/ci.bats
fi