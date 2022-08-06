<script>
	import { onMount } from 'svelte';
	import { melscan, cyrb53 } from '@utils/common';
	import * as vis from 'vis-network';
	import * as visd from 'vis-data';
	export let height;
	export let txhash;
	export let embed = false;
	let container;

	const getCoinCrawl = async (height, txhash) =>
		await melscan(fetch, `/raw/blocks/${height}/transactions/${txhash}/crawl`);

	const abbrString = (s, len) => {
		return s.substring(0, len) + '...' + s.substring(s.length - len, s.length);
	};

	const coinid_str = (c) => c.txhash + '-' + c.index;

	let loading = false;

	onMount(async () => {
		let nodes = new visd.DataSet([]);
		let edges = new visd.DataSet([]);

		// fix the level of a particular node
		const fixLevel = (left, right) => {
			let ln = nodes.get(left);
			let rn = nodes.get(right);

			if (rn.level <= ln.level) {
				rn.level = ln.level + 1;
				return false;
			}

			ln.hidden = false;
			rn.hidden = false;

			nodes.update([ln, rn]);
			return true;
		};
		let first = true;

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
					shakeTowards: 'leaves'
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
					springLength: 100
				},
				maxVelocity: 100
			}
		};
		// initialize your network!

		const refresh = async (height, txhash, initLevel = 0) => {
			loading = true;
			try {
				const crawl = await getCoinCrawl(height, txhash);
				// network.setData({ nodes: new visd.DataSet([]), edges: new visd.DataSet([]) });
				const addTxhash = (txhash, height) => {
					if (!nodes.get(txhash))
						nodes.update({
							id: txhash,
							label: `${abbrString(txhash, 10)}\n(block ${height})`,
							shape: 'box',
							size: 20,
							color: '#bbb',
							__height: height
							// level: initLevel,
							// hidden: true
						});
				};

				crawl.crawls.forEach(({ coinid, coindata, coinheight, spender }) => {
					let coin_hue = cyrb53(coindata.covhash) % 360;
					nodes.update({
						id: coinid_str(coinid),
						label: `Output ${coinid.index}\n${(coindata.value / 1_000_000).toFixed(6)} ${
							coindata.denom
						}\n${abbrString(coindata.covhash, 6)}`,
						shape: 'diamond',
						size: 10,
						color: `hsl(${coin_hue}, 80%, 40%)`
						// level: initLevel
						// hidden: true
					});
					if (first) {
						first = false;
						// nodes.update({ id: coinid_str(coinid), level: initLevel });
						console.log('first is', coinid_str(coinid));
					}
					if (!nodes.get(coinid.txhash)) addTxhash(coinid.txhash, coinheight);
					edges.update({
						id: `${coinid.txhash}/${coinid_str(coinid)}`,
						from: coinid.txhash,
						to: coinid_str(coinid),
						color: { inherit: 'to' }
					});
					if (spender) {
						let [height, txhash] = spender;
						edges.update({
							id: `${coinid_str(coinid)}/${txhash}`,
							from: coinid_str(coinid),
							to: txhash,
							color: { inherit: 'from' }
						});
						addTxhash(txhash, height);
					}
				});

				nodes.update({
					id: txhash,
					color: '#22cc44',
					__explored: true
				});

				// fix level in a loop
				// while (true) {
				// 	let badcount = 0;
				// 	edges.forEach((i, _) => {
				// 		if (!fixLevel(i.from, i.to)) {
				// 			badcount += 1;
				// 		}
				// 	});
				// 	console.log(badcount);
				// 	if (badcount === 0) {
				// 		break;
				// 	}
				// }
				// network.redraw();
			} finally {
				// network.setData(data);
				loading = false;
			}
		};
		await refresh(height, txhash);
		const network = new vis.Network(container, data, options);
		network.on('selectNode', (obj) => {
			obj.nodes.forEach(async (id) => {
				console.log('clicked', id);
				let node = nodes.get(id);
				if ('__height' in node) {
					if (!('__explored' in node)) {
						let height = node.__height;
						console.log('height', node.__height);
						await refresh(height, node.id, node.level);
					}
					height = node.__height;
					txhash = node.id;
				}
			});
		});
	});
</script>

<template>
	<div class="superroot">
		{#if !embed}
			<div class="pop">
				Transaction <a href={`/blocks/${height}/${txhash}`} class="text-blue-800 font-bold"
					>{txhash}</a
				>
			</div>
		{/if}
		<div class="root" bind:this={container} class:loading />
	</div>
</template>

<style>
	.pop {
		position: absolute;
		top: 2rem;
		left: 2rem;
		z-index: 2000;
		background-color: white;
		border: 1px solid #aaa;
		padding: 1rem;
	}

	.superroot {
		height: 100%;
		position: relative;
	}
	.root {
		height: 100%;
		border: 1px solid #aaa;
		background-color: white;
	}

	.loading {
		opacity: 0.5;
		pointer-events: none;
		background-color: #fefefe;
	}
</style>
