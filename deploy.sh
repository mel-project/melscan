#!/bin/bash


# Mainnet frontend
(cd frontend && VITE_BASE_URL=https://scan.themelio.org npm run build && cp package.json build)

rsync -avz --progress --rsync-path="sudo rsync" ./frontend/build/ debian@web.themelio.org:/var/www/melscan-frontend/

# Testnet frontend
(cd frontend && rm -rfv build && VITE_BASE_URL=https://scan-testnet.themelio.org npm run build && cp package.json build)

rsync -avz --progress --rsync-path="sudo rsync" ./frontend/build/ debian@web.themelio.org:/var/www/melscan-frontend-testnet/

# Backend
(cd backend && cargo build  --release --locked --target x86_64-unknown-linux-musl)
rsync -avz --delete --progress --rsync-path="sudo rsync" ./backend/target/x86_64-unknown-linux-musl/release/melscan-backend debian@web.themelio.org:/usr/local/bin/melscan-backend


# Restart services
ssh -v debian@web.themelio.org sudo systemctl restart melscan-mainnet-backend
ssh -v debian@web.themelio.org sudo systemctl restart melscan-mainnet-frontend
ssh -v debian@web.themelio.org sudo systemctl restart melscan-testnet-backend
ssh -v debian@web.themelio.org sudo systemctl restart melscan-testnet-frontend