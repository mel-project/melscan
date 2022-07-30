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

	let res: Vec<CoinSpend> = [];
	let locations: CoinSpend[] = [];
	let nodes = [{ id: txhash }];
	let links = [];
	let data;
	let node_id = (txhash, index) => `${txhash}-${index}`;

	onMount(async () => {
		try {
			let node_set = new Set();
			console.log(transaction.inputs);
			res = await melscan(fetch, `/raw/blocks/${height}/${txhash}/spends`);
			transaction.inputs.forEach((input) => {
				let id =  node_id(input.txhash, input.index);

				links.push({
					source: id,
					target: txhash,
					value: 1,
				})
				node_set.add(id);
				return node;
			});
			res.forEach((location: CoinSpend) => {
				let id = `${location.coinid.txhash}-${location.coinid.index}`;
				nodes.push({ id });
				node_set.add(location.txhash);
				links.push({
					source: id,
					target: location.txhash,
					value: 1
				});
			});

			let node_array = Array.from(node_set).map((id)=>({id}))
			//done to update nodes and links in the dom
			links = links.concat();
			nodes = nodes.concat(node_array as any);

			data = { nodes, links }
		} catch (e) {}
	});

	
</script>

<div class="chart-container">
	<div class="data1">
		{#if Object.keys(links).length > 0}
			<LayerCake {data}>
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
			{#each nodes as node}
				<div class="info">{node.id}</div>
			{/each}
		</div>
		
		<!-- Links
		<div class="data">
			{#each links as l}
				<div class="info">{JSON.stringify(l)}</div>
			{/each}
		</div> -->
	</div>
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
		width: 75%;
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
