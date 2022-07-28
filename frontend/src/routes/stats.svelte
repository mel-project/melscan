<script context="module" lang="ts">
	import { melscan, type LoadFunction } from '@utils/common';
	export let load: LoadFunction<any> = async (loadEvent) => {
		let props = {
			leaderboard: await melscan(loadEvent.fetch, '/raw/leaderboard/MEL')
		};
		return {
			status: 200,
			props
		};
	};
</script>

<script lang="ts">
	import BreadCrumbs from '@components/BreadCrumbs.svelte';

	import GraphPlot from '@components/GraphPlot.svelte';
	import TopNav from '@components/TopNav.svelte';
	import { queryGraph } from '@utils/common';
	import { BreadCrumb } from '@utils/page-types';

	let breadcrumbs = [BreadCrumb('Melscan', '/'), BreadCrumb('Stats', '.')];

	export let leaderboard: { [key: string]: number };

	let sortedLeaderboard = Object.entries(leaderboard).sort((a, b) => b[1] - a[1]);
	let totalBalance = Object.values(leaderboard).reduce((a, b) => a + b);
</script>

<template>
	<TopNav>
		<BreadCrumbs {breadcrumbs} />
	</TopNav>

	<div class="container mx-auto max-w-screen-lg">
		<div class="grid grid-cols-1 md:grid-cols-2 mt-8 mb-8">
			<div class="col-span-2 mb-3">
				<h3 class="text-2xl font-bold">Money supply</h3>
			</div>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-12 mt-8 mb-8">
			<div class="col-span-4">
				<b>MEL</b><br /><br />
				<GraphPlot
					fetchData={async (start, end) =>
						await queryGraph({
							id: {
								type: 'coin_supply',
								denom: 'MEL'
							},
							start: start,
							end: end
						})}
					unit="MEL"
					label="Supply"
					stroke="rgba(0, 0, 40, 1)"
					fill="rgba(0, 0, 255, 0.1)"
				/>
			</div>
			<div class="col-span-4">
				<b>SYM (PoS token)</b><br /><br />
				<GraphPlot
					fetchData={async (start, end) =>
						await queryGraph({
							id: {
								type: 'coin_supply',
								denom: 'SYM'
							},
							start: start,
							end: end
						})}
					unit="SYM"
					label="Supply"
					stroke="rgba(40, 0, 0, 1)"
					fill="rgba(255, 0, 0, 0.1)"
				/>
			</div>
			<div class="col-span-4">
				<b>ERG (minting intermediary)</b><br /><br />
				<GraphPlot
					fetchData={async (start, end) =>
						await queryGraph({
							id: {
								type: 'coin_supply',
								denom: 'ERG'
							},
							start: start,
							end: end
						})}
					unit="ERG"
					label="Supply"
					stroke="rgba(0, 40, 0, 1)"
					fill="rgba(0, 255, 0, 0.1)"
				/>
			</div>
		</div>
	</div>

	<div class="container mx-auto max-w-screen-lg mt-8 mb-8">
		<div class="col-span-2 mb-3">
			<h3 class="text-2xl font-bold">Top 50 addresses</h3>
		</div>
		<table class="table-auto w-full mt-3">
			<thead class="text-left text-sm text-black text-opacity-50">
				<tr>
					<th>Ranking</th>
					<th>Address</th>
					<th class="text-right">MEL balance</th>
					<th class="text-right">Percentage of supply</th>
				</tr>
			</thead>
			<tbody class="text-sm">
				{#each sortedLeaderboard as [address, balance], i}
					{#if i < 50}
						<tr>
							<td>{i + 1}</td>
							<td><a href={`/address/${address}`} class="text-blue-800">{address}</a></td>
							<td class="text-right">{balance.toFixed(6)}</td>
							<td class="text-right">{((100.0 * balance) / totalBalance).toFixed(4)}%</td>
						</tr>
					{/if}
				{/each}
			</tbody>
		</table>
	</div>
</template>
