<script context="module" lang="ts">
	import { melscan, type LoadFunction } from '@utils/common';

	export interface AddressSummary {
		balances: { [key: string]: number };
		transactions: {
			height: number;
			date: Date;
			txhash: string;
			deltas: { [key: string]: number };
		}[];
	}

	export let load: LoadFunction<any> = async (loadEvent) => {
		let { covhash } = loadEvent.params;
		let url = `/raw/address/${covhash}`;
		let res = (await melscan(loadEvent.fetch, url)) as any;
		res.transactions = res.transactions.map((t: any) => {
			t.date = new Date(t.date);
			return t;
		});

		return {
			status: 200,
			props: { summary: res as AddressSummary, covhash }
		};
	};
</script>

<script lang="ts">
	import BreadCrumbs from '@components/BreadCrumbs.svelte';
	import GraphPlot from '@components/GraphPlot.svelte';

	import TopNav from '@components/TopNav.svelte';
	import { BreadCrumb } from '@utils/page-types';

	export let summary: AddressSummary;
	export let covhash: string;

	let firstHeight = summary.transactions.length > 0 ? summary.transactions[0].height : 0;
	let firstDate =
		summary.transactions.length > 0
			? summary.transactions[0].date.toLocaleDateString('en-GB', {
					year: 'numeric',
					month: 'long',
					day: 'numeric'
			  })
			: '';

	let balanceHistory = (() => {
		let accum = 0;
		return summary.transactions.map((t) => {
			if ('MEL' in t.deltas) accum += t.deltas['MEL'];
			return {
				height: t.height,
				date: t.date,
				value: accum
			};
		});
	})();

	let transactionCount = 50;

	$: truncatedTransactions = summary.transactions.slice().reverse().slice(0, transactionCount);
</script>

<template>
	<TopNav>
		<BreadCrumbs
			breadcrumbs={[BreadCrumb('Melscan', '/'), BreadCrumb(`Address ${covhash}`, `.`)]}
		/>
	</TopNav>

	<div class="container mx-auto max-w-screen-lg">
		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Summary</h3>
		</div>

		<div class="m-3">
			<table class="table-fixed w-full text-sm text-left">
				<tbody>
					<tr>
						<td class="text-black text-opacity-50 font-bold w-1/2">First seen</td>
						<td>
							<a class="text-blue-800 font-medium" href={`/blocks/${firstHeight}`}>{firstHeight}</a>
							({firstDate})
						</td>
					</tr>
					<tr>
						<td class="text-black text-opacity-50 font-bold w-1/2">Total balance</td>
						<td>
							{#each Object.entries(summary.balances) as [denom, balance]}
								<b class="font-medium">{balance.toFixed(6)}</b>&nbsp;<i>{denom}</i>&nbsp;&nbsp;
							{/each}
						</td>
					</tr>
				</tbody>
			</table>
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Balance history (MEL)</h3>
			<div class="m-3">
				<GraphPlot
					stepped
					unit="MEL"
					fetchData={async (start, end) =>
						balanceHistory.filter((t) => (!start || t.date >= start) && (!end || t.date <= end))}
				/>
			</div>
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Transaction history</h3>
			<div class="m-3">
				<table class="table-fixed w-full text-sm text-left">
					<thead class="text-black text-opacity-50 font-bold">
						<td class="w-24">Height</td>
						<td>Hash</td>
						<td class="w-32">Balance change</td>
					</thead>
					<tbody>
						{#each truncatedTransactions as txn}
							<tr class="txn-row">
								<td
									><a class="text-blue-800 font-medium" href={`/blocks/${txn.height}`}
										>{txn.height}</a
									></td
								>
								<td class="overflow-ellipsis overflow-hidden mono">
									<a class="text-blue-800" href={`/blocks/${txn.height}/${txn.txhash}`}
										>{txn.txhash}</a
									></td
								>
								<td>
									{#each Object.entries(txn.deltas) as [denom, change]}
										{#if change > 0}
											<span class="text-green-800">+{change.toFixed(6)} {denom}</span><br />
										{:else}
											<span class="text-red-800">{change.toFixed(6)} {denom}</span><br />
										{/if}
									{/each}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
				{#if truncatedTransactions.length != summary.transactions.length}
					<button class="text-blue-800" on:click={() => (transactionCount += 50)}>Load more</button>
				{/if}
			</div>
		</div>
	</div>
</template>

<style>
	.txn-row td {
		vertical-align: top;
		padding-bottom: 1rem;
		/* border: 1px solid black; */
	}
</style>
