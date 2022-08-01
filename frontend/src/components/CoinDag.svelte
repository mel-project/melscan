<script>
	import { onMount } from 'svelte';
	import { melscan } from '@utils/common';
	import * as vis from 'vis-network';
	import * as visd from 'vis-data';
	export let height;
	export let txhash;

	let container;

	const getCoinCrawl = async () =>
		await melscan(fetch, `/raw/blocks/${height}/transactions/${txhash}/crawl`);

	const abbrString = (s, len) => {
		return s.substring(0, len) + '...' + s.substring(s.length - len, s.length);
	};

	onMount(async () => {
		console.log('vis is', vis);
		const crawl = await getCoinCrawl();
		let nodes = new visd.DataSet([]);
		let edges = new visd.DataSet([]);
		Object.entries(crawl.coin_contents).forEach(([coinid_str, coin_data]) => {
			nodes.add({
				id: coinid_str,
				label: `Output ${coinid_str.split('-')[1]}\n${(coin_data.value / 1_000_000).toFixed(6)} ${
					coin_data.denom
				}\n${abbrString(coin_data.covhash, 6)}`,
				shape: 'diamond',
				size: 10
			});
			try {
				nodes.add({
					id: coinid_str.split('-')[0],
					label: abbrString(coinid_str.split('-')[0], 10),
					shape: 'box'
				});
			} catch {}
			edges.add({ from: coinid_str.split('-')[0], to: coinid_str });
		});

		// create an array with edges
		Object.entries(crawl.coin_spenders).forEach(([coinid_str, txhash]) => {
			try {
				nodes.add({
					id: txhash,
					label: abbrString(txhash, 10),
					shape: 'box'
				});
			} catch {}
			edges.add({ from: coinid_str, to: txhash });
		});

		// create a network

		// provide the data in the vis format
		const data = {
			nodes: nodes,
			edges: edges
		};
		const options = {
			layout: {
				hierarchical: {
					enabled: true,
					direction: 'LR',
					sortMethod: 'directed'
				}
			},
			nodes: {},
			edges: {
				arrows: 'to'
			}
		};

		// initialize your network!
		const network = new vis.Network(container, data, options);
	});
</script>

<template>
	<div class="root" bind:this={container} />
</template>

<style>
	.root {
		height: 40rem;
		border: 1px solid #aaa;
		background-color: white;
	}
</style>
