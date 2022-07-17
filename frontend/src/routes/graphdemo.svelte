<script lang="ts">
	import GraphPlot from '@components/GraphPlot.svelte';
	import { queryGraph } from '@utils/common';
</script>

<template>
	<GraphPlot
		fetchData={async (start, end) =>
			await queryGraph({
				id: {
					type: 'pool_price',
					from: 'MEL',
					to: 'ERG'
				},
				start: start,
				end: end
			})}
		unit="ERG"
		label="1 MEL"
		title="MELERG rate"
	/>

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
		label="MEL supply"
		stroke="rgba(0, 0, 40, 1)"
		fill="rgba(0, 0, 255, 0.1)"
		title="MEL supply"
	/>

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
		label="SYM supply"
		title="SYM supply"
		stroke="rgba(40, 0, 0, 1)"
		fill="rgba(255, 0, 0, 0.1)"
	/>

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
		title="ERG supply"
		label="ERG supply"
		stroke="rgba(0, 40, 0, 1)"
		fill="rgba(0, 255, 0, 0.1)"
	/>
</template>
