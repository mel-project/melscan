<script lang="ts">
	import { LayerCake, Svg } from 'layercake';
	import Sankey from './layercake/Sankey.svelte';
	import { melscan } from '@utils/common';
	import type { BlockHeight, Vec, TxHash, Transaction, CoinSpend, CoinID } from '@utils/types';
	import { onMount } from 'svelte';
	export let height: BlockHeight;
	export let txhash: TxHash;
	export let transaction: Transaction;
	export let fetch;

	let node_id = (txhash, index) => `${txhash}-${index}`;

	const getDataAndRes: () => Promise<[any, CoinSpend[]]> = async () => {

		console.log(transaction.inputs);
		let dirty_res: (null | CoinSpend)[] = await melscan(fetch, `/raw/blocks/${height}/${txhash}/spends`);
		let res = dirty_res.filter((i: null | CoinSpend)=>i);



		let input_locations = transaction.inputs.map((input) => (
			{
				coinid: {
					txhash: input.txhash,
					index: input.index,
				},
				txhash,
				height
			}));

		let locations = res.concat(input_locations);


		let nodes_set = new Set()
		locations.forEach((location: CoinSpend) => {
			nodes_set.add(location.txhash)
			nodes_set.add(location.coinid.txhash )
		});


		let nodes = Array.from(nodes_set).map(id=>({id}));
		console.log("nodes", nodes)

		let links = [];
		locations.forEach((location: CoinSpend) => {
			let id = `${location.coinid.txhash}-${location.coinid.index}`;
			nodes.push({ id });
			console.log('location.txhash', location.txhash);

			// outputs go from this transaction to the this coin
			links.push({
				source: location.coinid.txhash,
				target: id,
				value: 1
			});

			// this utxo was spent at location.txhash
			links.push({
				source: id,
				target: location.txhash,
				value: 1
			});
		});




		return [{ nodes, links }, res];
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
		<div class="data-container">
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
		</div>
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
		width: 90vw;
		height: 90vh;
		display: flex;
		flex-direction: row;
		gap: 2em;
	}
	.data1 {
		width: 45%;
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
