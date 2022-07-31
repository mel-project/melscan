<script context="module" lang="ts">
	import BreadCrumbs from '@components/BreadCrumbs.svelte';
	import TopNav from '@components/TopNav.svelte';
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
	import { tooltips } from '@utils/common';
	import { BreadCrumb, type TransactionResponse } from '@utils/page-types';
	import CoinSankey from '@components/CoinSankey.svelte';

	export interface TransactionPage {
		status: number;
		props: TransactionResponse;
	}

	export let load: LoadFunction<TransactionPage> = async (loadEvent) => {
		let { height, txhash } = loadEvent.params;
		let url = `/raw/blocks/${height}/${txhash}`;
		let res = (await melscan(loadEvent.fetch, url)) as TransactionResponse;
		return {
			status: 200,
			props: res
		};
	};
</script>

<script lang="ts">
	export let testnet: boolean;
	export let txhash: TxHash;
	export let txhash_abbr: String;
	export let height: number;
	export let transaction: Transaction;
	export let inputs_with_cdh: Vec<[number, CoinID, CoinDataHeight, MicroUnit, string, string]>;

	export let outputs: Vec<[number, CoinData, MicroUnit, string, string]>;
	export let fee: MicroUnit;
	export let base_fee: MicroUnit;
	export let tips: MicroUnit;
	export let net_loss: Obj<Vec<MicroUnit>>;
	export let net_gain: Obj<Vec<MicroUnit>>;
	export let gross_gain: Vec<MicroUnit>;
	export let weight: number;
	export let kind: string;
	export let covenants: Vec<Vec<String>>;

	// console.log(covenants);
	$: breadcrumbs = [
		BreadCrumb('Melscan', '/'),
		BreadCrumb(`Block ${height}`, `.`),
		BreadCrumb(`Transaction ${txhash.substring(0, 10)}..`, '')
	];

	function print_coin(coin: MicroUnit) {
		return `${coin[0]} ${coin[1]}`;
	}
</script>

<template>
	<TopNav>
		<BreadCrumbs {breadcrumbs} />
	</TopNav>
	<div class="container mx-auto max-w-screen-lg">
		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Summary</h3>
		</div>

		<div class="m-3">
			<table class="table-fixed w-full text-sm text-left">
				<tbody>
					<tr>
						<td class="text-black text-opacity-50 font-bold w-1/3">Height</td>
						<td><a href="/blocks/{height}" class="text-blue-800 font-medium">{height}</a></td>
					</tr>
					<tr>
						<td class="text-black text-opacity-50 font-bold">
							<span class="name">Kind</span>
							{tooltips['kind']}
						</td>
						<td class="font-medium">{kind}</td>
					</tr>
					<tr>
						<td class="text-black text-opacity-50 font-bold">
							<span class="name">Hash</span>
						</td>
						<td class="font-medium mono overflow-ellipsis overflow-hidden">{txhash}</td>
					</tr>

					<td class="text-black text-opacity-50 font-bold w-1/3">Total output</td>
					<td>
						{#each gross_gain as gain_entry}
							{gain_entry[0]} {gain_entry[1]} {'  '}
						{/each}
					</td>
					<tr>
						<td class="text-black text-opacity-50 font-bold">
							<span class="name">Fee</span>
							{tooltips['fee']}
						</td>
						<td
							>{print_coin(fee)}<br />
							<span class="text-black text-opacity-50">{print_coin(base_fee)} <i>base</i></span><br
							/>
							<span class="text-black text-opacity-50">{print_coin(tips)} <i>tips</i></span><br />
						</td>
					</tr>
					<tr>
						<td class="text-black text-opacity-50 font-bold">
							<span class="name">Weight</span>
							{tooltips['weight']}
						</td>
						<td>{weight} wu</td>
					</tr>
				</tbody>
			</table>
		</div>
		<div class="grid grid-cols-1 text-sm">
			<div class="m-3">
				<span class="text-black text-opacity-50 font-bold">
					<span class="name">Net Senders</span>
					{tooltips['netSenders']}
				</span><br />
				<table class="table-fixed w-full text-left">
					<tbody>
						{#each Object.entries(net_loss) as entry}
							<tr>
								<td class="overflow-ellipsis overflow-hidden">
									<a class="text-blue-800" href={`/address/${entry[0]}`}>{entry[0]}</a>
								</td>
								<td class="font-medium" style="color: #a22041">
									{entry[1][0][0]} {entry[1][0][1]}</td
								>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<div class="m-3">
				<span class="text-black text-opacity-50 font-bold">
					<span class="name">Net Receivers</span>
					{tooltips['netReceivers']}
				</span><br />
				<table class="table-fixed w-full text-left">
					<tbody>
						{#each Object.entries(net_gain) as entry}
							<tr>
								<td class="overflow-ellipsis overflow-hidden">
									<a class="text-blue-800" href={`/address/${entry[0]}`}>{entry[0]}</a>
								</td>
								<td class="font-medium" style="color:#007b43">
									{entry[1][0][0]} {entry[1][0][1]}</td
								>
							</tr>
						{/each}

						<tr>
							<td><i>(Total fees)</i></td>
							<td class="font-medium" style="color: #007b43">
								{fee[0]}
								{fee[1]}
							</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Covenants</h3>
		</div>

		<div class="m-3">
			<table class="table-fixed w-full text-sm text-left">
				<tbody>
					{#each covenants as [covhash, covenant]}
						<tr>
							<td class="text-black text-opacity-50 font-bold overflow-ellipsis overflow-hidden">
								<span class="name">{covhash}</span>
							</td>
							<td class="font-medium">
								{#each covenant as opcode}
									{opcode} <br />
								{/each}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Flow summary</h3>
			<p>(MEL only)</p>
		</div>

		<div class="mb-3 mt-8">
			{#key txhash}
				<CoinSankey {height} {txhash} {transaction} {fetch} />
			{/key}
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Inputs</h3>
		</div>

		<div class="m-3">
			{#each inputs_with_cdh as [index, input, cdh, value, additional_data, recipient]}
				<table class="table-fixed w-full text-sm text-left mt-1 mb-1">
					<tbody>
						<tr>
							<td class="text-black text-opacity-50 font-bold w-1/3">Index</td>
							<td>{index}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold ">Spent CoinID</td>
							<td class="overflow-ellipsis overflow-hidden"
								><a class="text-blue-800 mono " href="/blocks/{cdh.height}/{input.txhash}"
									>{input.txhash}</a
								>-{input.index}</td
							>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold">Value</td>
							<td>{value[0]} {value[1]}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold ">Recipient</td>
							<td class="overflow-ellipsis overflow-hidden">{recipient}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold">Additional data</td>
							<td>{additional_data || '""'}</td>
						</tr>
					</tbody>
				</table>
			{/each}
		</div>

		<div class="mb-3 mt-8">
			<h3 class="text-2xl font-bold">Outputs</h3>
		</div>

		<div class="m-3">
			{#each outputs as [index, coin_data, value, additional_data, recipient]}
				<table class="table-fixed w-full text-sm text-left mt-1 mb-1">
					<tbody>
						<tr>
							<td class="text-black text-opacity-50 font-bold w-1/3">Index</td>
							<td>{index}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold">Value</td>
							<td>{value[0]} {value[1]}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold">Recipient</td>
							<td class="overflow-ellipsis overflow-hidden">{recipient}</td>
						</tr>
						<tr>
							<td class="text-black text-opacity-50 font-bold">Additional data</td>
							<td>{additional_data || '""'}</td>
						</tr>
					</tbody>
				</table>
			{/each}
		</div>
	</div>
</template>

<style>
	td {
		vertical-align: top;
	}
</style>
