<script>
	import { onMount } from 'svelte';
	import { melscan, cyrb53 } from '@utils/common';
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
			let coin_hue = cyrb53(coin_data.covhash) % 360;
			nodes.add({
				id: coinid_str,
				label: `Output ${coinid_str.split('-')[1]}\n${(coin_data.value / 1_000_000).toFixed(6)} ${
					coin_data.denom
				}\n${abbrString(coin_data.covhash, 6)}`,
				shape: 'diamond',
				size: 10,
				// mass: 200,
				color: `hsl(${coin_hue}, 50%, 50%)`,
				title: 'Title'
			});
			try {
				nodes.add({
					id: coinid_str.split('-')[0],
					label: abbrString(coinid_str.split('-')[0], 10),
					shape: 'box',
					size: 20,
					title: 'Title'
				});
			} catch {}
			edges.add({ from: coinid_str.split('-')[0], to: coinid_str, color: { inherit: 'to' } });
		});

		// create an array with edges
		Object.entries(crawl.coin_spenders).forEach(([coinid_str, txhash]) => {
			try {
				nodes.add({
					id: txhash,
					label: abbrString(txhash, 10),
					shape: 'box',
					title: 'Title'
				});
			} catch {}
			edges.add({ from: coinid_str, to: txhash, color: { inherit: 'from' } });
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
					direction: 'UD',
					sortMethod: 'directed',
					shakeTowards: 'roots'
				}
			},
			nodes: {},
			edges: {
				arrows: 'to'
			},
			interaction: {
				hover: true
			},
			physics: {
				hierarchicalRepulsion: {
					avoidOverlap: 1,
					damping: 0.4,
					springLength: 1
				},
				maxVelocity: 100
			}
		};

		// initialize your network!
		const network = new vis.Network(container, data, options);
		network.on('selectNode', (obj) => {
			obj.nodes.forEach((id) => {
				console.log('clicked', id);
			});
		});
	});
</script>

<template>
	<div class="root" bind:this={container} />
</template>

<style>
	.root {
		height: 100%;
		border: 1px solid #aaa;
		background-color: white;
	}
</style>
