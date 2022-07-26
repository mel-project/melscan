<script>
	export let testnet = false;
</script>

<nav class="mx-auto max-w-screen-lg mt-8 text-xl leading-tight">
	{#if testnet}
		<span class="font-bold text-black italic text-opacity-50">Testnet </span>
	{/if}
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
</nav>

<style lang="scss">
	nav :global(a) {
		text-overflow: ellipsis;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
