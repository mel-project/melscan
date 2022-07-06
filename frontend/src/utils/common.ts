
import { browser } from "$app/env";
import { invalidate } from "$app/navigation";
import { getStores } from "$app/stores";
import type {Load, LoadEvent} from "@sveltejs/kit/types"

export const backendUrl = (endpoint) => 'http://127.0.0.1:13000/raw' + endpoint;

export type Fetch = (info: RequestInfo, init?: RequestInit)=> Promise<Response>;

export const url_mapping = {
	'/': ['/overview']
}

export const melscan = async (fetch: Fetch, endpoint: string): Promise<JSON> => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	// console.log(`requesting ${endpoint}`)
	if (!response.ok) {
		throw `failed to fetch '${url}' data`;
	}
	let res = response.json()
	return res;
};


export const loader =  (endpoints?: string | [string]) => async (event: LoadEvent) => {
	let {url, fetch, params} = event;
	if(typeof endpoints == "string"){
		endpoints = [endpoints]
	}
	let sources: string[] = endpoints || url_mapping[url.pathname] || [url.pathname]
	let data: JSON[] = (await Promise.all(sources.map(e => melscan(fetch,e)))).flat();
	let props = Object.assign(...data);


	const refresh = ()=>Promise.all(sources.map(e => melscan(fetch, e)))
	return {
		status: 200,
		props: {
			refresh,
			autorefresh: ()=>{	
				setInterval(async () => {
					let v = await refresh()
					.catch((e)=>console.error(e))
					.then(async data=> {
						if(browser)
							invalidate(url.href);
					})
				}, 1000)
			},
			...props
		}
	};
}

export const load = loader()