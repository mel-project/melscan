<script lang="ts">
	import { LayerCake, Svg } from 'layercake';
	import Sankey from './layercake/Sankey.svelte';
	import { melscan } from '@utils/common';
	import type { BlockHeight, TxHash, Transaction, CoinCrawl } from '@utils/types';
	export let height: BlockHeight;
	export let txhash: TxHash;
	export let transaction: Transaction;
	export let fetch;

	const getDataAndRes: () => Promise<[any, CoinCrawl]> = async () => {
		console.log(transaction.inputs);
		let crawl = (await melscan(
			fetch,
			`/raw/blocks/${height}/transactions/${txhash}/crawl`
		)) as CoinCrawl;

		let nodes_set = new Set();
		Object.keys(crawl.coin_contents).forEach((coinid_str) => {
			nodes_set.add(coinid_str);
			nodes_set.add(coinid_str.split('-')[0]);
		});
		Object.values(crawl.coin_spenders).forEach((txhash) => {
			nodes_set.add(txhash);
		});

		let links_set = new Set();
		// coin creation
		Object.keys(crawl.coin_contents).forEach((coinid_str) => {
			links_set.add({
				source: coinid_str.split('-')[0],
				target: coinid_str,
				value: crawl.coin_contents[coinid_str].value
			});
		});
		// coin spend
		Object.entries(crawl.coin_spenders).forEach(([coinid_str, txhash]) => {
			links_set.add({
				source: coinid_str,
				target: txhash,
				value: crawl.coin_contents[coinid_str].value
			});
		});

		let nodes = Array.from(nodes_set).map((id) => ({ id }));
		console.log('nodes', nodes);

		let links = Array.from(links_set);

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
						<Sankey colorNodes={(d) => '#00bbff'} colorLinks={(d) => '#00bbff35'} />
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
		height: 50rem;
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
