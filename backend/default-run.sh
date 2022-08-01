#!/bin/bash
cargo run --release -- --connect 127.0.0.1:11814  --listen 127.0.0.1:13000 --blkidx-db /var/melscan/block-indexer.db
#cargo run --release -- --connect 146.59.84.29:11814 --listen 127.0.0.1:13000
