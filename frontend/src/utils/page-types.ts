import type { BlockSummary, f64, Vec } from './types';

interface Overview {
    erg_per_mel: f64,
    sym_per_mel: f64,
    recent_blocks: Vec<BlockSummary>,
}