<script context="module" lang="ts">
	import { melscan, type LoadFunction } from '@utils/common';
	import type {
		CoinData,
		CoinDataHeight,
		CoinID,
		MicroUnit,
		Obj,
		Transaction,
		TxHash,
		Vec
	} from '@utils/types';


    
	export let load: LoadFunction<TransactionPage> = async (loadEvent) => {
		let { height, txhash } = loadEvent.params;
		let url = `/raw/blocks/${height}/${txhash}`;
		let res = (await melscan(loadEvent.fetch, url)) as TransactionResponse;

		console.log(res);
		return {
			status: 200,
			props: res
		};
	};
</script>
<script lang="ts">
	import {LayerCake, Svg} from 'layercake';
</script>


