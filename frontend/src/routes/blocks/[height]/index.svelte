<script context="module" lang="ts">
	import TopNav from './../../../components/TopNav.svelte';
	export { load } from '@utils/common';
</script>
<script lang="ts">
import type { BlockSummary, Header } from '@utils/types';
	export let summary: BlockSummary;
	let header:Header = summary.header;
	let tooltips = {};
</script>
<template>
	<TopNav>Melscan</TopNav>
	<div class="mb-3 mt-8">
		<h3 class="text-2xl font-bold">Summary</h3>
	</div>

	<table id="block-info" class="table-fixed w-full m-3 text-sm ">
	<tbody>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			<span class="text-ellipsis overflow-hidden">Hash</span>
			{ tooltips["blockHash"] }
		</td>
		<td class="font-medium">{header.hash() }</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			<span class="name">Height</span>
			<!-- {tooltips["height"] } -->
		</td>
		<td>{header.height }</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">Number of transactions</td>
		<td>{txcount }</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Total transaction weight
		</td>
		<td>{txweight } wu</td>
		</tr>
	</tbody>
	</table>
	<table class="table-fixed w-full m-3 text-sm">
	<tbody>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			<span class="name">Fees charged</span>
			{tooltips["feesCharged"] }
		</td>
		<td>{total_fees }</td>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Fee multiplier
		</td>
		<td>{fee_multiplier } µMEL/wu</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Fee pool {tooltips["feePool"]}
		</td>
		<td>{fee_pool }</td>
		</tr>
	</tbody>
	</table>
	<table class="table-fixed w-full m-3 text-sm">
	<tbody>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Coins merkle root
		</td>
		<td>{header.coins_hash }</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Pools merkle root
		</td>
		<td>{header.pools_hash }</td>
		</tr>
		<tr>
		<td class="w-1/3 text-black text-opacity-50 font-bold">
			Transactions merkle root
		</td>
		<td>{header.transactions_hash }</td>
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
	</tbody>
	</table>

	
	
	
</template>
