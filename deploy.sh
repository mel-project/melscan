#!/bin/bash

(cd frontend && VITE_BASE_URL=https://scan.themelio.org npm run build && cp package.json build)

(cd backend && cargo build  --release --locked --target x86_64-unknown-linux-musl)

rsync -avz --progress --rsync-path="sudo rsync" ./frontend/build/ debian@web.themelio.org:/var/www/melscan-frontend/

rsync -avz --delete --progress --rsync-path="sudo rsync" ./backend/target/x86_64-unknown-linux-musl/release/melscan-backend debian@web.themelio.org:/usr/local/bin/melscan-backend

ssh debian@web.themelio.org sudo systemctl restart melscan-mainnet-backend
ssh debian@web.themelio.org sudo systemctl restart melscan-mainnet-frontend