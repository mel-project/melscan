<!--
	@component
	Generates an SVG Sankey chart using [d3-sankey](https://github.com/d3/d3-sankey).
 -->
<script>
	import { getContext } from 'svelte';
	import * as Sankey from 'd3-sankey';

	const { data, width, height } = getContext('LayerCake');

	/** @type {Function} [colorLinks=d => 'rgba(0, 0, 0, .2)'] – A function to return a color for the links. */
	export let colorLinks = (d) => 'rgba(0, 0, 0, .2)';

	/** @type {Function} [colorNodes=d => '#333'] – A function to return a color for each node. */
	export let colorNodes = (d) => '#333';

	/** @type {Function} [colorText=d => '#263238'] – A function to return a color for each text label. */
	export let colorText = (d) => '#263238';

	/** @type {Number} [nodeWidth=5] – The width of each node, in pixels, passed to [`sankey.nodeWidth`](https://github.com/d3/d3-sankey#sankey_nodeWidth). */
	export let nodeWidth = 20;

	/** @type {Number} [nodePadding=10] – The padding between nodes, passed to [`sankey.nodePadding`](https://github.com/d3/d3-sankey#sankey_nodePadding). */
	export let nodePadding = 10;

	/** @type {Function} [linkSort=null] – How to sort the links, passed to [`sankey.linkSort`](https://github.com/d3/d3-sankey#sankey_linkSort). */
	export let linkSort = null;

	/** @type {Function} [nodeId=d => d.id] – The ID field accessor, passed to [`sankey.nodeId`](https://github.com/d3/d3-sankey#sankey_nodeId). */
	export let nodeId = (d) => d.id;

	/** @type {Function} [nodeAlign=d3.sankeyLeft] – An alignment function to position the Sankey blocks. See the [d3-sankey documentation](https://github.com/d3/d3-sankey#alignments) for more. */
	export let nodeAlign = Sankey.sankeyLeft;

	$: sankey = Sankey.sankey()
		.nodeAlign(nodeAlign)
		.nodeWidth(nodeWidth)
		.nodePadding(nodePadding)
		.nodeId(nodeId)
		.size([$width, $height])
		.linkSort(linkSort);

	$: sankeyData = sankey($data);

	$: link = Sankey.sankeyLinkHorizontal();

	$: fontSize = 13;

	// https://stackoverflow.com/questions/7616461/generate-a-hash-from-string-in-javascript
	const cyrb53 = function (str, seed = 0) {
		let h1 = 0xdeadbeef ^ seed,
			h2 = 0x41c6ce57 ^ seed;
		for (let i = 0, ch; i < str.length; i++) {
			ch = str.charCodeAt(i);
			h1 = Math.imul(h1 ^ ch, 2654435761);
			h2 = Math.imul(h2 ^ ch, 1597334677);
		}
		h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
		h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);
		return 4294967296 * (2097151 & h2) + (h1 >>> 0);
	};
</script>

<g class="sankey-layer">
	<g class="link-group">
		{#each sankeyData.links as d}
			<path
				d={link(d)}
				fill="none"
				stroke={colorLinks(d)}
				stroke-opacity="0.5"
				stroke-width={d.width}
			/>
		{/each}
	</g>
	<g class="rect-group">
		{#each sankeyData.nodes as d, i}
			<rect x={d.x0} y={d.y0} height={d.y1 - d.y0} width={20} fill={colorNodes(d)} />
			<text
				x={d.x0 < $width / 4 ? d.x1 + 6 : d.x0 - 6}
				y={(d.y1 + d.y0) / 2}
				dy={fontSize / 2 - 2 + (cyrb53(d.id) % 80) - 40}
				style="fill: {colorText(d)};
							font-size: {fontSize}px;
							text-anchor: {d.x0 < $width / 4 ? 'start' : 'end'};"
			>
				{'label' in d ? d.label : d.id}
			</text>
		{/each}
	</g>
</g>

<style>
	text {
		pointer-events: none;
	}
</style>
