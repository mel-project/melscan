<script context="module" lang="ts">
	import type { LoadFunction } from '@utils/common';

	export let load: LoadFunction<any> = async (loadEvent) => {
		let { height, txhash } = loadEvent.params;
		return {
			status: 200,
			props: { height, txhash }
		};
	};
</script>

<script lang="ts">
	import CoinDag from '@components/CoinDag.svelte';

	export let height: number;
	export let txhash: string;
</script>

<div class="wrap">
	<CoinDag {height} {txhash} />
</div>

<style>
	.wrap {
		height: 100vh;
		width: 100vh;
	}
</style>
