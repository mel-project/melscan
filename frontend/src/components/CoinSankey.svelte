<script lang="ts">
	import { LayerCake, Svg } from 'layercake';
	import Sankey from './layercake/Sankey.svelte';
	import { melscan } from '@utils/common';
	import {
		type BlockHeight,
		type TxHash,
		type Transaction,
		type CoinCrawl,
		Denom,
		type CoinID
	} from '@utils/types';
	import { identity } from 'svelte/internal';
	export let height: BlockHeight;
	export let txhash: TxHash;
	export let transaction: Transaction;
	export let fetch;
	export let links;
	const abbrString = (s, len) => {
		return s.substring(0, len) + '...' + s.substring(s.length - len, s.length);
	};
	const coinid_str = (coinid: CoinID) => coinid.txhash + '-' + coinid.index;
	const decode_denom = (denom: string) => {
		if (denom == Denom.MEL){
			return "MEL"
		}
		else if(denom == Denom.SYM){
			return 'SYM'
		}
		else if(denom == Denom.ERG){
			return 'ERG'
		}
		else {
			return abbrString(denom, 4)
		}
	}
	const getDataAndRes: () => Promise<[any, CoinCrawl]> = async () => {
		console.log(transaction.inputs);
		let res = (await melscan(
			fetch,
			`/raw/blocks/${height}/transactions/${txhash}/crawl`
		)) as CoinCrawl;

		let crawls = res.crawls.filter(({coindata})=>coindata.denom == Denom.MEL)
		let coin_nodes = crawls
			.map(({ coinid, coindata }) => {
				let id = coinid_str(coinid);
				let coin_node = {
					id,
					label: `${id.split('-')[1]} [${(coindata.value / 1_000_000).toFixed(
						6
					)} ${decode_denom(coindata.denom)} => ${abbrString(coindata.covhash, 4)}]`
				};
				return coin_node
		});
		
		let nodes_set = new Set();
		let transaction_nodes = crawls
		.flatMap(({coinid, spender}) =>  [coinid.txhash, spender ? spender[1] : null])
		.filter(i=>i)
		.map((txhash) => {
			if(nodes_set.has(txhash)) return null
			nodes_set.add(txhash)
			return {id: txhash, label: abbrString(txhash, 4)}
		})
		.filter(i=>i)

		
		let nonspend_links = crawls.flatMap(({ coinid, coindata, spender }) => {
			let id = coinid_str(coinid);
			let transaction_to_coin = {
				source: coinid.txhash,
				target: id,
				value: coindata.value,
			};
			return transaction_to_coin
		});
		let spend_links = crawls
		.filter(({spender})=>spender)
		.map(({coinid, coindata, spender}) => {
			let [_, spender_txhash] = spender;
			let id = coinid_str(coinid);
			return {
				source: id,
				target: spender_txhash,
				value: coindata.value
			};
			
		})
		let nodes = transaction_nodes.concat(coin_nodes)
		links = nonspend_links.concat(spend_links)

		nodes.push({ id: 'Fees', label: "Fees" });
		links.push({
			source: txhash,
			target: 'Fees',
			value: transaction.fee
		});
		
		return [{ nodes, links }, {crawls}];
	};
</script>

<div class="chart-container">
	{#await getDataAndRes()}
		<i>loading...</i>
	{:then [data, res]}
		<div class="data1">
			{#if Object.keys(data.links).length > 0}
				<LayerCake data={JSON.parse(JSON.stringify(data))}>
					<Svg>
						<Sankey
							nodePadding={50}
							colorNodes={(d) => {
								if (d.id === 'Fees') {
									return '#ff0000';
								}
								if (!d.id.includes('-')) {
									return '#00bbff';
								}
								else {
									return '#ccc';
								}
							}}
							colorLinks={(d) => '#ccc'}
						/>
					</Svg>
				</LayerCake>
			{/if}
		</div>
		<!-- <div class="data-container">
			Server Response
			<div class="data">
				{#each res.crawls as {coinid, coindata, spender}}
					<div class="info">
						<div>{JSON.stringify(coinid)}</div>
						<div>{JSON.stringify(coindata)}</div>
						
						{#if spender}
							<div>

								Spent: <a href="/blocks/{spender[0]}/{spender[1]}"
									>{spender[0]}/{spender[1]}</a
								>
							</div>
						{/if}
					</div>
				{/each}
			</div>

			Transaction Inputs
			<div class="data">
				{#each transaction.inputs as input}
					<div class="info">
						<div>{JSON.stringify(input)}</div>
					</div>
				{/each}
			</div>

			Nodes
			<div class="data">
				{#each data.nodes as node}
					<div class="info">{node.id}</div>
				{/each}
			</div>

			Links
			<div class="data">
				{#each data.links as l}
					<div class="info">{JSON.stringify(l)}</div>
				{/each}
			</div>
		</div> -->
	{:catch error}
		<i>{error}</i>
	{/await}
</div>

<style>
	/*
	  The wrapper div needs to have an explicit width and height in CSS.
	  It can also be a flexbox child or CSS grid element.
	  The point being it needs dimensions since the <LayerCake> element will
	  expand to fill it.
	*/
	.chart-container {
		width: 100%;
		height: 60rem;
		display: flex;
		flex-direction: row;
		gap: 2em;
	}
	.data1 {
		width: 100%;
		height: 80%;
	}
	.data-container {
		display: flex;
		flex-direction: column;
		gap: 1em;
		overflow-y: scroll;
		overflow-x: visible;
	}
	.data {
		border: 1px solid grey;
		width: 100%;
		padding: 1em;
		display: flex;
		flex-direction: column;
		gap: 1em;
	}

	.info {
		border-bottom: 1px solid red;
	}
</style>
