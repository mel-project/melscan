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

export type Obj<T> = { [key: string]: T };
export type BTreeMap<T extends string | number | symbol, K> = Record<T, K>;

export type HashVal = String;

export enum NetID {
	Testnet = 0x01,
	Custom02 = 0x02,
	Custom03 = 0x03,
	Custom04 = 0x04,
	Custom05 = 0x05,
	Custom06 = 0x06,
	Custom07 = 0x07,
	Custom08 = 0x08,
	Mainnet = 0xff
}

export enum Denom {
	MEL = 'MEL',
	SYM = 'SYM',
	ERG = 'ERG'
}

export interface Header {
	network: NetID;
	previous: HashVal;
	height: BlockHeight;
	history_hash: HashVal;
	coins_hash: HashVal;
	transactions_hash: HashVal;
	fee_pool: CoinValue;
	fee_multiplier: u128;
	dosc_speed: u128;
	pools_hash: HashVal;
	stakes_hash: HashVal;
}
export interface TransactionSummary {
	hash: String;
	shorthash: String;
	height: u64;
	weight: u128;
	mel_moved: u128;
}

export interface PoolKey {
	left: Denom;
	right: Denom;
}

// 2 million cached pooldataitems is 64 mb
// 1 item is 256 bits
export interface PoolDataItem {
	date: u64;
	height: u64;
	price: f64;
	liquidity: f64;
	ergs_per_mel: f64;
}

export interface PoolState {
	lefts: u128;
	rights: u128;
	price_accum: u128;
	liqs: u128;
}

export interface CoinData {
	covhash: Address;
	value: CoinValue;
	denom: Denom;

	additional_data: Vec<u8>;
}
export interface Transaction {
	kind: TxKind;
	inputs: Vec<CoinID>;
	outputs: Vec<CoinData>;
	fee: CoinValue;
	covenants: Vec<Vec<u8>>;
	data: string;
	sigs: Vec<string>;
}

export interface CoinID {
	txhash: TxHash;
	index: u8;
}

export type TxHash = HashVal;
export type Address = HashVal;

/// Transaction represents an individual, serializable Themelio transaction.

export enum TxKind {
	DoscMint = 0x50,
	Faucet = 0xff,
	LiqDeposit = 0x52,
	LiqWithdraw = 0x53,
	Normal = 0x00,
	Stake = 0x10,
	Swap = 0x51
}

export type MicroUnit = [number, string];

export interface CoinDataHeight {
	coin_data: CoinData;
	height: BlockHeight;
}

export interface CoinSpend {
	coinid: CoinID;
	txhash: TxHash;
	height: BlockHeight;
}

export interface CoinCrawl {
	coin_contents: [CoinID, CoinData];
	coin_spenders: { [key: string]: string };
}
