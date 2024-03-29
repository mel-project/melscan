#!/bin/bash

export VITE_BASE_URL=http://127.0.0.1:13000

cd backend && find src | entr -r cargo run   --release --locked --target x86_64-unknown-linux-musl -- --connect 185.132.179.104:41814 --listen 127.0.0.1:13000 --blkidx-db ~/.melscan.db &
# cd backend && find src | entr -r cargo run   --release --locked --target x86_64-unknown-linux-musl -- --connect 194.182.170.249:11814 --listen 127.0.0.1:13000 --testnet &
# kill the background process when we exit
trap "sh -c 'killall entr; killall melscan-backend'" INT TERM EXIT

cd frontend && npm run dev