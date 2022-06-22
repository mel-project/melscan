<script context="module" lang="ts">
	import type { LoadEvent } from '@sveltejs/kit';
	import { melscan } from '@utils/common';

	// import {loader} from '@utils/common'
	// export const {refresh, load} = loader()
	export let data;
	export const load = (endpoint?: string) => async (event: LoadEvent) => {
		let { url } = event;
		console.log('loading');
		return {
			status: 200,
			props: {
				data: await melscan(endpoint || url)
			}
		};
	};
</script>
<script lang="ts">
    // export let network;
    // export let previous;
    // export let height;
    // export let history_hash;
    // export let coins_hash;
    // export let transactions_hash;
    // export let fee_pool;
    // export let fee_multiplier;
    // export let dosc_speed;
    // export let pools_hash;
    // export let stakes_hash;
</script>
<template>
	<div>{data}</div>
</template>
