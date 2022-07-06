import type { BlockSummary, bool, f64, PoolDataItem, PoolKey, Vec } from './types';

export interface Overview {
    erg_per_mel: f64,
    sym_per_mel: f64,
    recent_blocks: Vec<BlockSummary>,
}


export interface PoolTemplate {
    testnet: bool,
    friendly_denom: String,
    pool_key: PoolKey,
    last_item: PoolDataItem,
}