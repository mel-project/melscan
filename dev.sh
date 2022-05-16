#!/bin/sh

cd backend && find src | entr -r cargo run -- --connect 146.59.84.29:11814 --listen 127.0.0.1:13000 &
cd frontend && npm run dev