<script context="module" lang="ts">
	import { backendUrl, melscan, autorefresh } from '@utils/common';
	import type { LoadFunction } from '@utils/common';
	import type { Overview } from '@utils/page-types';
	interface OverviewPage {
		status: number;
		props: Overview;
	}

	export let load: LoadFunction<OverviewPage> = async (loadEvent) => {
		let endpoint = '/raw/overview';
		let props = (await melscan(loadEvent.fetch, endpoint)) as unknown as Overview;
		return {
			status: 200,
			props
		};
	};
</script>

<script lang="ts">
	import HashSearch from '@components/HashSearch.svelte';
	import type { BlockHeight } from '@utils/types';
	import TopNav from '../components/TopNav.svelte';

	export let erg_per_mel, sym_per_mel, recent_blocks;

	$: recentTxx = () => {
		let x = recent_blocks.map((b) => b.transactions).flat();
		if (x.length > 50) {
			x.length = 50;
		}
		return x;
	};
	$: {
		console.debug(recent_blocks);
	}
	
	autorefresh(backendUrl('/raw/overview'))();
</script>

<TopNav><a href="/">Melscan</a></TopNav>
<div class="container mx-auto max-w-screen-lg">
	<div class="grid mt-8">
		<div class="col-span-full">
			<HashSearch />
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 mt-8 mb-8">
		<div class="col-span-2 mb-3">
			<h3 class="text-2xl font-bold">Melmint/Melswap</h3>
		</div>
		<div>
			<span class="text-lg font-bold">
				<span class="text-black text-opacity-50">1 ERG =</span>
				{(1.0 / erg_per_mel).toFixed(5)} MEL
			</span>
			<br />
			<small class="text-blue-600 font-bold"><a href="/pools/ERG/MEL">See details →</a></small>
		</div>
		<div>
			<span class="text-lg font-bold">
				<span class="text-black text-opacity-50">1 SYM =</span>
				{(1.0 / sym_per_mel).toFixed(5)} MEL
			</span>
			<br />
			<small class="text-blue-600 font-bold"><a href="/pools/MEL/SYM">See details →</a></small>
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
					{#each recent_blocks as block (block.header.height)}
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
