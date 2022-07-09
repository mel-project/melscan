
import { browser } from "$app/env";
import { goto, invalidate } from "$app/navigation";
import { getStores } from "$app/stores";
import type {Load, LoadEvent} from "@sveltejs/kit/types"
import { onDestroy } from "svelte";



export const url_mapping = {
	'/': ['/raw/overview']
}
export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + (url_mapping[endpoint] || endpoint);

export type Fetch = (info: RequestInfo, init?: RequestInit)=> Promise<Response>;

export const melscan = async (fetch: Fetch, url: string): Promise<JSON> => {
	const response = await fetch(url);
	console.log(`requesting ${url}`)
	if (!response.ok) {
		throw `failed to fetch '${url}' data`;
	}
	let res = response.json()
	return res;
};


export type EndpointLoader =  (loadEvent: LoadEvent) => string | [string];
export const loader =  (endpoints?: string | [string] | EndpointLoader) => async (event: LoadEvent) => {
	let {url, fetch, params} = event;

	if(typeof endpoints == "function"){
		endpoints = endpoints(event)
	}

	if(typeof endpoints == "string"){
		endpoints = [endpoints]
	}
	let paths: string[] = endpoints || url_mapping[url.pathname] || [url.pathname]
	let sources = paths.map(path => backendUrl(path))
	let data: JSON[] = (await Promise.all(sources.map(source => melscan(fetch,source)))).flat();
	let props = Object.assign(...data);
	// console.log("Props: ", props);
	const refresh = ()=>Promise.all(sources.map(e => melscan(fetch, e)))
	return {
		status: 200,
		props: {
			refresh,
			autorefresh: (interval?: number)=>{
				console.log("autorefresh called)")
				if(browser){
					console.log(browser)
					interval = interval || 1000;
					let interval_code = setInterval(async () => {
						// let v = await refresh()
						sources.map(i => {
							console.log("invalidating:",i);
							invalidate(i)
						})

					}, interval)
					console.log(interval_code)
					onDestroy(()=>clearInterval(interval_code))
				}
			},
			...props,
			params,
		}
	};
}

export const load = loader()