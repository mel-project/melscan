<script context="module" lang="ts">
	import TopNav from '@components/TopNav.svelte';
	import { backendUrl, melscan, type LoadFunction } from '@utils/common';
	import type { LoadEvent } from '@sveltejs/kit';
	import type { BlockSummary, bool, BTreeMap, CoinData, CoinDataHeight, CoinID, HashVal, Header, MicroUnit, Obj, Transaction, TransactionSummary, TxHash, u64, Vec } from '@utils/types';
	import { tooltips } from '@utils/common';
	
  export interface TransactionResponse {
      testnet: bool,
      txhash: TxHash,
      txhash_abbr: String,
      height: u64,
      transaction: Transaction,
      inputs_with_cdh: Vec<[CoinID, CoinDataHeight, MicroUnit]>,
      outputs: Vec<[CoinData, MicroUnit]>,
      fee: MicroUnit,
      base_fee: MicroUnit,
      tips: MicroUnit,
      net_loss: BTreeMap<string, Vec<MicroUnit>>,
      net_gain: BTreeMap<string, Vec<MicroUnit>>,
      gross_gain: Vec<MicroUnit>,
  }
  export interface TransactionPage {
    status: number,
    props: TransactionResponse
}

	export let load: LoadFunction<TransactionPage> = async (loadEvent)=>{
		let res = await melscan(loadEvent.fetch, "/raw/overview") as unknown as TransactionResponse
    
		return {
			status: 200,
			props: res
		}
	};
</script>
    

<script lang="ts">
    export let testnet: boolean;
    export let txhash: TxHash;
    export let txhash_abbr: String;
    export let height: number;
    export let transaction: Transaction;
    export let inputs_with_cdh: Vec<[CoinID, CoinDataHeight, MicroUnit]>;
    export let outputs: Vec<[CoinData, MicroUnit]>;
    export let fee: MicroUnit;
    export let base_fee: MicroUnit;
    export let tips: MicroUnit;
    export let net_loss: Obj<Vec<MicroUnit>>
    export let net_gain: Obj<Vec<MicroUnit>>;
    export let gross_gain: Vec<MicroUnit>;

    $: transaction_weight = "Todo"
</script>

<template>


<div class="container mx-auto max-w-screen-lg">
  <div class="mb-3 mt-8">
    <h3 class="text-2xl font-bold">Summary</h3>
  </div>

  <div class="m-3">
    <table class="table-fixed w-full text-sm text-left">
      <tbody>
        <tr>
          <td class="text-black text-opacity-50 font-bold w-1/3">Height</td>
          <td><a href="/blocks/{height}" class="text-blue-600 font-medium">{height}</a></td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">
            <span class="name">Kind</span>
            { tooltips["kind"] }
          </td>
          <td class="font-medium">{transaction.kind }</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">
            <span class="name">Hash</span>
          </td>
          <td class="font-medium">{ txhash }</td>
        </tr>
    </table>
  </div>

  <div class="m-3">
    <table class="table-fixed w-full text-sm text-left">
      <td class="text-black text-opacity-50 font-bold w-1/3">Total output</td>
      <td> {#each gross_gain as gain_entry}
        { gain_entry }
        {/each}
      </td>
      <tr>
        <td class="text-black text-opacity-50 font-bold">
          <span class="name">Fee</span>
          { tooltips["fee"] }
        </td>
        <td>{ fee }<br>
          <span class="text-black text-opacity-50">{ base_fee } <i>base</i></span><br>
          <span class="text-black text-opacity-50">{ tips } <i>tips</i></span><br>
        </td>
      </tr>
      <tr>
        <td class="text-black text-opacity-50 font-bold">
          <span class="name">Weight</span>
          { tooltips["weight"] }
        </td>
        <td>{ transaction_weight } wu</td>
      </tr>
    </table>
  </div>

  <div class="grid grid-cols-1 text-sm">
    <div class="m-3">
      <span class="text-black text-opacity-50 font-bold">
        <span class="name">Net senders</span>
        { tooltips["netSenders"] }
      </span><br>
      <table class="table-fixed w-full text-left">
        <tbody>
          {#each Object.entries(net_loss) as loss_entry}
          <tr>
            <td class="text-ellipsis overflow-hidden">{ loss_entry[0] }</td>
            <td class="font-medium" style="color: #a22041">
            
            </td>
          </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <div class="m-3">
      <span class="name">Net recievers</span>
      { tooltips["netRecievers"] }
      <table class="table-fixed w-full text-left">
        <tbody>
          {#each Object.entries(net_gain) as gain_entry}
          <tr>
            <td class="font-medium" style="color: #007b43">
             
            </td>
          </tr>
          {/each}

          <tr>
            <td><i>(Total fees)</i></td>
            <td class="font-medium" style="color: #007b43">
              { fee }
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <div class="mb-3 mt-8">
    <h3 class="text-2xl font-bold">Inputs</h3>
  </div>

  <div class="m-3">
    {#each inputs_with_cdh as [input, cdh, value],index}
    <table class="table-fixed w-full text-sm text-left mt-1 mb-1">
      <tbody>
        <tr>
          <td class="text-black text-opacity-50 font-bold w-1/3">Index</td>
          <td>{index}</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Spent CoinID</td>
          <td><a class="text-blue-600" href="/blocks/{cdh.height}/{input.txhash}">{ input.txhash }</a>-{input.index}</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Value</td>
          <td>{value}</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Recipient</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Additional data</td>
          <td>{cdh.coin_data}.additional_data_hex()</td>
        </tr>
      </tbody>
    </table>
    {/each}
  </div>

  <div class="mb-3 mt-8">
    <h3 class="text-2xl font-bold">Outputs</h3>
  </div>

  <div class="m-3">
    {#each  outputs as [coin_data, value], index}
    <table class="table-fixed w-full text-sm text-left mt-1 mb-1">
      <tbody>
        <tr>
          <td class="text-black text-opacity-50 font-bold w-1/3">Index</td>
          <td>{index}</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Value</td>
          <td>{value}</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Recipient</td>
        </tr>
        <tr>
          <td class="text-black text-opacity-50 font-bold">Additional data</td>
          <td>{coin_data}.additional_data_hex()</td>
        </tr>
      </tbody>
    </table>
    {/each}
  </div>
</div>
</template>
