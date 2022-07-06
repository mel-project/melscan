<script context="module">
    import {loader} from "@utils/common";
    export let load = loader("")
</script>

<script lang="ts">
    import type { PoolDataItem, PoolKey } from "@utils/types";
    let testnet: Boolean;
    let friendly_denom: String;
    let pool_key: PoolKey;
    let last_item: PoolDataItem;
    let tooltips = {};
</script>

<svelte:head>
    <script src="https://cdn.jsdelivr.net/npm/chart.js@3.5.1/dist/chart.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/hammerjs@2.0.8"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js/dist/chart.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/luxon/2.0.2/luxon.min.js" integrity="sha512-frUCURIeB0OKMPgmDEwT3rC4NH2a4gn06N3Iw6T1z0WfrQZd7gNfJFbHrNsZP38PVXOp6nUiFtBqVvmCj+ARhw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/chartjs-plugin-zoom/1.1.1/chartjs-plugin-zoom.min.js" integrity="sha512-NxlWEbNbTV6acWnTsWRLIiwzOw0IwHQOYUCKBiu/NqZ+5jSy7gjMbpYI+/4KvaNuZ1qolbw+Vnd76pbIUYEG8g==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-adapter-luxon@^1"></script>
    <script src="https://cdn.jsdelivr.net/npm/lodash.debounce@4.0.8/index.min.js"></script>
</svelte:head>
<template>
      <style>
        #head button {
          font-size: 90%;
          margin-left: 4px;
          margin-right: 4px;
          border: #ccc;
          border-width: 1px;
          border-style: solid;
          padding-left: 4px;
          padding-right: 4px;
          border-radius: 10%;
        }
      
        #head button:hover {
          background-color: #eee;
        }
      
        .card {
          border: #ccc;
          border-width: 1px;
          border-style: solid;
          padding-left: 4px;
          padding-right: 4px;
          border-radius: 8px;
          margin: 4px;
          padding: 16px;
        }
      
        .ticker-card {
          display: flex;
      
          justify-content: center;
          flex-direction: column;
        }
      
        .tooltip .tooltiptext::after {
          border-color: transparent transparent transparent transparent;
        }
      
      </style>
      <a href="/" class="text-black hover:text-opacity-60 hover:underline">Melscan</a>

      <div class="container mx-auto max-w-screen-lg">
        <div class="mb-3 mt-8" style="display: flex">
          <h3 class="text-2xl font-bold">Pair {pool_key.left}/{pool_key.right}</h3>{denom_tooltip}
        </div>
      
        <div class="grid grid-cols-12 md:grid-flow-col grid-flow-row">
          <div class="col-span-12 md:col-span-3 card ticker-card">
            <div><small>Price</small>{tooltips["price"]}</div>
            <div class="text-lg font-medium">{(last_item.price * 1000.0).round() / 1000.0} {pool_key.left}/{pool_key.right}</div>
          </div>
          <div class="col-span-12 md:col-span-3 card ticker-card">
            <div><small>Liquidity</small>{tooltips["liquidity"]}</div>
            <div class="text-lg font-medium">{(last_item.liquidity * 1000.0).round() / 1000.0} <span class="text-sm">({pool_key.left} × {pool_key.right})<sup>1/2</sup></span>
            </div>
          </div>
          <div class="col-span-12 md:col-span-3 card ticker-card">
            <div><small>Current height</small>{tooltips["height"]}</div>
            <div class="text-lg font-medium">{last_item.height}</div>
          </div>
      
          <div class="md:col-span-9 col-span-12 md:row-span-3 card">
            <div class="grid grid-cols-2" id="head">
              <div class="text-left"><button onclick="handlers.loadLiquidity()">Liquidity</button><button onclick="handlers.loadPrice()">Price</button> {tooltips["graph"]}</div>
              <div class="text-right"><button onclick="handlers.loadLastDay()">1D</button><button onclick="handlers.loadLastWeek()">1W</button><button onclick="handlers.loadLastMonth()">1M</button><button onclick="handlers.loadAllTime()">All</button></div>
            </div>
            <div class="chart-container" style="width: 100%; height: 300px">
              <canvas id="chart"></canvas>
            </div>
          </div>
      
        </div>
      
      
      </div>
     
</template>