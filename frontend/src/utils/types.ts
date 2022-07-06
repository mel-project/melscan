export type bool = Boolean;

export type u128 = number;
export type u64 = number;
export type u32 = number;
export type u8 = number;

export type f64 = number;
export type f32 = number;

export type Vec<T> = T[];

export type BlockHeight = u64;
export type CoinValue = u128;

export type HashVal = [u8,u8,u8,u8,u8,u8,u8,u8,
                        u8,u8,u8,u8,u8,u8,u8,u8,
                        u8,u8,u8,u8,u8,u8,u8,u8,
                        u8,u8,u8,u8,u8,u8,u8,u8]
export enum NetID {
    Testnet = 0x01,
    Custom02 = 0x02,
    Custom03 = 0x03,
    Custom04 = 0x04,
    Custom05 = 0x05,
    Custom06 = 0x06,
    Custom07 = 0x07,
    Custom08 = 0x08,
    Mainnet = 0xff,
}

interface Denom {
     Mel,
     Sym,
     Erg,
 }
 
export interface Header{
     network: NetID,
     previous: HashVal,
     height: BlockHeight,
     history_hash: HashVal,
     coins_hash: HashVal,
     transactions_hash: HashVal,
     fee_pool: CoinValue,
     fee_multiplier: u128,
     dosc_speed: u128,
     pools_hash: HashVal,
     stakes_hash: HashVal,
}
export interface TransactionSummary{
     hash: String,
     shorthash: String,
     height: u64,
     weight: u128,
     mel_moved: u128,
}
export interface BlockSummary{
     header: Header,
     total_weight: u128,
     reward_amount: u128,
     transactions: Vec<TransactionSummary>,
}
export interface PoolKey {
     left: Denom,
     right: Denom,
 }
 
 
 // 2 million cached pooldataitems is 64 mb
 // 1 item is 256 bits
export interface PoolDataItem {
     date: u64,
     height: u64,
     price: f64,
     liquidity: f64,
     ergs_per_mel: f64,
 }
 