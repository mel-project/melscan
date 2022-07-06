<script context="module" lang='ts'>
	export { load } from '@utils/common';
</script>

<script lang="ts">
	import TopNav from '../components/TopNav.svelte';
	export let refresh: (s?: string)=>Promise<JSON>;
	export let autorefresh: () => Promise<number>

	// autorefresh();

	export let erg_per_mel: number;
	export let sym_per_mel: number;
	export let recent_blocks: [any];


	$: recentTxx = () => {
		let x = recent_blocks.map((b) => b.transactions).flat();
		if (x.length > 50) {
			x.length = 50;
		}
		return x;
	};
</script>

<TopNav>Melscan</TopNav>
<a href="/blocks">blocks</a>
<div class="container mx-auto max-w-screen-lg">
	<div class="grid grid-cols-1 md:grid-cols-2 mt-8 mb-8">
		<div class="col-span-2 mb-3">
			<h3 class="text-2xl font-bold">Melmint/Melswap</h3>
		</div>
		<div>
			<span class="text-lg font-bold">
				<span class="text-black text-opacity-50">1 ERG =</span>
				{(erg_per_mel).toFixed(5)} MEL
			</span>
			<br />
			<small class="text-blue-600 font-bold"><a href="/pools/MEL/ERG">See details →</a></small>
		</div>
		<div>
			<span class="text-lg font-bold">
				<span class="text-black text-opacity-50">1 SYM =</span>
				{(sym_per_mel).toFixed(5)} MEL
			</span>
			<br />
			<small class="text-blue-600 font-bold"><a href="/pools/MEL/ERG">See details →</a></small>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 mt-12 mb-12 grid-flow-row">
		<div class="col-span-full mb-3">
			<h3 class="text-2xl font-bold">Latest activity</h3>
		</div>
		<div id="latest-blocks">
			<h3 class="text-xl font-semibold">Latest blocks</h3>
			<div class="info-section">
				<small class="text-black text-opacity-50 font-bold">Most recently confirmed blocks</small>
				<div class="lds-ellipsis">
					<div />
					<div />
					<div />
					<div />
				</div>
			</div>
			<table class="table-auto w-full mt-3">
				<thead class="text-left text-sm text-black text-opacity-50">
					<tr>
						<th class="block-height">Height</th>
						<th class="block-weight">Total weight</th>
						<th class="block-reward">Proposer reward</th>
					</tr>
				</thead>
				<tbody id="block-rows" class="leading-loose text-sm">
					{#each recent_blocks as block}
						<tr>
							<td class="font-medium"
								><a href="/blocks/{block.header.height}" class="text-blue-600"
									>{block.header.height}</a
								></td
							>
							<td>{block.total_weight} wu</td>
							<td>{(block.reward_amount / 1000000).toFixed(6)} MEL</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
		<div>
			<h3 class="text-xl font-semibold">Latest transactions</h3>
			<div class="info-section">
				<small class="text-black text-opacity-50 font-bold">Transactions in latest blocks</small>
				<div class="lds-ellipsis">
					<div />
					<div />
					<div />
					<div />
				</div>
			</div>
			<table class="table-auto w-full mt-3">
				<thead class="text-left text-sm text-black text-opacity-50">
					<tr>
						<th>Hash</th>
						<th>Height</th>
						<th>Mel moved</th>
					</tr>
				</thead>
				<tbody id="transaction-rows" class="leading-loose text-sm">
					{#each recentTxx() as tx}
						<tr>
							<td class="font-medium" style="overflow: hidden; text-overflow: ellipsis">
								<a href="/blocks/{tx.height}/{tx.hash}" class="text-blue-600">{tx.hash}</a>
							</td>
							<td>{tx.height}</td>
							<td>{(tx.mel_moved / 1_000_000).toFixed(6)} MEL</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>

<style>
	td {
		max-width: 0;
		/* overflow: hidden; */
		text-overflow: ellipsis;
		/* white-space: nowrap; */
	}
</style>
