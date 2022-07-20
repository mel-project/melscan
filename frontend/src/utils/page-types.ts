<<<<<<< Updated upstream
import type { BlockSummary, bool, f64, PoolDataItem, PoolKey, PoolState, u64, Vec } from './types';
=======
import type { BlockSummary, bool, BTreeMap, CoinData, CoinDataHeight, CoinID, f64, MicroUnit, PoolDataItem, PoolKey, PoolState, Transaction, TxHash, u64, Vec } from './types';
>>>>>>> Stashed changes

export interface Overview {
	erg_per_mel: f64;
	sym_per_mel: f64;
	recent_blocks: Vec<BlockSummary>;
}

export interface PoolTemplate {
	testnet: bool;
	friendly_denom: String;
	pool_key: PoolKey;
	last_item: PoolDataItem;
}

export interface PoolInfo {
	pool_state: PoolState;
	latest_item: PoolDataItem;
}


export interface TransactionResponse {
	testnet: bool;
	txhash: TxHash;
	txhash_abbr: String;
	height: u64;
	transaction: Transaction;
	inputs_with_cdh: Vec<[number, CoinID, CoinDataHeight, MicroUnit, string]>;
	outputs: Vec<[number, CoinData, MicroUnit]>;
	fee: MicroUnit;
	base_fee: MicroUnit;
	tips: MicroUnit;
	net_loss: BTreeMap<string, Vec<MicroUnit>>;
	net_gain: BTreeMap<string, Vec<MicroUnit>>;
	gross_gain: Vec<MicroUnit>;
	weight: number;
	kind: string;
}

export interface BreadCrumb {
	title: string;
	href: string;
}

export let BreadCrumb: (t: string, h: string) => BreadCrumb = (title, href) => {
	return {
		title,
		href
	};
};

// A query for a graph
export interface GraphQuery {
	id: GraphId;
	start: Date | null;
	end: Date | null;
}

export type GraphId =
	| {
			type: 'pool_price';
			from: string;
			to: string;
	  }
	| {
			type: 'pool_liquidity';
			from: string;
			to: string;
	  }
	| {
			type: 'coin_supply';
			denom: string;
	  };

export interface GraphDatum {
	height: number;
	date: Date;
	value: number;
}
