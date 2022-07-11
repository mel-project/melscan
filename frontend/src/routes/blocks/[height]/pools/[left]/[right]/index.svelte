<script context="module" lang="ts">
	import BreadCrumbs  from '@components/BreadCrumbs.svelte';
	import { melscan } from '@utils/common';

	export let load = async (event) => {
		let { params } = event;
		console.log(params);
		let { height, left, right } = params;
		let res = (await melscan(fetch, `/raw/blocks/${height}/pools/${left}/${right}`)) as PoolInfo;
		return {
			status: 200,
			props: {...res, params}
		}
	};
</script>

<script lang="ts">
	import type { PoolDataItem, PoolKey, PoolState } from '@utils/types';
	import TopNav from '@components/TopNav.svelte';
	import { BreadCrumb, type PoolInfo } from '@utils/page-types';

	export let pool_state: PoolState;
	export let latest_item: PoolDataItem;
	export let params: any;

	let denom_tooltip = '';
	let last_item = latest_item;


	let { left, right, height } = params;
	let pool_key: PoolKey = { left, right };


	let breadcrumbs = [BreadCrumb("Melscan", "/")]
	

  // temp start 
	let handler = {
		get: function (target) {
			return '';
		}
	};
	let tooltips = new Proxy({}, handler);
  // temp end 

</script>

<template>
	<TopNav><BreadCrumbs {breadcrumbs}></BreadCrumbs></TopNav>
	<div class="container mx-auto max-w-screen-lg">
		<div class="mb-3 mt-8" style="display: flex">
			<h3 class="text-2xl font-bold">Pair {pool_key.left}/{pool_key.right}</h3>
			{denom_tooltip}
		</div>

		<div class="grid grid-cols-12 md:grid-flow-col grid-flow-row">
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Price</small>{tooltips['price']}</div>
				<div class="text-lg font-medium">
					{Math.round(last_item.price * 1000.0) / 1000.0}
					{pool_key.left}/{pool_key.right}
				</div>
			</div>
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Liquidity</small>{tooltips['liquidity']}</div>
				<div class="text-lg font-medium">
					{Math.round(last_item.liquidity * 1000.0) / 1000.0}
					<span class="text-sm">({pool_key.left} × {pool_key.right})<sup>1/2</sup></span>
				</div>
			</div>
			<div class="col-span-12 md:col-span-3 card ticker-card">
				<div><small>Current height</small>{tooltips['height']}</div>
				<div class="text-lg font-medium">{last_item.height}</div>
			</div>

			<div class="md:col-span-9 col-span-12 md:row-span-3 card">
				<div class="grid grid-cols-2" id="head" />
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
