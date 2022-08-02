<script>
	import { onMount } from 'svelte';
	import { melscan, cyrb53 } from '@utils/common';
	import * as vis from 'vis-network';
	import * as visd from 'vis-data';
	export let height;
	export let txhash;

	let container;

	const getCoinCrawl = async (height, txhash) =>
		await melscan(fetch, `/raw/blocks/${height}/transactions/${txhash}/crawl`);

	const abbrString = (s, len) => {
		return s.substring(0, len) + '...' + s.substring(s.length - len, s.length);
	};

	const coinid_str = (c) => c.txhash + '-' + c.index;

	onMount(async () => {
		let nodes = new visd.DataSet([]);
		let edges = new visd.DataSet([]);

		const refresh = async (height, txhash) => {
			const crawl = await getCoinCrawl(height, txhash);
			crawl.crawls.forEach(({ coinid, coindata, coinheight, spender }) => {
				let coin_hue = cyrb53(coindata.covhash) % 360;
				console.log('wait...');
				// await new Promise((r) => setTimeout(r, 10));
				console.log('dunn');
				nodes.update({
					id: coinid_str(coinid),
					label: `Output ${coinid.index}\n${(coindata.value / 1_000_000).toFixed(6)} ${
						coindata.denom
					}\n${abbrString(coindata.covhash, 6)}`,
					shape: 'diamond',
					size: 10,
					// mass: 200,
					color: `hsl(${coin_hue}, 80%, 40%)`
				});
				nodes.update({
					id: coinid.txhash,
					label: abbrString(coinid.txhash, 10),
					shape: 'box',
					size: 20,
					__height: coinheight
				});
				edges.update({ from: coinid.txhash, to: coinid_str(coinid), color: { inherit: 'to' } });
				if (spender) {
					let [height, txhash] = spender;
					edges.update({ from: coinid_str(coinid), to: txhash, color: { inherit: 'from' } });
					nodes.update({
						id: txhash,
						label: abbrString(txhash, 10),
						shape: 'box',
						size: 20,
						__height: height
					});
				}
			});
		};
		await refresh(height, txhash);

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
				stabilization: true,
				hierarchicalRepulsion: {
					avoidOverlap: 1,
					damping: 0.1,
					springLength: 50
				},
				maxVelocity: 100
			}
		};

		// initialize your network!
		const network = new vis.Network(container, data, options);
		network.on('selectNode', (obj) => {
			obj.nodes.forEach((id) => {
				console.log('clicked', id);
				let node = nodes.get(id);
				if ('__height' in node) {
					let height = node.__height;
					console.log('height', node.__height);
					refresh(height, node.id);
				}
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
