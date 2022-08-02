#!/bin/bash

export VITE_BASE_URL=http://127.0.0.1:13000

cd backend && find src | entr -r cargo run -- --connect 127.0.0.1:11814 --listen 127.0.0.1:13000 --blkidx-db test.db &
# kill the background process when we exit
trap "sh -c 'killall entr; killall melscan-backend'" INT TERM EXIT

cd frontend && npm run dev