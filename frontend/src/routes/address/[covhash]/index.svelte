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
						<td class="text-black text-opacity-50 font-bold w-1/3">First seen</td>
						<td>
							<a class="text-blue-600 font-medium" href={`/blocks/${firstHeight}`}>{firstHeight}</a>
							({firstDate})
						</td>
					</tr>
					<tr>
						<td class="text-black text-opacity-50 font-bold w-1/3">Total balance</td>
						<td>
							{#each Object.entries(summary.balances) as [denom, balance]}
								<b class="font-medium">{balance.toFixed(5)}</b>&nbsp;<i>{denom}</i>&nbsp;&nbsp;
							{/each}
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</template>
