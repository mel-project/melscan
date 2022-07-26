<script context="module" lang="ts">
	import BreadCrumbs from '@components/BreadCrumbs.svelte';
	import { melscan, queryGraph } from '@utils/common';

	export let load = async (event) => {
		let { params } = event;
		console.log(params);
		let { left, right } = params;
		return {
			status: 200,
			props: { params }
		};
	};
</script>

<script lang="ts">
	import type { PoolDataItem, PoolKey, PoolState } from '@utils/types';
	import TopNav from '@components/TopNav.svelte';
	import { BreadCrumb, type GraphDatum, type PoolInfo } from '@utils/page-types';
	import { onMount } from 'svelte';
	import GraphPlot from '@components/GraphPlot.svelte';

	export let params: any;

	let { left, right } = params;
	let pool_key: PoolKey = { left, right };

	let breadcrumbs = [BreadCrumb('Melscan', '/')];

	// temp start
	let handler = {
		get: function (target) {
			return '';
		}
	};
	let tooltips = new Proxy({}, handler);
	// temp end
	let price_data: GraphDatum[] = [];
	let liquidity_data: GraphDatum[] = [];
	const getPriceData = async (start, end) =>
		await queryGraph({
			id: {
				type: 'pool_price',
				from: right,
				to: left
			},
			start,
			end
		});
	const getLiquidityData = async (start, end) =>
		await queryGraph({
			id: {
				type: 'pool_liquidity',
				from: right,
				to: left
			},
			start,
			end
		});
	onMount(async () => {
		price_data = await getPriceData(null, null);
		liquidity_data = await getLiquidityData(null, null);
	});
	$: last_price = price_data.length > 0 ? price_data[price_data.length - 1].value : 0.0;
	$: last_liquidity =
		liquidity_data.length > 0 ? liquidity_data[liquidity_data.length - 1].value : 0.0;
	$: last_height = price_data.length > 0 ? price_data[price_data.length - 1].height : 0.0;
</script>

<template>
	<TopNav><BreadCrumbs {breadcrumbs} /></TopNav>
	<div class="container mx-auto max-w-screen-lg">
		<div class="mb-3 mt-8" style="display: flex">
			<h3 class="text-2xl font-bold">Pair {pool_key.left}/{pool_key.right}</h3>
		</div>

		<div class="grid grid-cols-12 md:grid-flow-col grid-flow-row">
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Price</small>{tooltips['price']}</div>
				<div class="text-lg font-medium">
					{last_price.toFixed(4)}
					{pool_key.left}/{pool_key.right}
				</div>
			</div>
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Liquidity</small>{tooltips['liquidity']}</div>
				<div class="text-lg font-medium">
					{last_liquidity.toFixed(4)}
					<span class="text-sm">({pool_key.left} × {pool_key.right})<sup>1/2</sup></span>
				</div>
			</div>
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Current height</small>{tooltips['height']}</div>
				<div class="text-lg font-medium">{last_height}</div>
			</div>

			<div class="md:col-span-9 col-span-12 md:row-span-3 card">
				<div class="grid grid-cols-2" id="head" />
				<GraphPlot
					fetchData={async (start, end) => await getPriceData(start, end)}
					unit={left}
					label={right}
				/>
			</div>
		</div>
	</div>
</template>

<style>
	#head button {
		font-size: 90%;
		margin-left: 4px;
		margin-right: 4px;
		border: #ccc;
		border-width: 1px;
		border-style: solid;
		padding-left: 4px;
		padding-right: 4px;
		border-radius: 10%;
	}

	#head button:hover {
		background-color: #eee;
	}

	.card {
		border: #ccc;
		border-width: 1px;
		border-style: solid;
		padding-left: 4px;
		padding-right: 4px;
		border-radius: 8px;
		margin: 4px;
		padding: 16px;
	}

	.ticker-card {
		display: flex;

		justify-content: center;
		flex-direction: column;
	}

	.tooltip .tooltiptext::after {
		border-color: transparent transparent transparent transparent;
	}
</style>
