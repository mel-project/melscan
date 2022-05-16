#!/bin/sh

cd backend && find src | entr -r cargo run --release -- --connect 146.59.84.29:11814 --listen 127.0.0.1:13000 &

# kill the background process when we exit
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

cd frontend && npm run dev