<script>
	import { isTestnet } from '@utils/common';

	export let testnet = isTestnet;
</script>

<nav class="mx-auto max-w-screen-lg mt-8 text-xl leading-tight outer">
	{#if testnet}
		<div class="font-bold text-black italic text-opacity-50 tnet">Testnet</div>
	{/if}
	<div class="inner">
		<slot />
		<div class="text-sm text-blue-600 hover:underline">
			<b>
				{#if testnet}
					<a href="https://scan.themelio.org">Switch to mainnet</a>
				{:else}
					<a href="https://scan-testnet.themelio.org">Switch to testnet</a>
				{/if}
			</b>
		</div>
	</div>
	<div class="second">
		<a href="/stats">[stats]</a>&nbsp;<a href="https://github.com/themeliolabs/melscan">[github]</a>
	</div>
</nav>

<style lang="scss">
	nav {
		text-overflow: ellipsis;
		overflow-x: hidden;
	}

	.outer {
		display: flex;
		flex-direction: row;
		width: 100%;
		justify-content: space-between;
	}

	.inner {
		flex-grow: 1;
		margin-left: 0.5rem;
	}

	.second {
		font-size: 1rem;
	}
</style>
