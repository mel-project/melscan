<script context="module" lang="ts">
	import BreadCrumbs from '@components/BreadCrumbs.svelte';
	import { backendUrl, melscan } from '@utils/common';
	import type { LoadEvent } from '@sveltejs/kit';
	import type { HashVal, Header, TransactionSummary } from '@utils/types';
	import { tooltips } from '@utils/common';
	import { BreadCrumb, type BlockSummary } from '@utils/page-types';
	import { invalidate } from '$app/navigation';

	export let load = async (event) => {
		let { params, fetch, url } = event;
		let res = (await melscan(fetch, '/raw' + url.pathname))
	
		return {
			status: 200,
			props: res
		};
	};
</script>