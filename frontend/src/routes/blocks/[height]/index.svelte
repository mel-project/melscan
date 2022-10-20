<script context="module" lang="ts">
	import TopNav from './../../../components/TopNav.svelte';
	import BreadCrumbs from '@components/BreadCrumbs.svelte';
	import { backendUrl, melscan } from '@utils/common';
	import type { LoadEvent } from '@sveltejs/kit';
	import type { HashVal, Header, TransactionSummary } from '@utils/types';
	import { tooltips } from '@utils/common';
	import { BreadCrumb, type BlockSummary } from '@utils/page-types';
	import { invalidate } from '$app/navigation';

	export let load = async (event) => {
		let { params, fetch, url } = event;
		let res = (await melscan(fetch, '/raw' + url.pathname + '/summary')) as BlockSummary;
		if (res == null) {
			invalidate(url);
			return;
		}
		return {
			status: 200,
			props: res
		};
	};
</script>

<script lang="ts">
	export let header: Header;
	export let total_weight: number;
	export let reward_amount: number;
	export let transactions: TransactionSummary[];
	export let total_fees: number;
	export let header_hash: HashVal;
	export let fee_multiplier: number;
	$: breadcrumbs = [BreadCrumb('Melscan', '/'), BreadCrumb(`Block ${header.height}`, ``)];
</script>

<template>
	<div class="container mx-auto max-w-screen-lg">
		<TopNav><BreadCrumbs {breadcrumbs} /></TopNav>
		<div class="bottom-nav">
			<span><a href="/blocks/{header.height - 1}">◂ Previous</a></span>
			<span><a href="/blocks/{header.height + 1}" rel="external">Next ▸</a></span>
		</div>
		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Summary</h3>
		</div>
		<table id="block-info" class="table-fixed w-full m-3 text-sm ">
			<tbody>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold">
						<span>Hash</span>
						{tooltips['blockHash']}
					</td>
					<td class="font-medium overflow-ellipsis overflow-hidden mono">{header_hash}</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold">
						<span class="name">Height</span>
						<!-- {tooltips["height"] } -->
					</td>
					<td>{header.height}</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold">Number of transactions</td>
					<td>{transactions.length}</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold"> Total transaction weight </td>
					<td>{total_weight} wu</td>
				</tr>
			</tbody>
		</table>

		<table class="table-fixed w-full m-3 text-sm">
			<tbody>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold">
						<span class="name">Fees charged</span>
						{tooltips['feesCharged']}
					</td>
					<td>{total_fees / 1000000} MEL</td>
				</tr><tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold"> Fee multiplier </td>
					<td>{fee_multiplier} µMEL/wu</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold">
						Fee pool {tooltips['feePool']}
					</td>
					<td>{header.fee_pool / 1000000} MEL</td>
				</tr>
			</tbody>
		</table>
		<table class="table-fixed w-full m-3 text-sm">
			<tbody>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold "> Coins merkle root </td>
					<td class="overflow-ellipsis overflow-hidden mono">{header.coins_hash}</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold"> Pools merkle root </td>
					<td class="overflow-ellipsis overflow-hidden mono">{header.pools_hash}</td>
				</tr>
				<tr>
					<td class="w-1/3 text-black text-opacity-50 font-bold"> Transactions merkle root </td>
					<td class="overflow-ellipsis overflow-hidden mono">{header.transactions_hash}</td>
				</tr>
			</tbody>
		</table>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Transactions</h3>
		</div>

		<table class="table-fixed text-left w-full m-3 text-sm">
			<thead class="text-black text-opacity-50">
				<tr>
					<th class="w-2/3">Hash</th>
					<th>Weight</th>
				</tr>
			</thead>
			<tbody>
				{#each transactions as transaction}
					<tr>
						<td class="overflow-ellipsis overflow-hidden mono"
							><a href="/blocks/{header.height}/{transaction.hash}" class="text-blue-600">
								{transaction.hash}</a
							></td
						>
						<td>{transaction.weight} wu</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</template>

<style>
	.container td {
		overflow: hidden;
		text-overflow: elipses;
	}
	.bottom-nav {
		display: flex;
		flex-direction: row;
		justify-content: space-around;
		width: 100%;
		padding: 1em;
		padding-left: 0;
	}
	a:hover {
		text-decoration: underline;
	}
</style>
