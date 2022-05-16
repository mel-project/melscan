<script context="module">
	export async function load({ params, fetch, session, stuff }) {
		const url = `http://127.0.0.1:13000/raw/latest`;
		const response = await fetch(url);

		return {
			status: response.status,
			props: {
	 				ss: response.ok && (await response.json())
			}
			};
	}
</script> 
 
<script> 
	export let ss;
</script> 

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<tt>{JSON.stringify(ss)}</tt>
  