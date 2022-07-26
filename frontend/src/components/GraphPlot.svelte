<script lang="ts">
	import { queryGraph } from '@utils/common';
	import type { GraphDatum } from '@utils/page-types';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import uPlot from 'uplot';

	/// Function for fetching data
	export let fetchData: (start: Date | null, end: Date | null) => Promise<GraphDatum[]>;

	export let initStart: Date | null = null;
	export let initEnd: Date | null = null;
	export let title = '';
	export let unit = '';
	export let label = 'Value';
	export let height = '20rem';
	export let stroke = 'black';
	export let fill = 'rgba(0, 0, 0, 0.1)';
	export let stepped = false;

	let container: HTMLElement;

	const formatData = (d: GraphDatum[]) => [
		d.map((dp) => dp.date.getTime() / 1000.0),
		d.map((dp) => (isNaN(dp.value) ? 0.0 : dp.value))
	];

	function clamp(
		nRange: number,
		nMin: number,
		nMax: number,
		fRange: number,
		fMin: number,
		fMax: number
	) {
		if (nRange > fRange) {
			nMin = fMin;
			nMax = fMax;
		} else if (nMin < fMin) {
			nMin = fMin;
			nMax = fMin + nRange;
		} else if (nMax > fMax) {
			nMax = fMax;
			nMin = fMax - nRange;
		}

		return [nMin, nMax];
	}

	let loading = writable(false);

	onMount(async () => {
		if (typeof window !== 'undefined') {
			const dataPoints = await fetchData(initStart, initEnd);

			let data = formatData(dataPoints);

			function getSize() {
				let { width, height } = container.getBoundingClientRect();
				return {
					width: width,
					height: height - 50
				};
			}
			let size = getSize();
			let opts = {
				title: title,
				id: 'chart1',
				class: 'my-chart',
				width: size.width,
				height: size.height,
				padding: [0, 0, 0, 0],
				series: [
					{},
					{
						// initial toggled state (optional)
						show: true,

						spanGaps: false,

						// in-legend display
						label: label,
						value: (self, rawValue) => rawValue.toFixed(4) + (unit ? ' ' + unit : unit),

						// series style
						stroke: stroke,
						width: 1,
						fill: fill,
						dash: [10, 0],

						paths: stepped
							? uPlot.paths.stepped({
									align: 1,
									alignGaps: 0
							  })
							: null
					}
				],
				cursor: {
					drag: {
						setScale: false,
						x: true,
						y: false
					}
				},
				hooks: {
					init: [
						(u) => {
							u.over.ondblclick = (e: any) => {
								console.log('Fetching data for full range');

								u.setData(data);
							};
						}
					],
					setScale: [
						(x) => {
							console.log('setScale', x);
						}
					],
					setSelect: [
						async (u: any) => {
							let min = u.posToVal(u.select.left, 'x');
							let max = u.posToVal(u.select.left + u.select.width, 'x');
							loading.set(true);
							try {
								console.log(u.select);

								console.log('Fetching data for range...', { min, max });
								let data = await fetchData(new Date(min * 1000.0), new Date(max * 1000.0));
								u.setData(formatData(data), true);
								console.log('Fetched');
								// u.setScale('x', { min, max });
							} finally {
								loading.set(false);
								u.setSelect({ width: 0, height: 0 }, false);
							}
						}
					]
				}
			};

			let u = new uPlot(opts, data, container);

			window.addEventListener('resize', (e) => {
				u.setSize(getSize());
			});
		}
	});
</script>

<div id="container" style="height: {height}" class:loading={$loading} bind:this={container} />

<style>
	.loading {
		opacity: 0.4;
	}
</style>
