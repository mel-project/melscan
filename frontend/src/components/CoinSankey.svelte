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
	const coinid_str = (coinid: CoinID) => coinid.txhash + "-" + coinid.index
	const getDataAndRes: () => Promise<[any, CoinCrawl]> = async () => {
		console.log(transaction.inputs);
		let crawl = (await melscan(
			fetch,
			`/raw/blocks/${height}/transactions/${txhash}/crawl`
		)) as CoinCrawl;

		let nodes_set = new Set();

		crawl.coins.forEach(({coinid, coindata, spender}) => {
			// add the transactions to the nodeset
			if (coindata.denom === Denom.MEL) {
				nodes_set.add(coinid_str(coinid));
				nodes_set.add(coinid.txhash);
			}
			// if spent, add spending txhash to nodeset
			if(spender){
				let [height, txhash] = spender;
				nodes_set.add(txhash)
			}
		});


		let links = crawl.coins.map(({coinid, coindata, spender})=>{
			
		});
		links.push({
			source: txhash,
			target: 'Fees',
			value: transaction.fee
		});
		
		

		// let nodes = Array.from(nodes_set).map((id: string) => {
		// 	if (id === 'Fees') {
		// 		return { id };
		// 	}
		// 	if (id.includes('-')) {
		// 		return {
		// 			id: id,
		// 			label: `${id.split('-')[1]} [${(crawl.coin_contents[id].value / 1_000_000).toFixed(
		// 				6
		// 			)} MEL => ${abbrString(crawl.coin_contents[id].covhash, 4)}]`
		// 		};
		// 	}
		// 	return { id, label: abbrString(id, 10) };
		// });
		// console.log('nodes', nodes);

		// let links = Array.from(links_set);

		return [{ nodes, links }, crawl];
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
								if (!res.coin_spenders[d.id]) {
									return '#ffbb00';
								} else {
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
				{#each res as location}
					<div class="info">
						<div>{JSON.stringify(location.coinid)}</div>
						<div>
							Spent: <a href="/blocks/{location.height}/{location.txhash}"
								>{location.height}/{location.txhash}</a
							>
						</div>
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
	/* .data-container {
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
	} */
</style>
