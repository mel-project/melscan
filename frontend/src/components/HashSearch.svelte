<script>
	import { goto } from '$app/navigation';
	import { melscan } from '@utils/common';
	let value = '';
	let pending = false;
	const onChange = (e) => {
		value = e.target.value;
		error = '';
	};
	let error = '';
	const onKeyUp = async (e) => {
		try {
			if (e.key === 'Enter') {
				pending = true;
				try {
					try {
						let resp = await melscan(fetch, '/raw/search/transaction/' + value);
						goto('/blocks/' + resp + '/' + value);
					} catch {
						let resp = await melscan(fetch, '/raw/search/block/' + value);
						goto('/blocks/' + resp);
					}
				} catch {
					let resp = await melscan(fetch, '/raw/blocks/' + value + '/summary');
					if (!resp) {
						throw 'no such block';
					}
					goto('/blocks/' + value);
				} finally {
					pending = false;
				}
			}
		} catch {
			error = 'no such block, transaction, or address';
		}
	};
</script>

<template>
	<input
		placeholder="Search for a hash or height"
		class="hs"
		on:input={onChange}
		{value}
		on:keyup={onKeyUp}
		disabled={pending}
	/>
	{#if error.length > 0}
		<div class="error">{error}</div>
	{/if}
</template>

<style>
	.hs {
		width: 100%;
		border: 1px black solid;
		font-size: 0.8rem;
		padding: 0.3rem;
	}

	.error {
		font-size: 0.8rem;
		color: red;
		padding-top: 0.2rem;
	}
</style>
