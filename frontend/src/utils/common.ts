
import { browser } from "$app/env";
import { goto, invalidate } from "$app/navigation";
import { getStores } from "$app/stores";
import type {Load, LoadEvent} from "@sveltejs/kit/types"
import { onDestroy } from "svelte";




export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

export const url_mapping = {
	'/': [backendUrl('/raw/overview')]
}

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


export type EndpointLoader =  (loadEvent: LoadEvent) => [string];
export const loader =  (endpoint_loader: EndpointLoader) => async (event: LoadEvent) => {
	let {url, fetch, params} = event;
	let sources = endpoint_loader(event)
	// console.log("Props: ", props);
	const refresh = ()=>Promise.all(sources.map(e => melscan(fetch, e)))
	let data: JSON[] = await refresh();
	console.log(data);
	let props = Object.assign(...data);
	return {
		status: 200,
		props: {
			refresh,
			autorefresh: (interval?: number)=>{
				if(browser){
					console.log(browser)
					interval = interval || 1000;
					let interval_code = setInterval(async () => {
						// let v = await refresh()
						sources.map(i => {
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


export const load = loader(({url})=> url_mapping[url.pathname] || [url.pathname])

// temp start 
let handler = {
	get: function (target) {
		return '';
	}
};
export const tooltips = new Proxy({}, handler);
// temp end 
